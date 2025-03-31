#!/bin/bash
# Build script for AB Macros - builds and runs the project
# Usage: ./build.sh [release]

# Exit immediately if any command fails
set -e
# Exit if any command in a pipeline fails
set -o pipefail

# Check if release mode is requested
if [ "$1" == "release" ]; then
    BUILD_MODE="release"
    echo "Building in RELEASE mode..."
else
    BUILD_MODE="dev"
    echo "Building in DEVELOPMENT mode..."
fi

# Create static directory if it doesn't exist
mkdir -p static
rm -rf static/*

# Build frontend with wasm-pack
echo "Building frontend with wasm-pack..."
cd frontend

if [ "$BUILD_MODE" == "release" ]; then
    echo "Building Wasm in release mode..."
    wasm-pack build --target web --out-dir pkg --release || { echo "Frontend build failed!"; exit 1; }
else
    echo "Building Wasm in development mode..."
    # Add the --dev flag and enable debug info in development mode
    RUSTFLAGS="-C debuginfo=2" wasm-pack build --target web --out-dir pkg --dev || { echo "Frontend build failed!"; exit 1; }
fi

# Copy frontend assets to static directory
echo "Copying frontend assets to static directory..."
cp index.html ../static/ || { echo "Failed to copy index.html!"; exit 1; }
cp -r pkg/* ../static/ || { echo "Failed to copy wasm package!"; exit 1; }

# Also copy the JS glue code specifically (sometimes needed for proper paths)
cp pkg/ab_macros_frontend.js ../static/ || { echo "Failed to copy JS glue code!"; exit 1; }
cp pkg/ab_macros_frontend_bg.wasm ../static/ || { echo "Failed to copy WASM binary!"; exit 1; }

# Copy other static assets if they exist
if [ -d "static" ]; then
    echo "Copying additional static assets..."
    cp -r static/* ../static/ || { echo "Failed to copy static assets!"; exit 1; }
fi

cd ..

# Build the backend
echo "Building backend..."
if [ "$BUILD_MODE" == "release" ]; then
    cargo build --release || { echo "Backend build failed!"; exit 1; }
    
    echo "Build completed successfully!"
    echo "Running server in release mode..."
    ./target/release/ab-macros
else
    cargo build || { echo "Backend build failed!"; exit 1; }
    
    echo "Build completed successfully!"
    echo "Running server in development mode..."
    ./target/debug/ab-macros
fi
