// src/main.rs
use actix_web::{
    dev::Payload, error, get, http, middleware, post, web, App, Error, FromRequest, HttpMessage,
    HttpRequest, HttpResponse, HttpServer, Responder,
};
use actix_session::{Session, SessionMiddleware, storage::CookieSessionStore};
use actix_cors::Cors;
use actix_web::cookie::{Key, SameSite};
use dotenv::dotenv;
use futures::future::{ready, Ready};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use sled::Db;
use std::env;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;
use uuid::Uuid;

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

    fn from_request(req: &HttpRequest, _: &mut Payload) -> Self::Future {
        // Check if the user is authenticated via session
        if let Some(user) = req.extensions().get::<User>() {
            ready(Ok(AuthenticatedUser(user.clone())))
        } else {
            ready(Err(error::ErrorUnauthorized("Unauthorized")))
        }
    }
}

// Auth middleware
fn auth_middleware<S, B>(
    req: HttpRequest,
    session: Session,
    srv: actix_web::dev::Service<HttpRequest, Response = actix_web::dev::ServiceResponse<B>, Error = Error>,
) -> Pin<Box<dyn Future<Output = Result<actix_web::dev::ServiceResponse<B>, Error>>>> {
    let fut = async move {
        // Skip auth for login-related endpoints
        let path = req.path();
        if path == "/api/login" || path == "/api/callback" || path == "/api/auth-url" {
            return srv.call(req).await;
        }

        // Check if the user is authenticated
        if let Ok(Some(user_json)) = session.get::<String>("user") {
            match serde_json::from_str::<User>(&user_json) {
                Ok(user) => {
                    // Check if the token is expired (in a real app, you'd refresh it)
                    let current_time = chrono::Utc::now().timestamp() as u64;
                    if user.expires_at > current_time {
                        // Add user to request extensions
                        req.extensions_mut().insert(user);
                        return srv.call(req).await;
                    }
                }
                Err(_) => {}
            }
        }

        // Not authenticated
        Ok(actix_web::dev::ServiceResponse::new(
            req,
            HttpResponse::Unauthorized().json(serde_json::json!({
                "error": "Unauthorized",
                "message": "You must be logged in to access this resource"
            })),
        ))
    };

    Box::pin(fut)
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
    // Get code and state from query params
    let query = req.query_string();
    let mut params = web::Query::<std::collections::HashMap<String, String>>::from_query(query)
        .unwrap_or_default();

    let code = match params.remove("code") {
        Some(code) => code,
        None => return HttpResponse::BadRequest().body("Missing code parameter"),
    };

    // Exchange the code for an access token
    let params = [
        ("grant_type", "authorization_code"),
        ("code", &code),
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
async fn get_me(user: AuthenticatedUser) -> impl Responder {
    HttpResponse::Ok().json(serde_json::json!({
        "username": user.0.username,
        "id": user.0.id
    }))
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
    let secret_key = env::var("SECRET_KEY").unwrap_or_else(|_| Uuid::new_v4().to_string());

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
        let key = Key::from(secret_key.as_bytes());

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
            .wrap_fn(auth_middleware)
            .app_data(app_state.clone())
            .service(get_auth_url)
            .service(oauth_callback)
            .service(get_me)
            .service(logout)
            .service(protected_endpoint)
            // Serve static files from the frontend build directory
            .service(actix_files::Files::new("/", "./dist").index_file("index.html"))
    })
    .bind(format!("{}:{}", host, port))?
    .run()
    .await
}
