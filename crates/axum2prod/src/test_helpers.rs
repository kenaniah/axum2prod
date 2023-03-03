use crate::telemetry::{get_subscriber, init_subscriber};
use once_cell::sync::Lazy;
use std::{future::Future, net::TcpListener, pin::Pin, time::Duration};

use crate::configuration;
use sqlx::PgPool;

// Ensure that the `tracing` stack is only initialized once using `once_cell`
static TRACING: Lazy<()> = Lazy::new(|| {
    let default_filter_level = "info".to_string();
    let subscriber_name = "test".to_string();
    if std::env::var("TEST_LOG").is_ok() {
        let subscriber = get_subscriber(subscriber_name, default_filter_level, std::io::stdout);
        init_subscriber(subscriber);
    } else {
        let subscriber = get_subscriber(subscriber_name, default_filter_level, std::io::sink);
        init_subscriber(subscriber);
    };
});

pub struct TestContext {
    pub address: String,
    pub db: PgPool,
}

/// Wrapper for tests to ensure each is run in an isolated environment
pub async fn run_test<T>(test: T)
where
    T: FnOnce(TestContext) -> Pin<Box<dyn Future<Output = ()> + Send>> + std::panic::UnwindSafe,
{
    // Configure the logger
    Lazy::force(&TRACING);

    // Spin up a random port
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind random port");
    let port = listener.local_addr().unwrap().port();
    let address = format!("http://127.0.0.1:{}", port);

    // Spin up a test database
    let test_db = configuration::get_config().test_db().await;

    // Spin up the server in the background
    let server = crate::run(listener, test_db.pool.clone()).unwrap();
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
