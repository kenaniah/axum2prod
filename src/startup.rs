use axum::{
    http::StatusCode,
    routing::{get, post, IntoMakeService},
    Router, Server,
};
use hyper::{self, server::conn::AddrIncoming};
use std::net::TcpListener;

#[derive(Clone)]
struct AppState {
    db_pool: sqlx::PgPool,
}

pub fn run(
    listener: TcpListener,
) -> Result<Server<AddrIncoming, IntoMakeService<Router>>, hyper::Error> {
    // build our application with a single route
    let app = Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .route("/health_check", get(|| async { StatusCode::OK }))
        .route("/subscriptions", post(crate::routes::subscribe));

    // run it
    Ok(axum::Server::from_tcp(listener)?.serve(app.into_make_service()))
}
