use actix_web::{web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use chrono::{Duration, Utc, DateTime};
use std::collections::HashMap;
use uuid::Uuid;

use crate::db::AppDb;
use crate::models::{FoodItem, Meal, MealItem, NutrientSummary, NutrientTargets, UserProfile};
use crate::auth::Claims;

// Configure API routes
pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("")
            .route("/foods", web::get().to(get_foods))
            .route("/meals", web::get().to(get_meals))
            .route("/meals", web::post().to(add_meal))
            .route("/meals/{id}", web::delete().to(delete_meal))
            .route("/summary", web::get().to(get_nutrient_summary))
            .route("/targets", web::get().to(get_targets))
            .route("/targets", web::post().to(update_targets))
            .route("/profile", web::get().to(get_profile))
    );
}

// Request models
#[derive(Debug, Deserialize)]
struct TimeRangeQuery {
    start_date: Option<DateTime<Utc>>,
    end_date: Option<DateTime<Utc>>,
}

#[derive(Debug, Deserialize)]
struct AddMealRequest {
    name: String,
    timestamp: Option<DateTime<Utc>>,
    items: Vec<MealItem>,
    notes: Option<String>,
}

// Get food database
async fn get_foods() -> impl Responder {
    // In a real application, this would load from the database
    // For now, returning a small sample of foods
    let foods = vec![
        FoodItem {
            id: "beef-ribeye".into(),
            name: "Beef Ribeye Steak".into(),
            category: crate::models::FoodCategory::Meat,
            serving_size: 100.0,
            serving_unit: "g".into(),
            macros: crate::models::Macronutrients {
                calories: 291.0,
                protein: 24.0,
                fat: 22.0,
                carbs: 0.0,
                fiber: 0.0,
                sugar: 0.0,
            },
            micros: crate::models::Micronutrients {
                iron: 2.1,
                zinc: 4.6,
                vitamin_b12: 2.5,
                ..Default::default()
            },
        },
        FoodItem {
            id: "eggs".into(),
            name: "Eggs (Whole)".into(),
            category: crate::models::FoodCategory::Eggs,
            serving_size: 50.0,
            serving_unit: "g".into(),
            macros: crate::models::Macronutrients {
                calories: 72.0,
                protein: 6.3,
                fat: 5.0,
                carbs: 0.4,
                fiber: 0.0,
                sugar: 0.4,
            },
            micros: crate::models::Micronutrients {
                vitamin_a: 98.0,
                vitamin_b12: 0.6,
                choline: 147.0,
                ..Default::default()
            },
        },
        // Add more foods as needed
    ];

    HttpResponse::Ok().json(foods)
}

// Get user's meals for a date range
async fn get_meals(
    db: web::Data<AppDb>,
    claims: web::ReqData<Claims>,
    query: web::Query<TimeRangeQuery>,
) -> impl Responder {
    let user_id = &claims.sub;
    
    // Default to past 7 days if no date range specified
    let end_date = query.end_date.unwrap_or_else(|| Utc::now());
    let start_date = query.start_date.unwrap_or_else(|| end_date - Duration::days(7));
    
    // Fetch meals from database
    match db.get_meals(user_id, start_date.timestamp(), end_date.timestamp()) {
        Ok(meal_data) => {
            let meals: Vec<Meal> = meal_data
                .iter()
                .filter_map(|(timestamp, data)| {
                    let timestamp = *timestamp;
                    serde_json::from_slice::<Meal>(data).ok()
                })
                .collect();
            
            HttpResponse::Ok().json(meals)
        },
        Err(e) => {
            log::error!("Failed to fetch meals: {}", e);
            HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "Failed to fetch meals"
            }))
        }
    }
}

// Add a new meal
async fn add_meal(
    db: web::Data<AppDb>,
    claims: web::ReqData<Claims>,
    req: web::Json<AddMealRequest>,
) -> impl Responder {
    let user_id = claims.sub.clone();
    
    let meal = Meal {
        id: Uuid::new_v4().to_string(),
        user_id,
        name: req.name.clone(),
        timestamp: req.timestamp.unwrap_or_else(Utc::now),
        items: req.items.clone(),
        notes: req.notes.clone(),
    };
    
    // Serialize and save meal
    match serde_json::to_vec(&meal) {
        Ok(meal_data) => {
            if let Err(e) = db.add_meal(&meal.user_id, &meal_data) {
                log::error!("Failed to save meal: {}", e);
                return HttpResponse::InternalServerError().json(serde_json::json!({
                    "error": "Failed to save meal"
                }));
            }
            
            HttpResponse::Created().json(meal)
        },
        Err(e) => {
            log::error!("Failed to serialize meal: {}", e);
            HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "Failed to process meal data"
            }))
        }
    }
}

// Delete a meal
async fn delete_meal(
    _db: web::Data<AppDb>,
    _claims: web::ReqData<Claims>,
    _path: web::Path<String>,
) -> impl Responder {
    // To be implemented
    HttpResponse::NotImplemented().json(serde_json::json!({
        "message": "Delete meal not implemented yet"
    }))
}

// Get nutrient summary for a time period
async fn get_nutrient_summary(
    _db: web::Data<AppDb>,
    _claims: web::ReqData<Claims>,
    _query: web::Query<TimeRangeQuery>,
) -> impl Responder {
    // To be implemented
    HttpResponse::NotImplemented().json(serde_json::json!({
        "message": "Nutrient summary not implemented yet"
    }))
}

// Get user's nutrient targets
async fn get_targets(
    _db: web::Data<AppDb>,
    _claims: web::ReqData<Claims>,
) -> impl Responder {
    // To be implemented
    HttpResponse::NotImplemented().json(serde_json::json!({
        "message": "Get targets not implemented yet"
    }))
}

// Update user's nutrient targets
async fn update_targets(
    _db: web::Data<AppDb>,
    _claims: web::ReqData<Claims>,
    _targets: web::Json<NutrientTargets>,
) -> impl Responder {
    // To be implemented
    HttpResponse::NotImplemented().json(serde_json::json!({
        "message": "Update targets not implemented yet"
    }))
}

// Get user profile
async fn get_profile(
    _db: web::Data<AppDb>,
    claims: web::ReqData<Claims>,
) -> impl Responder {
    // Simple profile
    let profile = UserProfile {
        id: claims.sub.clone(),
        phone: claims.sub.clone(),
        created_at: Utc::now(),
        targets: None,
    };
    
    HttpResponse::Ok().json(profile)
}
