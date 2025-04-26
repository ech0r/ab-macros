use serde::{Deserialize, Serialize};
use sled::Db;
use std::sync::Arc;
use std::error::Error;
use chrono::{DateTime, Local};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RedditUser {
    pub name: String,
    pub id: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct UserSession {
    pub reddit_user: RedditUser,
    pub reddit_access_token: String,
    pub reddit_refresh_token: Option<String>,
    pub expires_at: DateTime<Local>,
}

#[derive(Clone, Debug)]
pub struct SessionStore {
    pub db: Arc<Db>,
}

impl SessionStore {
    pub fn new(path: &str) -> Result<Self, sled::Error> {
        let db = sled::open(path)?;
        Ok(SessionStore {
            db: Arc::new(db),
        })
    }

    pub async fn save_session(&self, session_id: &str, session: &UserSession) -> Result<(), Box<dyn Error>> {
        let serialized = serde_json::to_vec(session)?;
        self.db.insert(session_id.as_bytes(), serialized)?;
        self.db.flush()?;
        Ok(())
    }

    pub async fn get_session(&self, session_id: &str) -> Result<Option<UserSession>, Box<dyn Error>> {
        if let Some(data) = self.db.get(session_id.as_bytes())? {
            let session: UserSession = serde_json::from_slice(&data)?;
            Ok(Some(session))
        } else {
            Ok(None)
        }
    }

    pub async fn delete_session(&self, session_id: &str) -> Result<(), Box<dyn Error>> {
        self.db.remove(session_id.as_bytes())?;
        self.db.flush()?;
        Ok(())
    }
}
