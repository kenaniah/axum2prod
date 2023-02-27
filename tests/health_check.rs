// Launch our application in the background ~somehow~
async fn spawn_app() -> hyper::Result<()> {
    axum2prod::run().await
}

#[tokio::test]
async fn health_check_works() {
    spawn_app().await.expect("Failed to spawn application");

    let client = reqwest::Client::new();
    let response = client
        .get("http://127.0.0.1:8000/health_check")
        .send()
        .await
        .expect("Failed to execute request.");
    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}
