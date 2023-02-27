use axum::{
    http::StatusCode,
    routing::{get, IntoMakeService},
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
        .route("/health_check", get(|| async { StatusCode::OK }));

    // run it
    Ok(axum::Server::from_tcp(listener)?.serve(app.into_make_service()))
}
