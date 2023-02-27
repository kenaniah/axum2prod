use std::net::TcpListener;

// Launch our application in the background ~somehow~
async fn spawn_app() -> hyper::Result<String> {
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind random port");
    let port = listener.local_addr().unwrap().port();
    let server = axum2prod::run(listener)?;
    let _ = tokio::spawn(server);
    // We return the application address to the caller!
    Ok(format!("http://127.0.0.1:{}", port))
}

#[tokio::test]
async fn health_check_works() {
    let address = spawn_app().await.expect("Failed to spawn application");

    let client = reqwest::Client::new();
    let response = client
        .get(&format!("{}/health_check", &address))
        .send()
        .await
        .expect("Failed to execute request.");
    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}
