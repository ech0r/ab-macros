// backend/src/db/mod.rs
pub mod sled;
pub use self::sled::SledDb;

use ab_macros_shared::{
    User, Food, Meal, Nutrient, DailyReport,
};
use anyhow::Result;
use async_trait::async_trait;
use chrono::{DateTime, NaiveDate, Utc};

// Define the database interface as a trait
#[async_trait]
pub trait Database: Send + Sync + 'static {
    // User operations
    async fn create_user(&self, user: &User) -> Result<()>;
    async fn get_user(&self, id: &str) -> Result<Option<User>>;
    async fn get_user_by_phone(&self, phone: &str) -> Result<Option<User>>;
    async fn update_user(&self, user: &User) -> Result<()>;
    
    // Authentication operations
    async fn store_otp(&self, phone: &str, otp: &str, expires_at: DateTime<Utc>) -> Result<()>;
    async fn verify_otp(&self, phone: &str, otp: &str) -> Result<bool>;
    
    // Food operations
    async fn create_food(&self, food: &Food) -> Result<()>;
    async fn get_food(&self, id: &str) -> Result<Option<Food>>;
    async fn search_foods(&self, query: &str, limit: usize, offset: usize) -> Result<Vec<Food>>;
    async fn list_foods_by_category(&self, category: &str, limit: usize, offset: usize) -> Result<Vec<Food>>;
    
    // Meal operations
    async fn create_meal(&self, meal: &Meal) -> Result<()>;
    async fn get_meal(&self, id: &str) -> Result<Option<Meal>>;
    async fn get_user_meals(&self, user_id: &str, limit: usize, offset: usize) -> Result<Vec<Meal>>;
    async fn get_user_meals_by_date_range(
        &self,
        user_id: &str,
        start_date: NaiveDate,
        end_date: NaiveDate,
        limit: usize,
        offset: usize,
    ) -> Result<Vec<Meal>>;
    
    // Nutrient operations
    async fn create_nutrient(&self, nutrient: &Nutrient) -> Result<()>;
    async fn get_nutrient(&self, id: &str) -> Result<Option<Nutrient>>;
    async fn list_nutrients(&self) -> Result<Vec<Nutrient>>;
    
    // Report operations
    async fn create_daily_report(&self, report: &DailyReport) -> Result<()>;
    async fn get_daily_report(&self, user_id: &str, date: NaiveDate) -> Result<Option<DailyReport>>;
    async fn get_user_reports(
        &self,
        user_id: &str,
        start_date: NaiveDate,
        end_date: NaiveDate,
    ) -> Result<Vec<DailyReport>>;
}
