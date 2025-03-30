// backend/src/db/sled.rs
use super::Database;
use ab_macros_shared::{
    User, Food, Meal, Nutrient, DailyReport, NutrientId,
};
use anyhow::{Result, Context, anyhow};
use async_trait::async_trait;
use bincode::{deserialize, serialize};
use chrono::{DateTime, NaiveDate, Utc, TimeZone};
use serde::{Serialize, de::DeserializeOwned};
use std::path::Path;
use tokio::task;
use uuid::Uuid;

pub struct SledDb {
    db: sled::Db,
}

impl SledDb {
    pub fn new<P: AsRef<Path>>(path: P) -> Result<Self> {
        // Create directories if they don't exist
        std::fs::create_dir_all(path.as_ref()).context("Failed to create database directory")?;
        
        let db = sled::open(path)?;
        Ok(Self { db })
    }
    
    // Helper methods for working with Sled
    fn get_tree(&self, name: &str) -> Result<sled::Tree> {
        self.db.open_tree(name).context(format!("Failed to open tree: {}", name))
    }
    
    async fn serialize_and_insert<T: Serialize + Send + Sync>(
        &self,
        tree: &str,
        key: &[u8],
        value: &T,
    ) -> Result<()> {
        let tree = self.get_tree(tree)?;
        let value_bytes = task::spawn_blocking(move || serialize(value))
            .await??;
        
        task::spawn_blocking(move || tree.insert(key, value_bytes))
            .await??;
        
        Ok(())
    }
    
    async fn get_and_deserialize<T: DeserializeOwned + Send>(
        &self,
        tree: &str,
        key: &[u8],
    ) -> Result<Option<T>> {
        let tree = self.get_tree(tree)?;
        
        let result = task::spawn_blocking(move || tree.get(key))
            .await??;
        
        match result {
            Some(bytes) => {
                let value = task::spawn_blocking(move || deserialize(&bytes))
                    .await??;
                Ok(Some(value))
            }
            None => Ok(None),
        }
    }
    
    // Generate a unique ID
    fn generate_id() -> String {
        Uuid::new_v4().to_string()
    }
    
    // Convert date to consistent string format for keys
    fn date_to_string(date: NaiveDate) -> String {
        date.format("%Y-%m-%d").to_string()
    }
    
    // Create compound key for user + date
    fn user_date_key(user_id: &str, date: NaiveDate) -> Vec<u8> {
        format!("{}:{}", user_id, Self::date_to_string(date)).into_bytes()
    }
}

#[async_trait]
impl Database for SledDb {
    // User operations
    async fn create_user(&self, user: &User) -> Result<()> {
        let user_id = user.id.as_bytes();
        let phone = user.phone.as_bytes();
        
        self.serialize_and_insert("users", user_id, user).await?;
        
        // Create a phone -> user_id index
        let phone_tree = self.get_tree("phones")?;
        task::spawn_blocking(move || phone_tree.insert(phone, user_id))
            .await??;
        
        Ok(())
    }
    
    async fn get_user(&self, id: &str) -> Result<Option<User>> {
        self.get_and_deserialize("users", id.as_bytes()).await
    }
    
    async fn get_user_by_phone(&self, phone: &str) -> Result<Option<User>> {
        let phone_tree = self.get_tree("phones")?;
        
        let result = task::spawn_blocking(move || phone_tree.get(phone.as_bytes()))
            .await??;
        
        match result {
            Some(user_id) => {
                let user_id_str = String::from_utf8(user_id.to_vec())?;
                self.get_user(&user_id_str).await
            }
            None => Ok(None),
        }
    }
    
    async fn update_user(&self, user: &User) -> Result<()> {
        self.serialize_and_insert("users", user.id.as_bytes(), user).await
    }
    
    // Authentication operations
    async fn store_otp(&self, phone: &str, otp: &str, expires_at: DateTime<Utc>) -> Result<()> {
        let otp_tree = self.get_tree("otps")?;
        let key = phone.as_bytes();
        let value = serialize(&(otp.to_string(), expires_at))?;
        
        task::spawn_blocking(move || otp_tree.insert(key, value))
            .await??;
        
        Ok(())
    }
    
    async fn verify_otp(&self, phone: &str, otp: &str) -> Result<bool> {
        let otp_tree = self.get_tree("otps")?;
        let key = phone.as_bytes();
        
        let result = task::spawn_blocking(move || otp_tree.get(key))
            .await??;
        
        match result {
            Some(bytes) => {
                let (stored_otp, expires_at): (String, DateTime<Utc>) = deserialize(&bytes)?;
                
                if stored_otp == otp && Utc::now() < expires_at {
                    // Remove the OTP after successful verification
                    let otp_tree = self.get_tree("otps")?;
                    let key = phone.as_bytes();
                    task::spawn_blocking(move || otp_tree.remove(key))
                        .await??;
                    
                    Ok(true)
                } else {
                    Ok(false)
                }
            }
            None => Ok(false),
        }
    }
    
    // Food operations
    async fn create_food(&self, food: &Food) -> Result<()> {
        self.serialize_and_insert("foods", food.id.as_bytes(), food).await
    }
    
    async fn get_food(&self, id: &str) -> Result<Option<Food>> {
        self.get_and_deserialize("foods", id.as_bytes()).await
    }
    
    async fn search_foods(&self, query: &str, limit: usize, offset: usize) -> Result<Vec<Food>> {
        let query = query.to_lowercase();
        let tree = self.get_tree("foods")?;
        
        let foods = task::spawn_blocking(move || {
            let mut results = Vec::new();
            
            for item in tree.iter() {
                if let Ok((_, value)) = item {
                    if let Ok(food) = deserialize::<Food>(&value) {
                        if food.name.to_lowercase().contains(&query) {
                            results.push(food);
                        }
                    }
                }
            }
            
            results
        })
        .await?;
        
        let end = std::cmp::min(offset + limit, foods.len());
        if offset >= foods.len() {
            Ok(Vec::new())
        } else {
            Ok(foods[offset..end].to_vec())
        }
    }
    
    async fn list_foods_by_category(&self, category: &str, limit: usize, offset: usize) -> Result<Vec<Food>> {
        let tree = self.get_tree("foods")?;
        let category = category.to_string();
        
        let foods = task::spawn_blocking(move || {
            let mut results = Vec::new();
            
            for item in tree.iter() {
                if let Ok((_, value)) = item {
                    if let Ok(food) = deserialize::<Food>(&value) {
                        if food.category.to_string() == category {
                            results.push(food);
                        }
                    }
                }
            }
            
            results
        })
        .await?;
        
        let end = std::cmp::min(offset + limit, foods.len());
        if offset >= foods.len() {
            Ok(Vec::new())
        } else {
            Ok(foods[offset..end].to_vec())
        }
    }
    
    // Meal operations
    async fn create_meal(&self, meal: &Meal) -> Result<()> {
        self.serialize_and_insert("meals", meal.id.as_bytes(), meal).await?;
        
        // Create an index by user_id + date for faster queries
        let user_meals_tree = self.get_tree("user_meals")?;
        let date = meal.timestamp.date_naive();
        let key = Self::user_date_key(&meal.user_id, date);
        
        // Get existing meal IDs for this user and date, or create new list
        let meal_ids = if let Ok(Some(ids)) = self.get_and_deserialize::<Vec<String>>("user_meals", &key).await {
            let mut ids = ids;
            ids.push(meal.id.clone());
            ids
        } else {
            vec![meal.id.clone()]
        };
        
        // Store updated meal IDs
        self.serialize_and_insert("user_meals", &key, &meal_ids).await?;
        
        Ok(())
    }
    
    async fn get_meal(&self, id: &str) -> Result<Option<Meal>> {
        self.get_and_deserialize("meals", id.as_bytes()).await
    }
    
    async fn get_user_meals(&self, user_id: &str, limit: usize, offset: usize) -> Result<Vec<Meal>> {
        let meals_tree = self.get_tree("meals")?;
        let user_id = user_id.to_string();
        
        let meals = task::spawn_blocking(move || {
            let mut results = Vec::new();
            
            for item in meals_tree.iter() {
                if let Ok((_, value)) = item {
                    if let Ok(meal) = deserialize::<Meal>(&value) {
                        if meal.user_id == user_id {
                            results.push(meal);
                        }
                    }
                }
            }
            
            // Sort by timestamp, newest first
            results.sort_by(|a, b| b.timestamp.cmp(&a.timestamp));
            results
        })
        .await?;
        
        let end = std::cmp::min(offset + limit, meals.len());
        if offset >= meals.len() {
            Ok(Vec::new())
        } else {
            Ok(meals[offset..end].to_vec())
        }
    }
    
    async fn get_user_meals_by_date_range(
        &self,
        user_id: &str,
        start_date: NaiveDate,
        end_date: NaiveDate,
        limit: usize,
        offset: usize,
    ) -> Result<Vec<Meal>> {
        let mut current_date = start_date;
        let mut all_meal_ids = Vec::new();
        
        // Collect all meal IDs in the date range
        while current_date <= end_date {
            let key = Self::user_date_key(user_id, current_date);
            if let Some(ids) = self.get_and_deserialize::<Vec<String>>("user_meals", &key).await? {
                all_meal_ids.extend(ids);
            }
            current_date = current_date.succ_opt().ok_or_else(|| anyhow!("Date overflow"))?;
        }
        
        // Fetch all the meals
        let mut meals = Vec::new();
        for id in all_meal_ids {
            if let Some(meal) = self.get_meal(&id).await? {
                meals.push(meal);
            }
        }
        
        // Sort by timestamp, newest first
        meals.sort_by(|a, b| b.timestamp.cmp(&a.timestamp));
        
        // Apply pagination
        let end = std::cmp::min(offset + limit, meals.len());
        if offset >= meals.len() {
            Ok(Vec::new())
        } else {
            Ok(meals[offset..end].to_vec())
        }
    }
    
    // Nutrient operations
    async fn create_nutrient(&self, nutrient: &Nutrient) -> Result<()> {
        let id = format!("{:?}", nutrient.id).as_bytes().to_vec();
        self.serialize_and_insert("nutrients", &id, nutrient).await
    }
    
    async fn get_nutrient(&self, id: &str) -> Result<Option<Nutrient>> {
        self.get_and_deserialize("nutrients", id.as_bytes()).await
    }
    
    async fn list_nutrients(&self) -> Result<Vec<Nutrient>> {
        let tree = self.get_tree("nutrients")?;
        
        let nutrients = task::spawn_blocking(move || {
            let mut results = Vec::new();
            
            for item in tree.iter() {
                if let Ok((_, value)) = item {
                    if let Ok(nutrient) = deserialize::<Nutrient>(&value) {
                        results.push(nutrient);
                    }
                }
            }
            
            results
        })
        .await?;
        
        Ok(nutrients)
    }
    
    // Report operations
    async fn create_daily_report(&self, report: &DailyReport) -> Result<()> {
        let key = Self::user_date_key(&report.user_id, report.date);
        self.serialize_and_insert("reports", &key, report).await
    }
    
    async fn get_daily_report(&self, user_id: &str, date: NaiveDate) -> Result<Option<DailyReport>> {
        let key = Self::user_date_key(user_id, date);
        self.get_and_deserialize("reports", &key).await
    }
    
    async fn get_user_reports(
        &self,
        user_id: &str,
        start_date: NaiveDate,
        end_date: NaiveDate,
    ) -> Result<Vec<DailyReport>> {
        let mut current_date = start_date;
        let mut reports = Vec::new();
        
        // Collect all reports in the date range
        while current_date <= end_date {
            let key = Self::user_date_key(user_id, current_date);
            if let Some(report) = self.get_and_deserialize::<DailyReport>("reports", &key).await? {
                reports.push(report);
            }
            current_date = current_date.succ_opt().ok_or_else(|| anyhow!("Date overflow"))?;
        }
        
        // Sort by date, newest first
        reports.sort_by(|a, b| b.date.cmp(&a.date));
        
        Ok(reports)
    }
}
