#!/usr/bin/env bash

# Exit immediately if any command fails
set -e

# Function to clean up background processes on exit
cleanup() {
  echo "Cleaning up processes..."
  # Kill any background processes started by this script
  if [[ -n "$TRUNK_PID" ]]; then
    echo "Stopping Trunk server (PID: $TRUNK_PID)..."
    kill $TRUNK_PID 2>/dev/null || true
  fi
  echo "Cleanup complete."
}

# Function to display errors and exit
build_failed() {
  echo "----------------------------------------------"
  echo "❌ BUILD FAILED: $1"
  echo "----------------------------------------------"
  exit 1
}

# Set up trap to call cleanup on script exit
trap cleanup EXIT

echo "Checking if project builds correctly..."

# Make sure the frontend-dist directory exists
echo "Creating frontend-dist directory..."
mkdir -p frontend-dist

# Ensure needed tools are installed
if ! command -v trunk &> /dev/null; then
    echo "Trunk not found. Installing..."
    cargo install trunk || build_failed "Failed to install Trunk"
fi

# Check if the frontend builds correctly
echo "Checking frontend build..."
cd frontend
if ! trunk build -v; then
    build_failed "Frontend build failed"
fi
cd ..

# Check if the backend builds correctly
echo "Checking backend build..."
if ! cargo check -p backend; then
    build_failed "Backend build failed"
fi

# If we get here, everything builds correctly
echo "✅ All components build successfully!"

if [[ "$1" == "release" ]]; then
    echo "Building for release..."
    
    # Build the frontend
    echo "Building frontend..."
    cd frontend
    trunk build --release || build_failed "Frontend release build failed"
    cd ..
    
    # Copy the frontend build to the frontend-dist directory
    echo "Copying frontend assets..."
    cp -r frontend/dist/* frontend-dist/ || build_failed "Failed to copy frontend assets"
    
    # Build the backend with embedded frontend
    echo "Building backend..."
    cargo build --release || build_failed "Backend release build failed"
    
    # Create a release directory and copy the binary
    echo "Creating release package..."
    rm -rf release
    mkdir -p release
    cp target/release/backend release/ab-macros || build_failed "Failed to copy binary"
    cp .env.example release/.env || build_failed "Failed to copy .env file"
    
    echo "✅ Release build completed! The binary is at release/ab-macros"
else
    # Development mode - create placeholder files for RustEmbed to find
    echo "Creating placeholder index.html for RustEmbed..."
    echo "<html><body>Placeholder for development</body></html>" > frontend-dist/index.html
    
    # Start trunk in the background
    echo "Starting frontend dev server..."
    cd frontend
    trunk serve --proxy-backend=http://localhost:8080/api/ &
    TRUNK_PID=$!
    cd ..
    
    # Start the backend
    echo "Starting backend server..."
    cargo run -p backend || build_failed "Backend server failed to start"
fi
