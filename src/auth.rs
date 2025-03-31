use actix_web::{web, HttpResponse, Responder, Error, FromRequest, HttpRequest, dev};
use actix_web::error::{ErrorUnauthorized};
use actix_web::HttpMessage; // Add this import for extensions()
use serde::{Deserialize, Serialize};
use rand::Rng;
use jsonwebtoken::{encode, Header, EncodingKey};
use std::env;
use chrono::{Duration, Utc};
use twilio::{Client, OutboundMessage};
use futures::future::ready;

use crate::db::AppDb;

// Test account constants
const TEST_PHONE: &str = "123";
const TEST_CODE: &str = "123456";

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Claims {
    pub sub: String, // Subject (user phone)
    pub exp: usize,  // Expiration time
    pub iat: usize,  // Issued at
}

// Implement FromRequest for Claims to extract it from request extensions
impl FromRequest for Claims {
    type Error = Error;
    type Future = futures::future::Ready<Result<Self, Self::Error>>;
    
    fn from_request(req: &HttpRequest, _: &mut dev::Payload) -> Self::Future {
        if let Some(claims) = req.extensions().get::<Claims>() {
            ready(Ok(claims.clone()))
        } else {
            ready(Err(ErrorUnauthorized("Not authorized")))
        }
    }
}

// Request structs
#[derive(Debug, Serialize, Deserialize)]
pub struct PhoneRequest {
    pub phone: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VerifyRequest {
    pub phone: String,
    pub code: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TokenResponse {
    pub token: String,
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
    // Check if this is our test account (using raw input to make it easier to test)
    if req.phone == TEST_PHONE {
        log::info!("Using test account - bypassing SMS");
        
        // Store the test code in the database
        let phone = normalize_phone(&req.phone);
        if let Err(e) = db.store_verification(&phone, TEST_CODE, Utc::now().timestamp() + 600) {
            log::error!("Failed to store test verification: {}", e);
            return HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "Failed to store verification"
            }));
        }
        
        return HttpResponse::Ok().json(serde_json::json!({
            "message": "Verification code sent (TEST MODE)"
        }));
    }
    
    // Normalize phone number for regular accounts
    let phone = normalize_phone(&req.phone);
    
    // For non-test accounts, continue with regular flow
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

    // Check if we're in dev mode
    if env::var("DEV_MODE").unwrap_or_default() == "1" {
        log::info!("DEV MODE: Skipping SMS, verification code: {}", code);
        return HttpResponse::Ok().json(serde_json::json!({
            "message": "Verification code sent (DEV MODE)"
        }));
    }
    
    // Try to send SMS via Twilio
    match send_sms(&phone, &code).await {
        Ok(_) => {
            log::info!("SMS sent successfully");
            HttpResponse::Ok().json(serde_json::json!({
                "message": "Verification code sent"
            }))
        },
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
    // Check if this is the test account
    if req.phone == TEST_PHONE && req.code == TEST_CODE {
        log::info!("Test account login successful");
        
        // Generate JWT token
        match create_token(&normalize_phone(TEST_PHONE)) {
            Ok(token) => return HttpResponse::Ok().json(TokenResponse { token }),
            Err(e) => {
                log::error!("Token generation failed: {}", e);
                return HttpResponse::InternalServerError().json(serde_json::json!({
                    "error": "Failed to generate token"
                }));
            }
        }
    }
    
    // Regular verification flow
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
    // Check if this is the test account
    if to == normalize_phone(TEST_PHONE) {
        log::info!("Test account - no SMS needed");
        return Ok(());
    }
    
    // Get Twilio credentials
    let account_sid = match env::var("TWILIO_ACCOUNT_SID") {
        Ok(sid) => sid,
        Err(e) => {
            log::error!("Missing TWILIO_ACCOUNT_SID: {}", e);
            return Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                "TWILIO_ACCOUNT_SID not set"
            )));
        }
    };
    
    let auth_token = match env::var("TWILIO_AUTH_TOKEN") {
        Ok(token) => token,
        Err(e) => {
            log::error!("Missing TWILIO_AUTH_TOKEN: {}", e);
            return Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                "TWILIO_AUTH_TOKEN not set"
            )));
        }
    };
    
    let from_number = match env::var("TWILIO_FROM_NUMBER") {
        Ok(number) => number,
        Err(e) => {
            log::error!("Missing TWILIO_FROM_NUMBER: {}", e);
            return Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                "TWILIO_FROM_NUMBER not set"
            )));
        }
    };
    
    log::info!("Sending SMS to {} with code {}", to, code);
    
    // Create Twilio client and message
    let client = Client::new(&account_sid, &auth_token);
    let message_text = format!("Your AB Macros verification code: {}", code);
    let message = OutboundMessage::new(
        &from_number,
        to,
        &message_text,
    );
    
    // Send the message
    match client.send_message(message).await {
        Ok(_) => {
            log::info!("SMS sent successfully");
            Ok(())
        },
        Err(e) => {
            log::error!("Twilio error: {:?}", e);
            Err(Box::new(e))
        }
    }
}
