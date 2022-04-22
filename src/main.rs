//! src/main.rs
use std::net::TcpListener;
use newsletterAPI::startup::run;


#[tokio::main]
async fn main() -> std::io::Result<()> {
    // Using the 0 port
    let listener = TcpListener::bind("0.0.0.0:0").expect("Failed to bind random port");
    
    run(listener)?.await 
}