use axum2prod::{get_config, run};
use std::net::TcpListener;

#[tokio::main]
async fn main() -> hyper::Result<()> {
    let config = get_config();
    let address = format!("127.0.0.1:{}", config.port);
    //.context("Failed to connect to Postgres.")?;
    let listener =
        TcpListener::bind(&address).unwrap_or_else(|_| panic!("Failed to bind to {}", &address));
    run(listener, config.db().await)?.await
}
