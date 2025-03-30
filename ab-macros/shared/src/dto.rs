// shared/src/dto.rs
use serde::{Deserialize, Serialize};
use validator::Validate;
use chrono::{DateTime, Utc, NaiveDate};
use crate::models::{FoodCategory, ServingUnit, MealItem, NutrientSummary};

// Authentication DTOs
#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct PhoneLoginRequest {
    #[validate(length(min = 10, max = 15))]
    pub phone: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PhoneLoginResponse {
    pub success: bool,
    pub message: String,
}

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct VerifyOtpRequest {
    #[validate(length(min = 10, max = 15))]
    pub phone: String,
    #[validate(length(min = 4, max = 8))]
    pub otp: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AuthResponse {
    pub token: String,
    pub expires_at: DateTime<Utc>,
    pub user_id: String,
}

// Food DTOs
#[derive(Debug, Serialize, Deserialize)]
pub struct FoodListRequest {
    pub category: Option<FoodCategory>,
    pub search: Option<String>,
    pub page: Option<usize>,
    pub per_page: Option<usize>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FoodListResponse {
    pub foods: Vec<FoodDto>,
    pub total: usize,
    pub page: usize,
    pub per_page: usize,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FoodDto {
    pub id: String,
    pub name: String,
    pub category: FoodCategory,
    pub serving_size: f32,
    pub serving_unit: ServingUnit,
    pub protein: f32,
    pub fat: f32,
    pub carbs: f32,
    pub calories: f32,
}

// Meal DTOs
#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct CreateMealRequest {
    pub name: Option<String>,
    pub timestamp: Option<DateTime<Utc>>,
    #[validate]
    pub food_items: Vec<MealItem>,
    pub notes: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MealResponse {
    pub id: String,
    pub name: Option<String>,
    pub timestamp: DateTime<Utc>,
    pub food_items: Vec<MealItemDto>,
    pub notes: Option<String>,
    pub total_calories: f32,
    pub total_protein: f32,
    pub total_fat: f32,
    pub total_carbs: f32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MealItemDto {
    pub food_id: String,
    pub food_name: String,
    pub amount: f32,
    pub serving_unit: ServingUnit,
    pub calories: f32,
    pub protein: f32,
    pub fat: f32,
    pub carbs: f32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MealListRequest {
    pub start_date: Option<NaiveDate>,
    pub end_date: Option<NaiveDate>,
    pub page: Option<usize>,
    pub per_page: Option<usize>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MealListResponse {
    pub meals: Vec<MealResponse>,
    pub total: usize,
    pub page: usize,
    pub per_page: usize,
}

// Report DTOs
#[derive(Debug, Serialize, Deserialize)]
pub struct DailyReportRequest {
    pub date: NaiveDate,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DailyReportResponse {
    pub date: NaiveDate,
    pub meals: Vec<MealResponse>,
    pub nutrient_summary: Vec<NutrientSummary>,
    pub total_calories: f32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WeeklyReportRequest {
    pub start_date: NaiveDate,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WeeklyReportResponse {
    pub start_date: NaiveDate,
    pub end_date: NaiveDate,
    pub daily_reports: Vec<DailyReportResponse>,
    pub nutrient_summary: Vec<NutrientSummary>,
    pub average_calories: f32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MonthlyReportRequest {
    pub year: i32,
    pub month: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MonthlyReportResponse {
    pub year: i32,
    pub month: u32,
    pub daily_reports: Vec<DailyReportResponse>,
    pub nutrient_summary: Vec<NutrientSummary>,
    pub average_calories: f32,
}

// Error Response
#[derive(Debug, Serialize, Deserialize)]
pub struct ErrorResponse {
    pub error: String,
    pub status_code: u16,
}
