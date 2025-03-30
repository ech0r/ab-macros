#!/bin/bash

# Create root directories
mkdir -p ab-macros/{backend,frontend,shared}/src
mkdir -p ab-macros/migrations

# Create backend structure
mkdir -p ab-macros/backend/src/{api,auth,db,models,services,utils}
touch ab-macros/backend/src/main.rs
touch ab-macros/backend/src/lib.rs
touch ab-macros/backend/src/config.rs
touch ab-macros/backend/src/server.rs
touch ab-macros/backend/src/api/mod.rs
touch ab-macros/backend/src/api/routes.rs
touch ab-macros/backend/src/api/handlers.rs
touch ab-macros/backend/src/auth/mod.rs
touch ab-macros/backend/src/auth/twilio.rs
touch ab-macros/backend/src/auth/middleware.rs
touch ab-macros/backend/src/db/mod.rs
touch ab-macros/backend/src/db/sled.rs
touch ab-macros/backend/src/models/mod.rs
touch ab-macros/backend/src/models/user.rs
touch ab-macros/backend/src/models/food.rs
touch ab-macros/backend/src/models/meal.rs
touch ab-macros/backend/src/models/nutrient.rs
touch ab-macros/backend/src/services/mod.rs
touch ab-macros/backend/src/services/meal_service.rs
touch ab-macros/backend/src/services/user_service.rs
touch ab-macros/backend/src/services/nutrient_service.rs
touch ab-macros/backend/src/utils/mod.rs
touch ab-macros/backend/Cargo.toml

# Create frontend structure
mkdir -p ab-macros/frontend/src/{components,pages,services,styles,utils}
mkdir -p ab-macros/frontend/src/components/{auth,food,meal,nutrient,report,ui}
mkdir -p ab-macros/frontend/src/pages/{auth,dashboard,food,meal,report,settings}
mkdir -p ab-macros/frontend/static/{css,js,assets,fonts}
touch ab-macros/frontend/src/main.rs
touch ab-macros/frontend/src/app.rs
touch ab-macros/frontend/src/router.rs
touch ab-macros/frontend/src/components/mod.rs
touch ab-macros/frontend/src/components/auth/mod.rs
touch ab-macros/frontend/src/components/auth/login.rs
touch ab-macros/frontend/src/components/auth/otp.rs
touch ab-macros/frontend/src/components/food/mod.rs
touch ab-macros/frontend/src/components/food/food_item.rs
touch ab-macros/frontend/src/components/food/food_list.rs
touch ab-macros/frontend/src/components/meal/mod.rs
touch ab-macros/frontend/src/components/meal/meal_form.rs
touch ab-macros/frontend/src/components/meal/meal_list.rs
touch ab-macros/frontend/src/components/nutrient/mod.rs
touch ab-macros/frontend/src/components/nutrient/nutrient_bar.rs
touch ab-macros/frontend/src/components/nutrient/nutrient_summary.rs
touch ab-macros/frontend/src/components/report/mod.rs
touch ab-macros/frontend/src/components/report/daily_report.rs
touch ab-macros/frontend/src/components/report/weekly_report.rs
touch ab-macros/frontend/src/components/report/monthly_report.rs
touch ab-macros/frontend/src/components/ui/mod.rs
touch ab-macros/frontend/src/components/ui/button.rs
touch ab-macros/frontend/src/components/ui/input.rs
touch ab-macros/frontend/src/components/ui/modal.rs
touch ab-macros/frontend/src/pages/mod.rs
touch ab-macros/frontend/src/pages/auth/mod.rs
touch ab-macros/frontend/src/pages/auth/login_page.rs
touch ab-macros/frontend/src/pages/dashboard/mod.rs
touch ab-macros/frontend/src/pages/dashboard/dashboard_page.rs
touch ab-macros/frontend/src/pages/food/mod.rs
touch ab-macros/frontend/src/pages/food/food_page.rs
touch ab-macros/frontend/src/pages/meal/mod.rs
touch ab-macros/frontend/src/pages/meal/add_meal_page.rs
touch ab-macros/frontend/src/pages/meal/meal_history_page.rs
touch ab-macros/frontend/src/pages/report/mod.rs
touch ab-macros/frontend/src/pages/report/report_page.rs
touch ab-macros/frontend/src/pages/settings/mod.rs
touch ab-macros/frontend/src/pages/settings/settings_page.rs
touch ab-macros/frontend/src/services/mod.rs
touch ab-macros/frontend/src/services/api.rs
touch ab-macros/frontend/src/services/auth.rs
touch ab-macros/frontend/src/services/storage.rs
touch ab-macros/frontend/src/styles/mod.rs
touch ab-macros/frontend/src/styles/theme.rs
touch ab-macros/frontend/src/utils/mod.rs
touch ab-macros/frontend/src/utils/formatters.rs
touch ab-macros/frontend/src/utils/validators.rs
touch ab-macros/frontend/Cargo.toml
touch ab-macros/frontend/index.html
touch ab-macros/frontend/static/css/main.css
touch ab-macros/frontend/static/css/neubrutalism.css
touch ab-macros/frontend/static/js/main.js
touch ab-macros/frontend/static/assets/logo.svg
touch ab-macros/frontend/static/assets/manifest.json
touch ab-macros/frontend/static/assets/service-worker.js

# Create shared structure
touch ab-macros/shared/src/lib.rs
touch ab-macros/shared/src/models.rs
touch ab-macros/shared/src/dto.rs
touch ab-macros/shared/src/validation.rs
touch ab-macros/shared/Cargo.toml

# Create migration files
touch ab-macros/migrations/initial_schema.rs
touch ab-macros/migrations/seed_nutrients.rs
touch ab-macros/migrations/seed_foods.rs

# Create workspace files
touch ab-macros/Cargo.toml
touch ab-macros/.gitignore
touch ab-macros/README.md
touch ab-macros/flake.nix

# Make script executable
chmod +x ab-macros/frontend/static/assets/service-worker.js

echo "Project skeleton created at ./ab-macros"
