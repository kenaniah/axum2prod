use axum2prod::run;

#[tokio::main]
async fn main() -> hyper::Result<()> {
    run().await
}
