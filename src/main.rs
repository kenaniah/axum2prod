use axum2prod::{get_config, run};
use std::net::TcpListener;

#[tokio::main]
async fn main() -> hyper::Result<()> {
    let config = get_config();
    let address = format!("127.0.0.1:{}", config.port);
    let listener = TcpListener::bind(&address).expect(&format!("Failed to bind to {}", &address));
    run(listener)?.await
}
