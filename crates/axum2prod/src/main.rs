use axum2prod::{get_config, run};
use std::net::TcpListener;
use tracing::subscriber::set_global_default;
use tracing_bunyan_formatter::{BunyanFormattingLayer, JsonStorageLayer};
use tracing_log::LogTracer;
use tracing_subscriber::{layer::SubscriberExt, EnvFilter, Registry};

#[tokio::main]
async fn main() -> hyper::Result<()> {
    // Configure logging and tracing
    LogTracer::init().expect("Failed to set logger");

    let env_filter = EnvFilter::try_from_default_env().unwrap_or(EnvFilter::new("info"));
    let formatting_layer = BunyanFormattingLayer::new("axum2prod".into(), std::io::stdout);
    let subscriber = Registry::default()
        .with(env_filter)
        .with(JsonStorageLayer)
        .with(formatting_layer);
    set_global_default(subscriber).expect("Failed to set subscriber");

    // Configure the server
    let config = get_config();
    let address = format!("127.0.0.1:{}", config.port);
    //.context("Failed to connect to Postgres.")?;
    let listener =
        TcpListener::bind(&address).unwrap_or_else(|_| panic!("Failed to bind to {}", &address));

    // Run the server
    run(listener, config.db().await)?.await
}
