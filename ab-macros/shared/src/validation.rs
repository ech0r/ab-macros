// shared/src/validation.rs
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ValidationError {
    #[error("Missing required field: {0}")]
    MissingField(String),
    
    #[error("Invalid field format: {0}")]
    InvalidFormat(String),
    
    #[error("Value out of range: {0}")]
    OutOfRange(String),
    
    #[error("Entity not found: {0}")]
    NotFound(String),
    
    #[error("Unauthorized: {0}")]
    Unauthorized(String),
    
    #[error("Internal error: {0}")]
    Internal(String),
}

pub type ValidationResult<T> = Result<T, ValidationError>;

// Helper validation functions
pub fn validate_phone_number(phone: &str) -> ValidationResult<()> {
    if phone.len() < 10 || phone.len() > 15 {
        return Err(ValidationError::InvalidFormat("Phone number must be between 10 and 15 digits".into()));
    }
    
    if !phone.chars().all(|c| c.is_ascii_digit() || c == '+') {
        return Err(ValidationError::InvalidFormat("Phone number can only contain digits and '+' symbol".into()));
    }
    
    Ok(())
}

pub fn validate_amount(amount: f32) -> ValidationResult<()> {
    if amount <= 0.0 {
        return Err(ValidationError::OutOfRange("Amount must be positive".into()));
    }
    
    Ok(())
}
