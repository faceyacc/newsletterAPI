//! src/main.rs
use newsletterAPI::startup::run;
use newsletterAPI::configuration::get_configuration;
use sqlx::PgPool;
use uuid::Uuid;
use std::net::TcpListener;



#[tokio::main]
async fn main() -> std::io::Result<()> {
    // Panic if it can't read configuration
    let mut configuration = get_configuration().expect("Failed to read configuration.");
    
    // Randomize database name
    configuration.database.database_name = Uuid::new_v4().to_string();

    let connection_pool = PgPool::connect(&configuration.database.connection_string())
        .await
        .expect("Failed to connect to Postgres.");
    
    // Port comes from settings
    let address = format!("127.0.0.1:{}", configuration.application_port);
    let listener = TcpListener::bind(address)?;

    run(listener, connection_pool)?.await

}
