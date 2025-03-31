pub mod home;
pub mod login;
pub mod meal_entry;
pub mod meal_list;
pub mod nutrient_report;
pub mod profile;
pub mod not_found;

// Re-export for easier imports
pub use home::HomePage;
pub use login::LoginPage;
pub use meal_entry::MealEntryPage;
pub use meal_list::MealListPage;
pub use nutrient_report::NutrientReportPage;
pub use profile::ProfilePage;
pub use not_found::NotFoundPage;
