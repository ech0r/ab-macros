use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

// Food categories for an animal-based diet
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum FoodCategory {
    Meat,
    Organ,
    Fish,
    Eggs,
    Dairy,
    Fruit,
    Honey,
    Other,
}

// Represent a single food item
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FoodItem {
    pub id: String,
    pub name: String,
    pub category: FoodCategory,
    pub serving_size: f32,
    pub serving_unit: String,
    pub macros: Macronutrients,
    pub micros: Micronutrients,
}

// Macronutrients data
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Macronutrients {
    pub calories: f32,
    pub protein: f32,
    pub fat: f32,
    pub carbs: f32,
    pub fiber: f32,
    pub sugar: f32,
}

// Micronutrients data - focusing on animal-based diet priorities
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Micronutrients {
    // Vitamins
    pub vitamin_a: f32,
    pub vitamin_b1: f32,
    pub vitamin_b2: f32,
    pub vitamin_b3: f32,
    pub vitamin_b5: f32,
    pub vitamin_b6: f32,
    pub vitamin_b9: f32,
    pub vitamin_b12: f32,
    pub vitamin_c: f32,
    pub vitamin_d: f32,
    pub vitamin_e: f32,
    pub vitamin_k: f32,
    
    // Minerals
    pub calcium: f32,
    pub copper: f32,
    pub iron: f32,
    pub magnesium: f32,
    pub manganese: f32,
    pub phosphorus: f32,
    pub potassium: f32,
    pub selenium: f32,
    pub sodium: f32,
    pub zinc: f32,
    
    // Other
    pub cholesterol: f32,
    pub choline: f32,
    pub dha: f32,
    pub epa: f32,
}

// A single meal record
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Meal {
    pub id: String,
    pub user_id: String,
    pub name: String,
    pub timestamp: DateTime<Utc>,
    pub items: Vec<MealItem>,
    pub notes: Option<String>,
}

// Food item within a meal
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MealItem {
    pub food_id: String,
    pub amount: f32, // Quantity in the serving unit of the food
}

// Daily nutrient targets
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NutrientTargets {
    pub user_id: String,
    pub macros: MacroTargets,
    pub micros: MicroTargets,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MacroTargets {
    pub calories: Range,
    pub protein: Range,
    pub fat: Range,
    pub carbs: Range,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MicroTargets {
    pub vitamin_a: Range,
    pub vitamin_b12: Range,
    pub vitamin_c: Range,
    pub vitamin_d: Range,
    pub vitamin_k: Range,
    pub calcium: Range,
    pub iron: Range,
    pub magnesium: Range,
    pub potassium: Range,
    pub sodium: Range,
    pub zinc: Range,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Range {
    pub min: f32,
    pub max: f32,
}

// User profile
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserProfile {
    pub id: String,
    pub phone: String,
    pub created_at: DateTime<Utc>,
    pub targets: Option<NutrientTargets>,
}

// Nutrient summary for reporting
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NutrientSummary {
    pub start_date: DateTime<Utc>,
    pub end_date: DateTime<Utc>,
    pub days_count: u32,
    pub macros: Macronutrients,
    pub micros: Micronutrients,
    pub targets: Option<NutrientTargets>,
    pub daily_breakdown: Vec<DailyNutrients>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DailyNutrients {
    pub date: DateTime<Utc>,
    pub macros: Macronutrients,
    pub micros: Micronutrients,
}
