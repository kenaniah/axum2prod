use std::{future::Future, net::TcpListener, pin::Pin, time::Duration};

use axum2prod::configuration;
use sqlx::PgPool;

struct TestContext {
    pub address: String,
    pub db: PgPool,
}

/// Wrapper for tests to ensure each is run in an isolated environment
async fn run_test<T>(test: T)
where
    T: FnOnce(TestContext) -> Pin<Box<dyn Future<Output = ()> + Send>> + std::panic::UnwindSafe,
{
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind random port");
    let port = listener.local_addr().unwrap().port();
    let address = format!("http://127.0.0.1:{}", port);

    // Spin up a test database
    let test_db = configuration::get_config().test_db().await;

    // Spin up the server in the background
    let server = axum2prod::run(listener, test_db.pool.clone()).unwrap();
    let handle = tokio::spawn(server);

    // Run the test
    let task = tokio::spawn(test(TestContext {
        address,
        db: test_db.pool.clone(),
    }));
    let result = task.await;

    // Stop the server
    handle.abort();

    // Drop the test database
    drop(test_db);
    tokio::time::sleep(Duration::from_secs(1)).await;

    // Propagate any panics
    if let Err(err) = result {
        if err.is_panic() {
            std::panic::resume_unwind(err.into_panic());
        }
    }
}

#[tokio::test]
async fn health_check_works() {
    run_test(|ctx| {
        Box::pin(async move {
            let client = reqwest::Client::new();
            let response = client
                .get(&format!("{}/health_check", &ctx.address))
                .send()
                .await
                .expect("Failed to execute request.");
            assert!(response.status().is_success());
            assert_eq!(Some(0), response.content_length());
        })
    })
    .await;
}

#[tokio::test]
async fn subscribe_returns_a_201_for_valid_form_data() {
    run_test(|ctx| {
        Box::pin(async move {
            // Arrange
            let client = reqwest::Client::new();

            // Act
            let body = "name=le%20guin&email=ursula_le_guin%40gmail.com";
            let response = client
                .post(&format!("{}/subscriptions", &ctx.address))
                .header("Content-Type", "application/x-www-form-urlencoded")
                .body(body)
                .send()
                .await
                .expect("Failed to execute request.");

            // Assert
            assert_eq!(201, response.status().as_u16());

            let saved = sqlx::query!("SELECT email, name FROM subscriptions",)
                .fetch_one(&ctx.db)
                .await
                .expect("Failed to fetch saved subscription.");
            assert_eq!(saved.email, "ursula_le_guin@gmail.com");
            assert_eq!(saved.name, "le guin");
        })
    })
    .await;
}

#[tokio::test]
async fn subscribe_returns_a_422_when_data_is_missing() {
    run_test(|ctx| {
        Box::pin(async move {
            // Arrange
            let client = reqwest::Client::new();
            let test_cases = vec![
                ("name=le%20guin", "missing the email"),
                ("email=ursula_le_guin%40gmail.com", "missing the name"),
                ("", "missing both name and email"),
            ];

            for (invalid_body, error_message) in test_cases {
                // Act
                let response = client
                    .post(&format!("{}/subscriptions", &ctx.address))
                    .header("Content-Type", "application/x-www-form-urlencoded")
                    .body(invalid_body)
                    .send()
                    .await
                    .expect("Failed to execute request.");

                // Assert
                assert_eq!(
                    422,
                    response.status().as_u16(),
                    // Additional customized error message on test failure
                    "The API did not fail with 400 Bad Request when the payload was {}.",
                    error_message
                );
            }
        })
    })
    .await;
}
