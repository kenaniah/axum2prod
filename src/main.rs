use axum2prod::{get_configuration, run};
use std::net::TcpListener;

#[tokio::main]
async fn main() -> hyper::Result<()> {
    let config = get_configuration().expect("Failed to read configuration.");
    let address = format!("127.0.0.1:{}", config.application_port);
    let listener = TcpListener::bind(&address).expect(&format!("Failed to bind to {}", &address));
    run(listener)?.await
}
