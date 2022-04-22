//! src/main.rs
use std::net::TcpListener;
use newsletterAPI::startup::run;
use newsletterAPI::configuration::get_configuration;


#[tokio::main]
async fn main() -> std::io::Result<()> {
    // Panic if it can't read configuration
    let configuration = get_configuration().expect("Failed to read configuration.");
    
    // Port comes from settings
    let address = format!("127.0.0.1:{}", configuration.application_port);
    let listener = TcpListener::bind(address)?;

    run(listener)?.await
}
