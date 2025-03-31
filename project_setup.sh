#!/bin/bash
# Script to create all project files for AB Macros application

# Create directories
mkdir -p src
mkdir -p frontend/src/components
mkdir -p frontend/src/models
mkdir -p frontend/src/pages
mkdir -p frontend/src/utils
mkdir -p frontend/src/store
mkdir -p frontend/static/icons

# Create root files
touch Cargo.toml
touch .gitignore
cp .env.example .env
touch README.md

# Create backend files
touch src/main.rs
touch src/api.rs
touch src/auth.rs
touch src/db.rs
touch src/models.rs
touch src/utils.rs

# Create frontend files
touch frontend/Cargo.toml
touch frontend/index.html

# Create frontend source files
touch frontend/src/main.rs
touch frontend/src/app.rs
touch frontend/src/api.rs
touch frontend/src/styles.rs

# Create frontend components
touch frontend/src/components/mod.rs
touch frontend/src/components/layout.rs
touch frontend/src/components/auth_guard.rs
touch frontend/src/components/food_item_card.rs

# Create frontend pages
touch frontend/src/pages/mod.rs
touch frontend/src/pages/login.rs
touch frontend/src/pages/home.rs
touch frontend/src/pages/meal_entry.rs
touch frontend/src/pages/meal_list.rs
touch frontend/src/pages/nutrient_report.rs
touch frontend/src/pages/profile.rs
touch frontend/src/pages/not_found.rs

# Create frontend utils
touch frontend/src/utils/mod.rs
touch frontend/src/utils/storage.rs

# Create frontend store
touch frontend/src/store/mod.rs
touch frontend/src/store/auth.rs

# Create frontend models
touch frontend/src/models/mod.rs

# Create frontend static files
touch frontend/static/manifest.json
touch frontend/static/service-worker.js

# Create frontend placeholder icon files
touch frontend/static/icons/icon-192x192.png
touch frontend/static/icons/icon-512x512.png
touch frontend/static/icons/maskable-icon.png
touch frontend/static/icons/shortcut-meal-96x96.png
touch frontend/static/icons/shortcut-reports-96x96.png

echo "All files and directories created successfully!"
