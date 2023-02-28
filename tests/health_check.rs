use std::net::TcpListener;

use axum2prod::configuration;
use sqlx::{Connection, PgConnection};

// Launch our application in the background
async fn spawn_app() -> hyper::Result<String> {
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind random port");
    let port = listener.local_addr().unwrap().port();
    let db = configuration::get_config().db().await;
    let server = axum2prod::run(listener, db)?;
    let _ = tokio::spawn(server);
    // We return the application address to the caller
    Ok(format!("http://127.0.0.1:{}", port))
}

#[tokio::test]
async fn health_check_works() {
    let address = spawn_app().await.expect("Failed to spawn application");

    let client = reqwest::Client::new();
    let response = client
        .get(&format!("{}/health_check", &address))
        .send()
        .await
        .expect("Failed to execute request.");
    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}

#[tokio::test]
async fn subscribe_returns_a_201_for_valid_form_data() {
    // Arrange
    let app_address = spawn_app().await.expect("Failed to spawn application");
    let client = reqwest::Client::new();
    let config = configuration::get_config();
    let mut connection = PgConnection::connect(&config.database_url)
        .await
        .expect("Failed to connect to Postgres.");

    // Act
    let body = "name=le%20guin&email=ursula_le_guin%40gmail.com";
    let response = client
        .post(&format!("{}/subscriptions", &app_address))
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(body)
        .send()
        .await
        .expect("Failed to execute request.");

    // Assert
    assert_eq!(201, response.status().as_u16());

    let saved = sqlx::query!("SELECT email, name FROM subscriptions",)
        .fetch_one(&mut connection)
        .await
        .expect("Failed to fetch saved subscription.");
    assert_eq!(saved.email, "ursula_le_guin@gmail.com");
    assert_eq!(saved.name, "le guin");
}

#[tokio::test]
async fn subscribe_returns_a_422_when_data_is_missing() {
    // Arrange
    let app_address = spawn_app().await.expect("Failed to spawn application");
    let client = reqwest::Client::new();
    let test_cases = vec![
        ("name=le%20guin", "missing the email"),
        ("email=ursula_le_guin%40gmail.com", "missing the name"),
        ("", "missing both name and email"),
    ];

    for (invalid_body, error_message) in test_cases {
        // Act
        let response = client
            .post(&format!("{}/subscriptions", &app_address))
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
}
