use actix_web::{web, App, HttpServer, middleware, error};
use actix_web::dev::Service;
use actix_web::HttpMessage;
use actix_cors::Cors;
use actix_files::Files;
use dotenv::dotenv;
use env_logger;
use log;
use std::env;
use jsonwebtoken::{decode, DecodingKey, Validation, Algorithm};

mod api;
mod auth;
mod db;
mod models;
mod utils;

// Implement Clone for AppDb
impl Clone for db::AppDb {
    fn clone(&self) -> Self {
        db::AppDb {
            conn: self.conn.clone(),
        }
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Load .env file
    dotenv().ok();
    
    // Initialize logger
    env_logger::init();
    
    // Get configuration from env
    let host = env::var("HOST").unwrap_or_else(|_| "127.0.0.1".to_string());
    let port = env::var("PORT").unwrap_or_else(|_| "8080".to_string()).parse::<u16>().unwrap();
    let jwt_secret = env::var("JWT_SECRET").expect("JWT_SECRET must be set");
    let static_path = env::var("STATIC_FILES_PATH").unwrap_or_else(|_| "./static".to_string());
    
    // Initialize database
    let db = match db::init_db() {
        Ok(db) => {
            log::info!("Database initialized successfully");
            db
        },
        Err(e) => {
            log::error!("Failed to initialize database: {}", e);
            panic!("Database initialization failed");
        }
    };
    
    let db_data = web::Data::new(db);
    let jwt_secret_bytes = jwt_secret.as_bytes().to_vec();
    
    // Start HTTP server
    log::info!("Starting server at http://{}:{}", host, port);
    log::info!("Serving static files from: {}", static_path);
    
    // Store static path for closures
    let static_files_path = static_path.clone();
    
    HttpServer::new(move || {
        // CORS configuration
        let cors = Cors::default()
            .allow_any_origin()
            .allow_any_method()
            .allow_any_header()
            .max_age(3600);
        
        // Clone the paths for closures
        let files_path = static_files_path.clone();
        let index_path = static_files_path.clone();
        let secret = jwt_secret_bytes.clone();
        
        App::new()
            .wrap(middleware::Logger::default())
            .wrap(cors)
            .app_data(db_data.clone())
            // API routes with JWT validation middleware
            .service(
                web::scope("/api")
                    // Use wrap_fn instead of a custom middleware
                    .wrap_fn(move |req, srv| {
                        let path = req.path().to_string();
                        let secret = secret.clone();
                        
                        // Skip auth for public routes
                        if path.contains("/auth/") {
                            return srv.call(req);
                        }
                        
                        // Check for JWT token
                        if let Some(auth_header) = req.headers().get("Authorization") {
                            if let Ok(auth_str) = auth_header.to_str() {
                                if auth_str.starts_with("Bearer ") {
                                    let token = &auth_str[7..]; // Skip "Bearer " prefix
                                    
                                    match decode::<auth::Claims>(
                                        token,
                                        &DecodingKey::from_secret(&secret),
                                        &Validation::new(Algorithm::HS256)
                                    ) {
                                        Ok(token_data) => {
                                            req.extensions_mut().insert(token_data.claims);
                                            return srv.call(req);
                                        },
                                        Err(_) => {
                                            return Box::pin(async move {
                                                Err(error::ErrorUnauthorized("Invalid token"))
                                            });
                                        }
                                    }
                                }
                            }
                        }
                        
                        // No valid auth
                        Box::pin(async move {
                            Err(error::ErrorUnauthorized("Authorization required"))
                        })
                    })
                    .configure(api::configure)
                    .configure(auth::configure)
            )
            // Serve static files for frontend
            .service(
                Files::new("/", &files_path)
                    .index_file("index.html")
                    .default_handler(
                        web::route().to(move || {
                            // Using a captured string clone
                            let path_str = index_path.clone();
                            async move {
                                let full_path = format!("{}/index.html", path_str);
                                match std::fs::read_to_string(&full_path) {
                                    Ok(content) => actix_web::HttpResponse::Ok()
                                        .content_type("text/html; charset=utf-8")
                                        .body(content),
                                    Err(e) => {
                                        log::error!("Failed to read index.html: {}", e);
                                        actix_web::HttpResponse::InternalServerError()
                                            .body("Failed to load application")
                                    }
                                }
                            }
                        })
                    )
            )
    })
    .bind((host, port))?
    .run()
    .await
}
