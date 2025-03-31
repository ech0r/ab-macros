use gloo_net::http::Request;
use serde::{de::DeserializeOwned, Deserialize, Serialize};

use crate::models::{FoodItem, Meal, NutrientSummary, NutrientTargets, UserProfile};
use crate::utils::storage::get_token;

const API_BASE: &str = "/api";

// API error type
#[derive(Debug, Clone)]
pub enum ApiError {
    NetworkError(String),
    DeserializationError(String),
    ApiError { status: u16, message: String },
    Unauthorized,
    NotFound,
}

impl From<gloo_net::Error> for ApiError {
    fn from(err: gloo_net::Error) -> Self {
        ApiError::NetworkError(err.to_string())
    }
}

impl From<serde_json::Error> for ApiError {
    fn from(err: serde_json::Error) -> Self {
        ApiError::DeserializationError(err.to_string())
    }
}

// Helper function to include auth token in requests
fn with_auth(request: Request) -> Request {
    if let Some(token) = get_token() {
        request.header("Authorization", &format!("Bearer {}", token))
    } else {
        request
    }
}

// Generic request function
async fn request<T, R>(method: &str, endpoint: &str, body: Option<&T>) -> Result<R, ApiError>
where
    T: Serialize + ?Sized,
    R: DeserializeOwned,
{
    let url = format!("{}{}", API_BASE, endpoint);
    
    let mut request = match method {
        "GET" => with_auth(Request::get(&url)),
        "POST" => with_auth(Request::post(&url)),
        "PUT" => with_auth(Request::put(&url)),
        "DELETE" => with_auth(Request::delete(&url)),
        _ => return Err(ApiError::NetworkError("Invalid HTTP method".into())),
    };
    
    // Add JSON body if provided
    if let Some(data) = body {
        request = request
            .header("Content-Type", "application/json")
            .json(data)?;
    }
    
    // Execute request
    let response = request.send().await?;
    
    match response.status() {
        200 | 201 => {
            let data = response.json::<R>().await?;
            Ok(data)
        }
        401 => Err(ApiError::Unauthorized),
        404 => Err(ApiError::NotFound),
        status => {
            let error_text = response.text().await.unwrap_or_else(|_| "Unknown error".into());
            Err(ApiError::ApiError {
                status,
                message: error_text,
            })
        }
    }
}

// Authentication API
pub async fn send_verification_code(phone: &str) -> Result<(), ApiError> {
    #[derive(Serialize)]
    struct PhoneRequest {
        phone: String,
    }
    
    let req = PhoneRequest {
        phone: phone.to_string(),
    };
    
    request::<_, serde_json::Value>("POST", "/auth/send-code", Some(&req)).await?;
    
    Ok(())
}

pub async fn verify_code(phone: &str, code: &str) -> Result<String, ApiError> {
    #[derive(Serialize, Deserialize)]
    struct VerifyRequest {
        phone: String,
        code: String,
    }
    
    #[derive(Serialize, Deserialize)]
    struct TokenResponse {
        token: String,
    }
    
    let req = VerifyRequest {
        phone: phone.to_string(),
        code: code.to_string(),
    };
    
    let response: TokenResponse = request("POST", "/auth/verify", Some(&req)).await?;
    
    Ok(response.token)
}

// Food API
pub async fn get_foods() -> Result<Vec<FoodItem>, ApiError> {
    request::<(), Vec<FoodItem>>("GET", "/foods", None).await
}

// Meal API
pub async fn get_meals(start_date: Option<&str>, end_date: Option<&str>) -> Result<Vec<Meal>, ApiError> {
    let mut endpoint = "/meals".to_string();
    
    // Add query parameters if provided
    if start_date.is_some() || end_date.is_some() {
        endpoint.push('?');
        
        if let Some(start) = start_date {
            endpoint.push_str(&format!("start_date={}", start));
        }
        
        if let Some(end) = end_date {
            if start_date.is_some() {
                endpoint.push('&');
            }
            endpoint.push_str(&format!("end_date={}", end));
        }
    }
    
    request::<(), Vec<Meal>>("GET", &endpoint, None).await
}

pub async fn add_meal(meal: &Meal) -> Result<Meal, ApiError> {
    request("POST", "/meals", Some(meal)).await
}

pub async fn delete_meal(id: &str) -> Result<(), ApiError> {
    request::<(), serde_json::Value>("DELETE", &format!("/meals/{}", id), None).await?;
    Ok(())
}

// Nutrient summary API
pub async fn get_nutrient_summary(
    start_date: Option<&str>,
    end_date: Option<&str>,
) -> Result<NutrientSummary, ApiError> {
    let mut endpoint = "/summary".to_string();
    
    // Add query parameters if provided
    if start_date.is_some() || end_date.is_some() {
        endpoint.push('?');
        
        if let Some(start) = start_date {
            endpoint.push_str(&format!("start_date={}", start));
        }
        
        if let Some(end) = end_date {
            if start_date.is_some() {
                endpoint.push('&');
            }
            endpoint.push_str(&format!("end_date={}", end));
        }
    }
    
    request::<(), NutrientSummary>("GET", &endpoint, None).await
}

// Targets API
pub async fn get_targets() -> Result<NutrientTargets, ApiError> {
    request::<(), NutrientTargets>("GET", "/targets", None).await
}

pub async fn update_targets(targets: &NutrientTargets) -> Result<NutrientTargets, ApiError> {
    request("POST", "/targets", Some(targets)).await
}

// Profile API
pub async fn get_profile() -> Result<UserProfile, ApiError> {
    request::<(), UserProfile>("GET", "/profile", None).await
}
