use axum::{
    http::StatusCode,
    routing::{get, post, IntoMakeService},
    Router, Server,
};
use hyper::{self, server::conn::AddrIncoming};
use sqlx::PgPool;
use std::net::TcpListener;

pub mod error;

/// Context that is shared across all requests
#[derive(Clone)]
pub struct AppContext {
    pub db: PgPool,
}

/// Builds the application's routes and returns a server instance
/// that can be run on a tokio runtime
pub fn run(
    listener: TcpListener,
    db: sqlx::PgPool,
) -> Result<Server<AddrIncoming, IntoMakeService<Router>>, hyper::Error> {
    let app_state = AppContext { db };

    // build the application's routes
    let app = Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .route("/health_check", get(|| async { StatusCode::OK }))
        .route("/subscriptions", post(crate::routes::subscribe))
        .with_state(app_state);

    // return a server instance
    Ok(axum::Server::from_tcp(listener)?.serve(app.into_make_service()))
}
