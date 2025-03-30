// backend/src/server.rs
use crate::{
    api::routes::configure_routes,
    config::Config,
    db::sled::SledDb,
    auth::twilio::TwilioClient,
};
use actix_cors::Cors;
use actix_files::Files;
use actix_web::{
    middleware,
    web::{self, Data},
    App, HttpServer,
};
use std::sync::Arc;
use tracing::info;

pub async fn run_server(config: Arc<Config>) -> std::io::Result<()> {
    // Initialize the database
    let db = SledDb::new(&config.db.path).expect("Failed to initialize database");
    let db = Data::new(db);
    
    // Initialize Twilio client
    let twilio = TwilioClient::new(
        &config.twilio.account_sid,
        &config.twilio.auth_token,
        &config.twilio.from_number,
        config.twilio.enabled,
        &config.twilio.test_user_id,
        &config.twilio.test_user_phone,
    );
    let twilio = Data::new(twilio);
    
    let config_data = Data::new(config.clone());
    
    let bind_address = format!("{}:{}", config.server.host, config.server.port);
    info!("Starting server on {}", bind_address);
    
    HttpServer::new(move || {
        // Configure CORS
        let cors = Cors::default()
            .allow_any_origin()
            .allow_any_method()
            .allow_any_header()
            .max_age(3600);
        
        App::new()
            // Enable logger middleware
            .wrap(middleware::Logger::default())
            // Enable CORS
            .wrap(cors)
            // Register the database connection
            .app_data(db.clone())
            // Register the Twilio client
            .app_data(twilio.clone())
            // Register the configuration
            .app_data(config_data.clone())
            // Configure API routes
            .configure(configure_routes)
            // Serve static files
            .service(Files::new("/", "./static").index_file("index.html"))
    })
    .workers(config.server.workers)
    .bind(bind_address)?
    .run()
    .await
}
