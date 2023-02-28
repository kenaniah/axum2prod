use axum::{
    http::StatusCode,
    routing::{get, post, IntoMakeService},
    Router, Server,
};
use hyper::{self, server::conn::AddrIncoming};
use std::net::TcpListener;

#[derive(Clone)]
struct AppState {
    db: sqlx::PgPool,
}

pub fn run(
    listener: TcpListener,
    db: sqlx::PgPool,
) -> Result<Server<AddrIncoming, IntoMakeService<Router>>, hyper::Error> {
    let app_state = AppState { db };

    // build our application with a single route
    let app = Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .route("/health_check", get(|| async { StatusCode::OK }))
        .route("/subscriptions", post(crate::routes::subscribe))
        .with_state(app_state);

    // run it
    Ok(axum::Server::from_tcp(listener)?.serve(app.into_make_service()))
}
