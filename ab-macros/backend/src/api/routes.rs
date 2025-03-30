// backend/src/api/routes.rs
use crate::{
    api::handlers::{
        auth::{login, verify_otp},
        foods::{get_food, list_foods, search_foods},
        meals::{add_meal, get_meal, list_meals},
        reports::{get_daily_report, get_monthly_report, get_weekly_report},
        users::get_current_user,
    },
    auth::middleware::JwtAuth,
};
use actix_web::web::{self, ServiceConfig};

pub fn configure_routes(cfg: &mut ServiceConfig) {
    // API version path
    let api = web::scope("/api/v1");
    
    // Configure routes
    cfg.service(
        api
            // Public routes (no auth required)
            .service(
                web::scope("/auth")
                    .route("/login", web::post().to(login))
                    .route("/verify", web::post().to(verify_otp))
            )
            // Protected routes (auth required)
            .service(
                web::scope("")
                    .wrap(JwtAuth)
                    // User routes
                    .service(
                        web::scope("/users")
                            .route("/me", web::get().to(get_current_user))
                    )
                    // Food routes
                    .service(
                        web::scope("/foods")
                            .route("", web::get().to(list_foods))
                            .route("/search", web::get().to(search_foods))
                            .route("/{id}", web::get().to(get_food))
                    )
                    // Meal routes
                    .service(
                        web::scope("/meals")
                            .route("", web::post().to(add_meal))
                            .route("", web::get().to(list_meals))
                            .route("/{id}", web::get().to(get_meal))
                    )
                    // Report routes
                    .service(
                        web::scope("/reports")
                            .route("/daily", web::get().to(get_daily_report))
                            .route("/weekly", web::get().to(get_weekly_report))
                            .route("/monthly", web::get().to(get_monthly_report))
                    )
            )
    );
}
