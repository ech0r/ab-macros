// backend/src/config.rs
use serde::Deserialize;
use std::env;
use anyhow::Result;
use dotenv::dotenv;

#[derive(Debug, Clone, Deserialize)]
pub struct Config {
    pub server: ServerConfig,
    pub db: DbConfig,
    pub auth: AuthConfig,
    pub twilio: TwilioConfig,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ServerConfig {
    #[serde(default = "default_port")]
    pub port: u16,
    #[serde(default = "default_host")]
    pub host: String,
    #[serde(default = "default_workers")]
    pub workers: usize,
}

#[derive(Debug, Clone, Deserialize)]
pub struct DbConfig {
    #[serde(default = "default_db_path")]
    pub path: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct AuthConfig {
    pub jwt_secret: String,
    #[serde(default = "default_jwt_expiration")]
    pub jwt_expiration: i64, // In seconds
    #[serde(default = "default_otp_expiration")]
    pub otp_expiration: i64, // In seconds
}

#[derive(Debug, Clone, Deserialize)]
pub struct TwilioConfig {
    #[serde(default = "default_twilio_enabled")]
    pub enabled: bool,
    pub account_sid: String,
    pub auth_token: String,
    pub from_number: String,
    #[serde(default = "default_test_user_id")]
    pub test_user_id: String,
    #[serde(default = "default_test_user_phone")]
    pub test_user_phone: String,
}

fn default_port() -> u16 {
    8080
}

fn default_host() -> String {
    "127.0.0.1".to_string()
}

fn default_workers() -> usize {
    num_cpus::get()
}

fn default_db_path() -> String {
    "./data/db".to_string()
}

fn default_jwt_expiration() -> i64 {
    60 * 60 * 24 * 7 // 7 days
}

fn default_otp_expiration() -> i64 {
    60 * 10 // 10 minutes
}

fn default_twilio_enabled() -> bool {
    true
}

fn default_test_user_id() -> String {
    "test-user-id".to_string()
}

fn default_test_user_phone() -> String {
    "+15555555555".to_string()
}

impl Config {
    pub fn from_env() -> Result<Self> {
        // Load .env file if it exists
        dotenv().ok();
        
        let server = ServerConfig {
            port: env::var("SERVER_PORT")
                .ok()
                .and_then(|s| s.parse().ok())
                .unwrap_or_else(default_port),
            host: env::var("SERVER_HOST").unwrap_or_else(|_| default_host()),
            workers: env::var("SERVER_WORKERS")
                .ok()
                .and_then(|s| s.parse().ok())
                .unwrap_or_else(default_workers),
        };
        
        let db = DbConfig {
            path: env::var("DB_PATH").unwrap_or_else(|_| default_db_path()),
        };
        
        let auth = AuthConfig {
            jwt_secret: env::var("JWT_SECRET")
                .expect("JWT_SECRET must be set"),
            jwt_expiration: env::var("JWT_EXPIRATION")
                .ok()
                .and_then(|s| s.parse().ok())
                .unwrap_or_else(default_jwt_expiration),
            otp_expiration: env::var("OTP_EXPIRATION")
                .ok()
                .and_then(|s| s.parse().ok())
                .unwrap_or_else(default_otp_expiration),
        };
        
        let twilio_enabled = env::var("TWILIO_ENABLED")
            .map(|val| val.to_lowercase() == "true")
            .unwrap_or_else(|_| default_twilio_enabled());
            
        let twilio = if twilio_enabled {
            TwilioConfig {
                enabled: true,
                account_sid: env::var("TWILIO_ACCOUNT_SID")
                    .expect("TWILIO_ACCOUNT_SID must be set when TWILIO_ENABLED=true"),
                auth_token: env::var("TWILIO_AUTH_TOKEN")
                    .expect("TWILIO_AUTH_TOKEN must be set when TWILIO_ENABLED=true"),
                from_number: env::var("TWILIO_FROM_NUMBER")
                    .expect("TWILIO_FROM_NUMBER must be set when TWILIO_ENABLED=true"),
                test_user_id: env::var("TEST_USER_ID").unwrap_or_else(|_| default_test_user_id()),
                test_user_phone: env::var("TEST_USER_PHONE").unwrap_or_else(|_| default_test_user_phone()),
            }
        } else {
            TwilioConfig {
                enabled: false,
                account_sid: env::var("TWILIO_ACCOUNT_SID").unwrap_or_default(),
                auth_token: env::var("TWILIO_AUTH_TOKEN").unwrap_or_default(),
                from_number: env::var("TWILIO_FROM_NUMBER").unwrap_or_default(),
                test_user_id: env::var("TEST_USER_ID").unwrap_or_else(|_| default_test_user_id()),
                test_user_phone: env::var("TEST_USER_PHONE").unwrap_or_else(|_| default_test_user_phone()),
            }
        };
        
        Ok(Config {
            server,
            db,
            auth,
            twilio,
        })
    }
}
