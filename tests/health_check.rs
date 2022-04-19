//! tests/health_check.rs 
use std::net::TcpListener;


#[tokio::test]
async fn health_check_works() {
    // Arrange
    let address = spawn_app();
    let client = reqwest::Client::new();

    // Act
    let response = client
    .get(&format!("{}/health_check", &address)).send()
    .await
    .expect("Failed to execute request.");

    // Assert 
    assert!(response.status().is_success()); 
    assert_eq!(Some(0), response.content_length());
}



fn spawn_app() -> String {
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind random port");
    
    // Retrieve the port assigned to use by the OS
    let port = listener.local_addr().unwrap().port();
    let server = newsletterAPI::run(listener).expect("Failed to bind address");

    // Use tokio spawn to run the server in the background
    let _ = tokio::spawn(server);

    // Return the application address to the caller
    format!("http://127.0.0.1:{}", port)
}