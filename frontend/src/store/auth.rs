use serde::{Deserialize, Serialize};
use yew::prelude::*;
use gloo::storage::{LocalStorage, Storage};

use crate::utils::storage::get_token;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct AuthState {
    pub token: Option<String>,
    pub is_authenticated: bool,
    pub user_id: Option<String>,
}

impl AuthState {
    pub fn new() -> Self {
        // Load token from storage
        let token = get_token();
        let is_authenticated = token.is_some();
        let user_id = None; // Would be set from the JWT in a full implementation
        
        Self {
            token,
            is_authenticated,
            user_id,
        }
    }
    
    pub fn login(&mut self, token: String) {
        self.token = Some(token.clone());
        self.is_authenticated = true;
        
        // Save token to storage
        LocalStorage::set("auth_token", token).expect("Failed to save token");
    }
    
    pub fn logout(&mut self) {
        self.token = None;
        self.is_authenticated = false;
        self.user_id = None;
        
        // Remove token from storage
        LocalStorage::delete("auth_token");
    }
}

// Creates a hook for the auth state
#[hook]
pub fn use_auth() -> UseStateHandle<AuthState> {
    use_state(AuthState::new)
}
