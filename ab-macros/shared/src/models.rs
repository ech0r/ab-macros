// shared/src/models.rs
use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: String,
    pub phone: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<DateTime<Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_login: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Food {
    pub id: String,
    pub name: String,
    pub category: FoodCategory,
    pub nutrients: HashMap<NutrientId, f32>,
    pub serving_size: f32,
    pub serving_unit: ServingUnit,
    pub source: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Meal {
    pub id: String,
    pub user_id: String,
    pub name: Option<String>,
    pub timestamp: DateTime<Utc>,
    pub food_items: Vec<MealItem>,
    pub notes: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MealItem {
    pub food_id: String,
    pub amount: f32, // In serving units
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Nutrient {
    pub id: NutrientId,
    pub name: String,
    pub unit: NutrientUnit,
    pub category: NutrientCategory,
    pub recommended_daily: Option<f32>,
    pub upper_limit: Option<f32>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum NutrientId {
    Protein,
    Fat,
    Carbohydrate,
    Fiber,
    VitaminA,
    VitaminB1,
    VitaminB2,
    VitaminB3,
    VitaminB5,
    VitaminB6,
    VitaminB7,
    VitaminB9,
    VitaminB12,
    VitaminC,
    VitaminD,
    VitaminE,
    VitaminK,
    Calcium,
    Chloride,
    Chromium,
    Copper,
    Fluoride,
    Iodine,
    Iron,
    Magnesium,
    Manganese,
    Molybdenum,
    Phosphorus,
    Potassium,
    Selenium,
    Sodium,
    Zinc,
    Cholesterol,
    Choline,
    // Add more as needed
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum NutrientUnit {
    Gram,
    Milligram,
    Microgram,
    IU,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum NutrientCategory {
    Macronutrient,
    Vitamin,
    Mineral,
    Other,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum FoodCategory {
    Meat,
    Organ,
    Seafood,
    Dairy,
    Fruit,
    Egg,
    Other,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum ServingUnit {
    Gram,
    Ounce,
    Milliliter,
    Tablespoon,
    Teaspoon,
    Cup,
    Piece,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NutrientSummary {
    pub nutrient_id: NutrientId,
    pub consumed: f32,
    pub target: Option<f32>,
    pub unit: NutrientUnit,
    pub percent_of_target: Option<f32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DailyReport {
    pub date: chrono::NaiveDate,
    pub user_id: String,
    pub meals: Vec<Meal>,
    pub nutrient_summary: Vec<NutrientSummary>,
    pub total_calories: f32,
}
