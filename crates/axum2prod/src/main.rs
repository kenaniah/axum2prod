use axum2prod::{get_config, run};
use env_logger::Env;
use std::net::TcpListener;

#[tokio::main]
async fn main() -> hyper::Result<()> {
    // Initialize the logger
    env_logger::Builder::from_env(Env::default().default_filter_or("debug")).init();

    // Configure the server
    let config = get_config();
    let address = format!("127.0.0.1:{}", config.port);
    //.context("Failed to connect to Postgres.")?;
    let listener =
        TcpListener::bind(&address).unwrap_or_else(|_| panic!("Failed to bind to {}", &address));

    // Run the server
    run(listener, config.db().await)?.await
}
