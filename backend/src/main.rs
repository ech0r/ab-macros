use core::error;
use std::{boxed, env};
use actix_session::storage::SessionStore as ActixSessionStore;
use actix_web::body::MessageBody;
use actix_web::dev::{ServiceRequest, ServiceResponse};
use actix_web::HttpResponse;
use log::{LevelFilter, info, warn, error, debug};
use env_logger::{Builder, Env};
use actix_web::{middleware::{from_fn, Next}, get, post, web, App, http::header, cookie::Cookie, HttpServer, Responder, Error as ActixError};
use reqwest::{Client, Response, Error as ReqwestError};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::error::Error;
use std::fmt;
use sled::Db;
use std::sync::Arc;
use chrono::{Duration, Local};

// local stuff
mod models;
use crate::models::{UserSession, RedditUser, SessionStore};

#[derive(Clone, Debug)]
struct AppState {
    env_config: EnvConfig,
    session_store: SessionStore,
}

impl AppState {
    fn new(env_config: &EnvConfig, session_store: SessionStore) -> Self {
        AppState {
            env_config: env_config.clone(),
            session_store
        }
    }
}

#[derive(Clone, Debug, Serialize)]
struct TokenRequest {
    grant_type: String,
    code: String,
    redirect_uri: String,
}

impl TokenRequest {
    fn new(code: String, env_config: &EnvConfig) -> Self {
        TokenRequest {
            grant_type: "authorization_code".to_owned(),
            code: code.clone(),
            redirect_uri: env_config.reddit_redirect_uri.clone(),
        }
    }
}

#[derive(Clone, Debug, Deserialize)]
struct TokenResponse {
    access_token: String,
    token_type: String,
    expires_in: u64,
    refresh_token: Option<String>,
    scope: String,
}

#[derive(Clone, Debug, Deserialize)]
struct SessionCookie {
    message: String,
}

#[derive(Clone, Debug)]
struct EnvConfig {
    reddit_client_id: String,
    reddit_client_secret: String,
    reddit_redirect_uri: String,
    reddit_auth_uri: String,
    reddit_access_uri: String,
    reddit_author: String,
    reddit_get_user_uri: String,
    log_level: LevelFilter,
}


impl EnvConfig {
    fn is_empty(&self) -> bool {
        self.reddit_redirect_uri.is_empty() || self.reddit_client_id.is_empty() || self.reddit_client_secret.is_empty() || self.reddit_access_uri.is_empty()
    }

    fn new() -> Self {
        let mut env_config = EnvConfig {
            reddit_client_id: String::new(),
            reddit_client_secret: String::new(),
            reddit_redirect_uri: String::new(),
            reddit_auth_uri: String::new(),
            reddit_access_uri: String::new(),
            reddit_author: String::new(),
            reddit_get_user_uri: String::new(),
            log_level: LevelFilter::Off,
        };
        let env_file = include_str!(".env");
        let env_file_lines = env_file.lines();
        for line in env_file_lines {
            let mut line_parts = line.split("=");
            match line_parts.next() {
                None => panic!("[ERROR]: Invalid config file at build time"),
                Some(part) => {
                    match part {
                       "REDDIT_CLIENT_ID" => env_config.reddit_client_id = line_parts.next().expect("[ERROR]: Missing reddit client id!").to_owned(),
                       "REDDIT_CLIENT_SECRET" => env_config.reddit_client_secret = line_parts.next().expect("[ERROR]: Missing reddit client secret!").to_owned(),
                       "REDDIT_REDIRECT_URI" => env_config.reddit_redirect_uri = line_parts.next().expect("[ERROR]: Missing reddit redirect uri!").to_owned(),
                       "REDDIT_AUTH_URI" => env_config.reddit_auth_uri = line_parts.next().expect("[ERROR]: Missing reddit auth uri!").to_owned(),
                       "REDDIT_ACCESS_URI" => env_config.reddit_access_uri = line_parts.next().expect("[ERORR] Missing reddit access uri!").to_owned(),
                       "REDDIT_AUTHOR" => env_config.reddit_author = line_parts.next().expect("[ERORR] Missing reddit author").to_owned(),
                       "REDDIT_GET_USER_URI" => env_config.reddit_get_user_uri = line_parts.next().expect("[ERORR] Missing reddit get_user uri !").to_owned(),
                       "RUST_LOG" => match line_parts.next().expect("[ERROR]: No RUST_LOG set at build time.").to_lowercase().as_str() {
                           "off" => env_config.log_level = LevelFilter::Off,
                           "error" => env_config.log_level = LevelFilter::Error,
                           "warn" => env_config.log_level = LevelFilter::Warn,
                           "info" => env_config.log_level = LevelFilter::Info,
                           "debug" => env_config.log_level = LevelFilter::Debug,
                           "trace" => env_config.log_level = LevelFilter::Trace,
                           _ => env_config.log_level = LevelFilter::Info,
                               
                       }
                       _ => {},
                    }
                }

            }
                
        }
        if env_config.is_empty() {
            panic!("[ERROR]: Missing key .env values at build time");
        }
        env_config
    }
}

#[derive(Debug)]
struct RedditLoginError(String);

impl Error for RedditLoginError {}

impl fmt::Display for RedditLoginError {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
      write!(f, "{}", self.0)
  }
}

async fn get_reddit_access_token(auth_code: String, env_config: &EnvConfig) -> Result<TokenResponse, Box<dyn Error>> {
    let client = Client::new();
    let token_request = TokenRequest::new(auth_code, env_config);
    info!("[TOKEN REQUEST]: {:?}", token_request);
    let token_result = client.post(env_config.reddit_access_uri.clone())
        .basic_auth(env_config.reddit_client_id.clone(), Some(env_config.reddit_client_secret.clone()))
        .header("User-Agent", env_config.reddit_author.clone())
        .form(&token_request);
    info!("[ACCESS TOKEN REQUEST]: {:?}", token_result);
    let token_result = token_result.send().await;
    match token_result {
        Err(e) => Err(Box::new(RedditLoginError(e.to_string()))), 
        Ok(response) => {
            if response.status().is_success() {
                match response.json::<TokenResponse>().await {
                    Err(e) => Err(Box::new(RedditLoginError(e.to_string()))),
                    Ok(token_data) => {
                        info!("[SUCCESS]: Obtained reddit access token.");
                        Ok(token_data)
                    }
                }
            } else {
                let error_msg = format!("[ERROR]: Reddit token exchange failed with status: {}", response.status());
                error!("{}", error_msg);
                Err(Box::new(RedditLoginError(error_msg)))
            }
        }
    }
}

async fn get_reddit_user(token_response: TokenResponse, env_config: &EnvConfig) -> Result<UserSession, Box<dyn Error>> {
    let client = Client::new();
    let reddit_user = client.get(env_config.reddit_get_user_uri.clone())
        .bearer_auth(token_response.access_token.clone())
        .header("User-Agent", env_config.reddit_author.clone())
        .send()
        .await?
        .json::<RedditUser>()
        .await?;
    let right_now = Local::now();
    let expiration_time = Duration::seconds(token_response.expires_in as i64);
    let user_session = UserSession {
        reddit_user,
        reddit_access_token: token_response.access_token,
        reddit_refresh_token: token_response.refresh_token,
        expires_at: right_now + expiration_time,
    };
    info!("[INFO]: User session: {:?}", user_session);
    Ok(user_session)
}

async fn auth_middleware(req: ServiceRequest, next: Next<impl MessageBody>) -> Result<ServiceResponse<impl MessageBody>, ActixError> {
    match req.cookie("session") {
        None => error!("[ERROR]: no session cookie found! "),
        Some(cookie) => {
            info!("[SUCCESS]: session cookie: {:?}", cookie)

        },
    };
    next.call(req).await
}

#[get("/protected")]
async fn hello() -> impl Responder {
    info!("[INFO]: Hello world!");
    HttpResponse::Ok().body("Hello world!")
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    info!("[INFO]: echo!");
    HttpResponse::Ok().body(req_body)
}

async fn manual_hello() -> impl Responder {
    info!("[INFO]: manual hello!");
    HttpResponse::Ok().body("Hey there!")
}

#[get("/auth/success")]
async fn auth_success(data: web::Data<AppState>) -> impl Responder {
    HttpResponse::Ok()
}

// // Route to initiate the OAuth flow
#[get("/login/reddit")]
async fn reddit_login(data: web::Data<AppState>) -> impl Responder {
    // Create the Reddit authorization URL
    // client_id={}&response_type=code&state=randomstate&redirect_uri={}&duration=permanent&scope=identity
    let auth_url = format!("{}client_id={}&response_type=code&state=randomstate&redirect_uri={}&duration=permanent&scope=identity",
        data.env_config.reddit_auth_uri,
        data.env_config.reddit_client_id, data.env_config.reddit_redirect_uri
    );
    info!("[AUTH URL]: {}", auth_url);
    // Redirect the user to Reddit's authorization page
    HttpResponse::Found()
        .insert_header((header::LOCATION, auth_url))
        .finish()
}

#[get("/login/reddit/callback")]
async fn reddit_login_callback(query: web::Query<HashMap<String,String>>, data: web::Data<AppState>) -> impl Responder {
    info!("[INFO]: Received callback query: {:?}", query);
    let code = query.get("code");
    let state = query.get("state");
    info!("Code: {:?}, State: {:?}", code, state);
    match code {
        None => HttpResponse::BadRequest().finish(),
        Some(code_value) => {
            // get access token to fetch username
            match get_reddit_access_token(code_value.to_string(), &data.env_config).await {
                Err(_e) => HttpResponse::InternalServerError().finish(),
                Ok(token) => {
                    println!("token: {:?}", token);
                    // get reddit username
                    match get_reddit_user(token, &data.env_config).await {
                        Err(_e) => HttpResponse::InternalServerError().finish(),
                        Ok(session) => {
                            // save user session on backend
                            match data.session_store.save_session(&session.reddit_user.id, &session).await {
                                Err(_e) => HttpResponse::InternalServerError().finish(),
                                Ok(_) => {
                                    // set cookie up
                                    info!("[SUCCESS]: User session successfully created!");
                                    HttpResponse::Ok().finish()
                                }
                            }
                        }
                    }
                },
            }
        } 
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let env_config = EnvConfig::new();
    // setup logging 
    Builder::new()
        .filter_level(env_config.log_level)
        .init();
    let session_store = SessionStore::new("user-sessions")?;
    let app_state = web::Data::new(AppState::new(&env_config, session_store));
    info!("[INFO] Environment config: {:?}", env_config);
    HttpServer::new(move || {
        App::new()
            .app_data(app_state.clone())
            .service(echo)
            .service(reddit_login)
            .service(reddit_login_callback)
            .service(
                web::scope("/protected")
                    .wrap(from_fn(auth_middleware))
                    .service(hello)
            )
            .route("/hey", web::get().to(manual_hello))
    })
    .bind(("127.0.0.1", 8081))?
    .run()
    .await
}
