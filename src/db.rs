use sled::{Config, Db};
use std::env;
use std::path::Path;

pub struct AppDb {
    pub conn: Db,
}

impl AppDb {
    pub fn new(conn: Db) -> Self {
        Self { conn }
    }
    
    // Add meal record
    pub fn add_meal(&self, user_id: &str, meal_data: &[u8]) -> Result<(), sled::Error> {
        let tree = self.conn.open_tree(format!("user_meals_{}", user_id))?;
        let timestamp = chrono::Utc::now().timestamp();
        tree.insert(timestamp.to_be_bytes(), meal_data)?;
        Ok(())
    }
    
    // Get meals for a user within a date range
    pub fn get_meals(&self, user_id: &str, start_date: i64, end_date: i64) -> Result<Vec<(i64, Vec<u8>)>, sled::Error> {
        let tree = self.conn.open_tree(format!("user_meals_{}", user_id))?;
        let mut meals = Vec::new();
        
        for result in tree.range(start_date.to_be_bytes()..end_date.to_be_bytes()) {
            let (key, value) = result?;
            let timestamp = i64::from_be_bytes(key.as_ref().try_into().unwrap());
            meals.push((timestamp, value.to_vec()));
        }
        
        Ok(meals)
    }
    
    // Store user phone for OTP verification
    pub fn store_verification(&self, phone: &str, code: &str, expiry: i64) -> Result<(), sled::Error> {
        let verifications = self.conn.open_tree("verifications")?;
        verifications.insert(phone.as_bytes(), format!("{}:{}", code, expiry).as_bytes())?;
        Ok(())
    }
    
    // Verify OTP code
    pub fn verify_code(&self, phone: &str, code: &str) -> Result<bool, sled::Error> {
        let verifications = self.conn.open_tree("verifications")?;
        
        if let Some(stored) = verifications.get(phone.as_bytes())? {
            let stored_str = String::from_utf8_lossy(&stored);
            let parts: Vec<&str> = stored_str.split(':').collect();
            
            if parts.len() == 2 {
                let stored_code = parts[0];
                let expiry = parts[1].parse::<i64>().unwrap_or(0);
                let now = chrono::Utc::now().timestamp();
                
                if stored_code == code && now < expiry {
                    // Remove verification after successful use
                    verifications.remove(phone.as_bytes())?;
                    return Ok(true);
                }
            }
        }
        
        Ok(false)
    }
}

pub fn init_db() -> Result<AppDb, sled::Error> {
    let db_path = env::var("DB_PATH").unwrap_or_else(|_| "ab_macros_db".to_string());
    let path = Path::new(&db_path);
    
    let db = Config::new()
        .path(path)
        .cache_capacity(128 * 1024 * 1024) // 128 MB cache
        .flush_every_ms(Some(1000))
        .open()?;
    
    Ok(AppDb::new(db))
}
