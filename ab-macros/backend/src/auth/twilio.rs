// backend/src/auth/twilio.rs
use anyhow::{Result, Context};
use reqwest::{Client, StatusCode};
use serde::{Deserialize, Serialize};
use std::time::Duration;
use tracing::{info, error};
use base64::{engine::general_purpose, Engine as _};

#[derive(Debug, Clone)]
pub struct TwilioClient {
    enabled: bool,
    account_sid: String,
    auth_token: String,
    from_number: String,
    test_user_id: String,
    test_user_phone: String,
    http_client: Client,
}

#[derive(Debug, Serialize)]
struct SmsRequest<'a> {
    #[serde(rename = "To")]
    to: &'a str,
    #[serde(rename = "From")]
    from: &'a str,
    #[serde(rename = "Body")]
    body: &'a str,
}

#[derive(Debug, Deserialize)]
struct TwilioResponse {
    sid: String,
    status: String,
}

impl TwilioClient {
    pub fn new(
        account_sid: &str, 
        auth_token: &str, 
        from_number: &str,
        enabled: bool,
        test_user_id: &str,
        test_user_phone: &str,
    ) -> Self {
        let http_client = Client::builder()
            .timeout(Duration::from_secs(10))
            .build()
            .expect("Failed to create HTTP client");
        
        Self {
            enabled,
            account_sid: account_sid.to_string(),
            auth_token: auth_token.to_string(),
            from_number: from_number.to_string(),
            test_user_id: test_user_id.to_string(),
            test_user_phone: test_user_phone.to_string(),
            http_client,
        }
    }
    
    pub fn is_enabled(&self) -> bool {
        self.enabled
    }
    
    pub fn get_test_user_id(&self) -> &str {
        &self.test_user_id
    }
    
    pub fn get_test_user_phone(&self) -> &str {
        &self.test_user_phone
    }
    
    pub async fn send_sms(&self, to: &str, message: &str) -> Result<()> {
        // If Twilio is disabled, just log the message
        if !self.enabled {
            info!("[TWILIO DISABLED] Would send SMS to {}: {}", to, message);
            return Ok(());
        }
        
        // Make sure the phone number has a + prefix
        let to = if to.starts_with('+') {
            to.to_string()
        } else {
            format!("+{}", to)
        };
        
        // Construct the request URL
        let url = format!(
            "https://api.twilio.com/2010-04-01/Accounts/{}/Messages.json",
            self.account_sid
        );
        
        // Create the auth header
        let auth = general_purpose::STANDARD.encode(
            format!("{}:{}", self.account_sid, self.auth_token).as_bytes()
        );
        
        // Create the request body
        let request_body = SmsRequest {
            to: &to,
            from: &self.from_number,
            body: message,
        };
        
        info!("Sending SMS to {}", to);
        
        // Make the request
        let response = self.http_client
            .post(&url)
            .header("Authorization", format!("Basic {}", auth))
            .form(&request_body)
            .send()
            .await
            .context("Failed to send SMS request")?;
        
        match response.status() {
            StatusCode::CREATED | StatusCode::OK => {
                let twilio_response: TwilioResponse = response
                    .json()
                    .await
                    .context("Failed to parse Twilio response")?;
                
                info!("SMS sent successfully, SID: {}, Status: {}", 
                      twilio_response.sid, twilio_response.status);
                
                Ok(())
            }
            status => {
                let error_text = response
                    .text()
                    .await
                    .unwrap_or_else(|_| "Unable to read error response".to_string());
                
                error!("Failed to send SMS: Status: {}, Error: {}", status, error_text);
                
                Err(anyhow::anyhow!(
                    "Twilio API error: Status: {}, Message: {}", 
                    status, error_text
                ))
            }
        }
    }
    
    pub async fn send_otp(&self, to: &str, otp: &str) -> Result<()> {
        let message = format!(
            "Your AB Macros verification code is: {}. This code will expire in 10 minutes.", 
            otp
        );
        
        self.send_sms(to, &message).await
    }
}
