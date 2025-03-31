use gloo::storage::{LocalStorage, SessionStorage, Storage};

// Token storage key
const AUTH_TOKEN_KEY: &str = "auth_token";

// Get auth token from storage
pub fn get_token() -> Option<String> {
    LocalStorage::get(AUTH_TOKEN_KEY).ok()
}

// Set auth token in storage
pub fn set_token(token: &str) {
    LocalStorage::set(AUTH_TOKEN_KEY, token).expect("Failed to store token");
}

// Remove auth token from storage
pub fn remove_token() {
    LocalStorage::delete(AUTH_TOKEN_KEY);
}

// Check if user is logged in
pub fn is_logged_in() -> bool {
    get_token().is_some()
}

// Generic storage utilities for other data
pub fn store_data<T>(key: &str, data: &T) -> Result<(), String> 
where
    T: serde::Serialize,
{
    LocalStorage::set(key, data).map_err(|e| e.to_string())
}

pub fn get_data<T>(key: &str) -> Option<T> 
where
    T: serde::de::DeserializeOwned,
{
    LocalStorage::get(key).ok()
}

pub fn remove_data(key: &str) {
    LocalStorage::delete(key);
}

// Session storage (cleared when browser is closed)
pub fn store_session_data<T>(key: &str, data: &T) -> Result<(), String> 
where
    T: serde::Serialize,
{
    SessionStorage::set(key, data).map_err(|e| e.to_string())
}

pub fn get_session_data<T>(key: &str) -> Option<T> 
where
    T: serde::de::DeserializeOwned,
{
    SessionStorage::get(key).ok()
}

pub fn remove_session_data(key: &str) {
    SessionStorage::delete(key);
}
