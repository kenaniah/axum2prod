use axum2prod::run;
use std::net::TcpListener;

#[tokio::main]
async fn main() -> hyper::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:3000").expect("Failed to bind to port 3000");
    run(listener)?.await
}
