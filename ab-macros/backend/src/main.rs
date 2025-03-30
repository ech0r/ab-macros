// backend/src/main.rs
use ab_macros_backend::{config::Config, server::run_server};
use std::sync::Arc;
use tracing::{info, Level};
use tracing_subscriber::FmtSubscriber;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Load configuration from environment or config file
    let config = Config::from_env().expect("Failed to load configuration");
    
    // Set up logging
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::INFO)
        .finish();
    
    tracing::subscriber::set_global_default(subscriber)
        .expect("Failed to set up logging");
    
    info!("Starting ab-macros server");
    
    // Run the server
    let config = Arc::new(config);
    run_server(config).await
}
