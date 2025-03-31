use actix_web::{web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use rand::Rng;
use jsonwebtoken::{encode, Header, EncodingKey};
use std::env;
use chrono::{Duration, Utc};
use twilio::{Client, OutboundMessage};

use crate::db::AppDb;

#[derive(Debug, Serialize, Deserialize)]
pub struct PhoneRequest {
    phone: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VerifyRequest {
    phone: String,
    code: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TokenResponse {
    token: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String, // Subject (user phone)
    pub exp: usize,  // Expiration time
    pub iat: usize,  // Issued at
}

// Configure auth routes
pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/auth")
            .route("/send-code", web::post().to(send_code))
            .route("/verify", web::post().to(verify_code)),
    );
}

// Generate a 6-digit OTP code
fn generate_code() -> String {
    let mut rng = rand::thread_rng();
    format!("{:06}", rng.gen_range(0..1000000))
}

// Send verification code via Twilio SMS
async fn send_code(
    db: web::Data<AppDb>,
    req: web::Json<PhoneRequest>,
) -> impl Responder {
    // Normalize phone number
    let phone = normalize_phone(&req.phone);
    
    // Generate 6-digit code
    let code = generate_code();
    
    // Store code in database with 10-minute expiry
    let expiry = Utc::now()
        .checked_add_signed(Duration::minutes(10))
        .unwrap()
        .timestamp();
    
    if let Err(e) = db.store_verification(&phone, &code, expiry) {
        log::error!("Failed to store verification: {}", e);
        return HttpResponse::InternalServerError().json(serde_json::json!({
            "error": "Failed to store verification"
        }));
    }
    
    // Send SMS using Twilio
    match send_sms(&phone, &code).await {
        Ok(_) => HttpResponse::Ok().json(serde_json::json!({
            "message": "Verification code sent"
        })),
        Err(e) => {
            log::error!("Failed to send SMS: {}", e);
            HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "Failed to send verification code"
            }))
        }
    }
}

// Verify the code and issue a token
async fn verify_code(
    db: web::Data<AppDb>,
    req: web::Json<VerifyRequest>,
) -> impl Responder {
    let phone = normalize_phone(&req.phone);
    
    match db.verify_code(&phone, &req.code) {
        Ok(true) => {
            // Generate JWT token
            match create_token(&phone) {
                Ok(token) => HttpResponse::Ok().json(TokenResponse { token }),
                Err(e) => {
                    log::error!("Token generation failed: {}", e);
                    HttpResponse::InternalServerError().json(serde_json::json!({
                        "error": "Failed to generate token"
                    }))
                }
            }
        },
        Ok(false) => HttpResponse::BadRequest().json(serde_json::json!({
            "error": "Invalid or expired verification code"
        })),
        Err(e) => {
            log::error!("Verification error: {}", e);
            HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "Verification failed"
            }))
        }
    }
}

// Create JWT token
fn create_token(phone: &str) -> Result<String, jsonwebtoken::errors::Error> {
    let jwt_secret = env::var("JWT_SECRET").expect("JWT_SECRET must be set");
    
    let now = Utc::now();
    let iat = now.timestamp() as usize;
    let exp = (now + Duration::days(7)).timestamp() as usize; // 7 day expiry
    
    let claims = Claims {
        sub: phone.to_string(),
        exp,
        iat,
    };
    
    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(jwt_secret.as_bytes()),
    )
}

// Normalize phone number to E.164 format
fn normalize_phone(phone: &str) -> String {
    // Basic normalization - in production, you'd want more robust validation
    let digits: String = phone.chars().filter(|c| c.is_digit(10)).collect();
    
    if digits.starts_with("1") && digits.len() == 11 {
        format!("+{}", digits)
    } else if !digits.starts_with("1") && digits.len() == 10 {
        format!("+1{}", digits)
    } else {
        format!("+{}", digits)
    }
}

// Send SMS via Twilio
async fn send_sms(to: &str, code: &str) -> Result<(), Box<dyn std::error::Error>> {
    let account_sid = env::var("TWILIO_ACCOUNT_SID").expect("TWILIO_ACCOUNT_SID must be set");
    let auth_token = env::var("TWILIO_AUTH_TOKEN").expect("TWILIO_AUTH_TOKEN must be set");
    let from_number = env::var("TWILIO_FROM_NUMBER").expect("TWILIO_FROM_NUMBER must be set");
    
    let client = Client::new(&account_sid, &auth_token);
    let message = OutboundMessage::new(
        &from_number,
        to,
        &format!("Your AB Macros verification code: {}", code),
    );
    
    client.send_message(message).await?;
    Ok(())
}
