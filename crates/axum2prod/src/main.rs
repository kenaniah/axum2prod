use axum2prod::telemetry::{get_subscriber, init_subscriber};
use axum2prod::{get_config, run};
use std::net::TcpListener;

#[tokio::main]
async fn main() -> hyper::Result<()> {
    // Configure the logger
    let subscriber = get_subscriber("axum2prod".into(), "info".into(), std::io::stdout);
    init_subscriber(subscriber);

    // Configure the server
    let config = get_config();
    let address = format!("127.0.0.1:{}", config.port);
    //.context("Failed to connect to Postgres.")?;
    let listener =
        TcpListener::bind(&address).unwrap_or_else(|_| panic!("Failed to bind to {}", &address));

    // Run the server
    run(listener, config.db().await)?.await
}
