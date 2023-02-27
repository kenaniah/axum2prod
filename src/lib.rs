use axum::Form;
use axum::{
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post, IntoMakeService},
    Router, Server,
};
use hyper::{self, server::conn::AddrIncoming};
use std::net::TcpListener;

pub fn run(
    listener: TcpListener,
) -> Result<Server<AddrIncoming, IntoMakeService<Router>>, hyper::Error> {
    // build our application with a single route
    let app = Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .route("/health_check", get(|| async { StatusCode::OK }))
        .route("/subscriptions", post(subscribe));

    // run it
    Ok(axum::Server::from_tcp(listener)?.serve(app.into_make_service()))
}

#[derive(serde::Deserialize)]
struct FormData {
    name: String,
    email: String,
}

async fn subscribe(Form(_data): Form<FormData>) -> impl IntoResponse {
    StatusCode::CREATED
}
