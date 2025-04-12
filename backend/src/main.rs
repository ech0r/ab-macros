use actix_web::{
    dev::Service, get, http, middleware, post, web, App, Error, 
    FromRequest, HttpMessage, HttpRequest, HttpResponse, HttpServer, Responder,
};
use actix_session::{Session, SessionMiddleware, storage::CookieSessionStore, SessionExt};
use actix_cors::Cors;
use actix_web::cookie::{Key, SameSite};
use dotenv::dotenv;
use futures::future::{ready, Ready};
use reqwest::Client;
use rust_embed::RustEmbed;
use serde::{Deserialize, Serialize};
use sled::Db;
use std::env;
use std::sync::Arc;
use uuid::Uuid;

// Embed frontend assets
#[derive(RustEmbed)]
#[folder = "../frontend-dist/"]
struct FrontendAssets;

// AppState will hold our database and HTTP client
struct AppState {
    db: Arc<Db>,
    http_client: Client,
    reddit_client_id: String,
    reddit_client_secret: String,
    redirect_uri: String,
}

// User struct for storing user information
#[derive(Serialize, Deserialize, Clone, Debug)]
struct User {
    id: String,
    username: String,
    access_token: String,
    refresh_token: Option<String>,
    expires_at: u64,
}

// Reddit OAuth response
#[derive(Deserialize, Debug)]
struct RedditTokenResponse {
    access_token: String,
    token_type: String,
    expires_in: u64,
    refresh_token: Option<String>,
    scope: String,
}

// Reddit user info response
#[derive(Deserialize, Debug)]
struct RedditUserInfo {
    name: String,
    id: String,
}

// Authentication extractor
struct AuthenticatedUser(User);

impl FromRequest for AuthenticatedUser {
    type Error = Error;
    type Future = Ready<Result<Self, Self::Error>>;

    fn from_request(req: &HttpRequest, _: &mut actix_web::dev::Payload) -> Self::Future {
        // Get session
        let session = req.get_session();
        
        // Check if the user is authenticated via session
        if let Ok(Some(user_json)) = session.get::<String>("user") {
            if let Ok(user) = serde_json::from_str::<User>(&user_json) {
                let current_time = chrono::Utc::now().timestamp() as u64;
                if user.expires_at > current_time {
                    return ready(Ok(AuthenticatedUser(user)));
                }
            }
        }
        
        ready(Err(actix_web::error::ErrorUnauthorized("Unauthorized")))
    }
}

// Get the Reddit authorization URL
#[get("/api/auth-url")]
async fn get_auth_url(data: web::Data<AppState>) -> impl Responder {
    let state = Uuid::new_v4().to_string();
    let auth_url = format!(
        "https://www.reddit.com/api/v1/authorize?client_id={}&response_type=code&state={}&redirect_uri={}&duration=permanent&scope=identity",
        data.reddit_client_id, state, data.redirect_uri
    );

    HttpResponse::Ok().json(serde_json::json!({
        "url": auth_url,
        "state": state
    }))
}

// OAuth callback handler
#[get("/api/callback")]
async fn oauth_callback(
    req: HttpRequest,
    data: web::Data<AppState>,
    session: Session,
) -> impl Responder {
    // Get code from query params
    let query = req.query_string();
    let mut query_map = std::collections::HashMap::new();
    
    // Manually parse the query parameters
    for pair in query.split('&') {
        if pair.is_empty() {
            continue;
        }
        let mut parts = pair.split('=');
        if let (Some(key), Some(value)) = (parts.next(), parts.next()) {
            query_map.insert(key.to_string(), value.to_string());
        }
    }
    
    let code = match query_map.get("code") {
        Some(code) => code,
        None => return HttpResponse::BadRequest().body("Missing code parameter"),
    };

    // Exchange the code for an access token
    let params = [
        ("grant_type", "authorization_code"),
        ("code", code),
        ("redirect_uri", &data.redirect_uri),
    ];

    let client = &data.http_client;
    let token_response = match client
        .post("https://www.reddit.com/api/v1/access_token")
        .basic_auth(&data.reddit_client_id, Some(&data.reddit_client_secret))
        .form(&params)
        .send()
        .await
    {
        Ok(response) => match response.json::<RedditTokenResponse>().await {
            Ok(token_data) => token_data,
            Err(err) => {
                eprintln!("Failed to parse token response: {:?}", err);
                return HttpResponse::InternalServerError().body("Failed to parse token response");
            }
        },
        Err(err) => {
            eprintln!("Token request failed: {:?}", err);
            return HttpResponse::InternalServerError().body("Token request failed");
        }
    };

    // Get user info from Reddit
    let user_info = match client
        .get("https://oauth.reddit.com/api/v1/me")
        .header(
            http::header::AUTHORIZATION,
            format!("Bearer {}", token_response.access_token),
        )
        .header(http::header::USER_AGENT, "ab-macros:0.1.0 (by /u/USERNAME)")
        .send()
        .await
    {
        Ok(response) => match response.json::<RedditUserInfo>().await {
            Ok(user_data) => user_data,
            Err(err) => {
                eprintln!("Failed to parse user info: {:?}", err);
                return HttpResponse::InternalServerError().body("Failed to parse user info");
            }
        },
        Err(err) => {
            eprintln!("User info request failed: {:?}", err);
            return HttpResponse::InternalServerError().body("User info request failed");
        }
    };

    // Calculate token expiration time
    let current_time = chrono::Utc::now().timestamp() as u64;
    let expires_at = current_time + token_response.expires_in;

    // Create user object
    let user = User {
        id: user_info.id,
        username: user_info.name,
        access_token: token_response.access_token,
        refresh_token: token_response.refresh_token,
        expires_at,
    };

    // Store user in the database
    match data.db.insert(
        format!("user:{}", user.id),
        serde_json::to_vec(&user).unwrap(),
    ) {
        Ok(_) => {
            // Store user in session
            if let Err(err) = session.insert("user", serde_json::to_string(&user).unwrap()) {
                eprintln!("Failed to store user in session: {:?}", err);
                return HttpResponse::InternalServerError().body("Failed to store user in session");
            }

            // Redirect to frontend with success
            HttpResponse::Found()
                .append_header((http::header::LOCATION, "/"))
                .finish()
        }
        Err(err) => {
            eprintln!("Failed to store user in database: {:?}", err);
            HttpResponse::InternalServerError().body("Failed to store user in database")
        }
    }
}

// Get current user info
#[get("/api/me")]
async fn get_me(user: Option<AuthenticatedUser>) -> impl Responder {
    match user {
        Some(user) => HttpResponse::Ok().json(serde_json::json!({
            "username": user.0.username,
            "id": user.0.id
        })),
        None => HttpResponse::Unauthorized().json(serde_json::json!({
            "error": "Unauthorized",
            "message": "You must be logged in to access this resource"
        })),
    }
}

// Logout endpoint
#[post("/api/logout")]
async fn logout(session: Session) -> impl Responder {
    session.purge();
    HttpResponse::Ok().json(serde_json::json!({
        "message": "Logged out successfully"
    }))
}

// Protected endpoint example
#[get("/api/protected")]
async fn protected_endpoint(user: AuthenticatedUser) -> impl Responder {
    HttpResponse::Ok().json(serde_json::json!({
        "message": format!("Hello, {}! This is a protected endpoint.", user.0.username)
    }))
}

// Handle embedded frontend assets
async fn handle_embedded_assets(req: HttpRequest) -> HttpResponse {
    let path = if req.path() == "/" {
        // Serve index.html for root path
        "index.html"
    } else {
        // Remove leading slash
        &req.path()[1..]
    };

    // Try to find the file in embedded assets
    match FrontendAssets::get(path) {
        Some(content) => {
            // Guess the MIME type based on the file extension
            let mime_type = mime_guess::from_path(path).first_or_octet_stream();
            
            HttpResponse::Ok()
                .content_type(mime_type.as_ref())
                .body(content.data.into_owned())
        }
        None => {
            // If the asset doesn't exist, try to serve index.html for client-side routing
            match FrontendAssets::get("index.html") {
                Some(content) => HttpResponse::Ok()
                    .content_type("text/html")
                    .body(content.data.into_owned()),
                None => HttpResponse::NotFound().body("Not found"),
            }
        }
    }
}

// Middleware for checking user authentication from session
struct AuthMiddleware;

impl<S, B> actix_web::dev::Transform<S, actix_web::dev::ServiceRequest> for AuthMiddleware
where
    S: Service<actix_web::dev::ServiceRequest, Response = actix_web::dev::ServiceResponse<B>, Error = Error> + 'static + Clone,
    S::Future: 'static + Send,
    B: 'static,
{
    type Response = actix_web::dev::ServiceResponse<B>;
    type Error = Error;
    type Transform = AuthMiddlewareService<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(AuthMiddlewareService { service }))
    }
}

pub struct AuthMiddlewareService<S> {
    service: S,
}

impl<S> Clone for AuthMiddlewareService<S>
where
    S: Clone,
{
    fn clone(&self) -> Self {
        Self {
            service: self.service.clone(),
        }
    }
}

impl<S, B> Service<actix_web::dev::ServiceRequest> for AuthMiddlewareService<S>
where
    S: Service<actix_web::dev::ServiceRequest, Response = actix_web::dev::ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static + Send,
    B: 'static,
{
    type Response = actix_web::dev::ServiceResponse<B>;
    type Error = Error;
    type Future = futures::future::BoxFuture<'static, Result<Self::Response, Self::Error>>;

    actix_web::dev::forward_ready!(service);

    fn call(&self, req: actix_web::dev::ServiceRequest) -> Self::Future {
        let path = req.path().to_string();
        let service = self.service.clone();
        
        // Skip auth for non-API paths and public endpoints
        if !path.starts_with("/api/") || path == "/api/auth-url" || path == "/api/callback" || path == "/api/logout" {
            return Box::pin(service.call(req));
        }
        
        // Try to get the session
        let session = req.get_session();
        
        // Check for user in session
        Box::pin(async move {
            if let Ok(Some(user_json)) = session.get::<String>("user") {
                if let Ok(user) = serde_json::from_str::<User>(&user_json) {
                    let current_time = chrono::Utc::now().timestamp() as u64;
                    if user.expires_at > current_time {
                        // User is authenticated, add to request extensions
                        req.extensions_mut().insert(user);
                        return service.call(req).await;
                    }
                }
            }
            
            // Not authenticated, return 401
            let (request, _) = req.into_parts();
            let response = HttpResponse::Unauthorized()
                .json(serde_json::json!({
                    "error": "Unauthorized",
                    "message": "You must be logged in to access this resource"
                }));

            // Convert to expected body type
            Ok(actix_web::dev::ServiceResponse::new(
                request, 
                response.map_into_boxed_body()
            ))
        })
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Load environment variables from .env file
    dotenv().ok();

    // Initialize logger
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    // Get environment variables
    let reddit_client_id = env::var("REDDIT_CLIENT_ID").expect("REDDIT_CLIENT_ID must be set");
    let reddit_client_secret = env::var("REDDIT_CLIENT_SECRET").expect("REDDIT_CLIENT_SECRET must be set");
    let redirect_uri = env::var("REDDIT_REDIRECT_URI").expect("REDDIT_REDIRECT_URI must be set");
    let host = env::var("HOST").unwrap_or_else(|_| "127.0.0.1".to_string());
    let port = env::var("PORT").unwrap_or_else(|_| "8080".to_string());
    
    // Generate a secure key for sessions (ensure it's at least 32 bytes)
    let secret_key = env::var("SECRET_KEY").unwrap_or_else(|_| {
        // Generate a key with enough entropy
        format!("{}{}{}", Uuid::new_v4(), Uuid::new_v4(), Uuid::new_v4())
    });
    
    // Ensure key is at least 32 bytes
    let mut key_data = secret_key.as_bytes().to_vec();
    if key_data.len() < 32 {
        key_data.resize(32, 0);
    }

    // Open Sled database
    let db = Arc::new(
        sled::open("data.db").expect("Failed to open database"),
    );

    // Create HTTP client
    let http_client = Client::new();

    // Create app state
    let app_state = web::Data::new(AppState {
        db,
        http_client,
        reddit_client_id,
        reddit_client_secret,
        redirect_uri,
    });

    println!("Starting server at http://{}:{}", host, port);

    // Start HTTP server
    HttpServer::new(move || {
        // Configure CORS
        let cors = Cors::default()
            .allowed_origin("http://localhost:8080")
            .allowed_methods(vec!["GET", "POST"])
            .allowed_headers(vec![http::header::AUTHORIZATION, http::header::CONTENT_TYPE])
            .supports_credentials()
            .max_age(3600);

        // Create cookie key for session
        let key = Key::from(&key_data);

        App::new()
            .wrap(middleware::Logger::default())
            .wrap(
                SessionMiddleware::builder(CookieSessionStore::default(), key.clone())
                    .cookie_secure(false) // Set to true in production with HTTPS
                    .cookie_http_only(true)
                    .cookie_same_site(SameSite::Lax)
                    .build(),
            )
            .wrap(cors)
            .app_data(app_state.clone())
            .service(get_auth_url)
            .service(oauth_callback)
            .service(get_me)
            .service(logout)
            .service(protected_endpoint)
            // Default route handler for embedded frontend assets
            .default_service(web::to(handle_embedded_assets))
    })
    .bind(format!("{}:{}", host, port))?
    .run()
    .await
}
