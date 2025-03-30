// backend/src/api/mod.rs
pub mod routes;
pub mod handlers;

use actix_web::{
    error::ResponseError,
    http::StatusCode,
    HttpResponse,
};
use derive_more::Display;
use serde::Serialize;
use std::fmt;

#[derive(Debug, Display)]
pub enum ApiError {
    #[display(fmt = "Bad Request: {}", _0)]
    BadRequest(String),
    
    #[display(fmt = "Unauthorized: {}", _0)]
    Unauthorized(String),
    
    #[display(fmt = "Forbidden: {}", _0)]
    Forbidden(String),
    
    #[display(fmt = "Not Found: {}", _0)]
    NotFound(String),
    
    #[display(fmt = "Conflict: {}", _0)]
    Conflict(String),
    
    #[display(fmt = "Internal Server Error")]
    InternalServerError,
}

impl ResponseError for ApiError {
    fn status_code(&self) -> StatusCode {
        match self {
            ApiError::BadRequest(_) => StatusCode::BAD_REQUEST,
            ApiError::Unauthorized(_) => StatusCode::UNAUTHORIZED,
            ApiError::Forbidden(_) => StatusCode::FORBIDDEN,
            ApiError::NotFound(_) => StatusCode::NOT_FOUND,
            ApiError::Conflict(_) => StatusCode::CONFLICT,
            ApiError::InternalServerError => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
    
    fn error_response(&self) -> HttpResponse {
        let status = self.status_code();
        
        let error_message = match self {
            ApiError::InternalServerError => "Internal Server Error".to_string(),
            _ => self.to_string(),
        };
        
        HttpResponse::build(status).json(ErrorResponse {
            error: error_message,
            status_code: status.as_u16(),
        })
    }
}

impl From<anyhow::Error> for ApiError {
    fn from(err: anyhow::Error) -> Self {
        tracing::error!("Internal error: {:?}", err);
        ApiError::InternalServerError
    }
}

#[derive(Serialize)]
pub struct ErrorResponse {
    error: String,
    status_code: u16,
}

#[derive(Serialize)]
pub struct SuccessResponse<T> {
    pub data: T,
}

pub fn json_success<T: Serialize>(data: T) -> HttpResponse {
    HttpResponse::Ok().json(SuccessResponse { data })
}
