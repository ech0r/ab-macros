// backend/src/auth/mod.rs
pub mod middleware;
pub mod twilio;

use anyhow::Result;
use chrono::{DateTime, Duration, Utc};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use rand::Rng;
use serde::{Deserialize, Serialize};
use crate::config::Config;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,        // Subject (user ID)
    pub phone: String,      // User's phone number
    pub exp: usize,         // Expiration time (as UTC timestamp)
    pub iat: usize,         // Issued at (as UTC timestamp)
}

pub fn generate_jwt(
    user_id: &str, 
    phone: &str, 
    secret: &str, 
    expiration_seconds: i64
) -> Result<String> {
    let now = Utc::now();
    let expiration = now + Duration::seconds(expiration_seconds);
    
    let claims = Claims {
        sub: user_id.to_owned(),
        phone: phone.to_owned(),
        iat: now.timestamp() as usize,
        exp: expiration.timestamp() as usize,
    };
    
    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_bytes()),
    )?;
    
    Ok(token)
}

pub fn verify_jwt(token: &str, secret: &str) -> Result<Claims> {
    let token = decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret.as_bytes()),
        &Validation::default(),
    )?;
    
    Ok(token.claims)
}

pub fn generate_otp() -> String {
    let mut rng = rand::thread_rng();
    let otp: u32 = rng.gen_range(100_000..999_999);
    otp.to_string()
}

// Calculate when an OTP should expire
pub fn calculate_otp_expiry(config: &Config) -> DateTime<Utc> {
    Utc::now() + Duration::seconds(config.auth.otp_expiration)
}
