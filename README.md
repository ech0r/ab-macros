# AB Macros - Animal-Based Diet Tracker

A full-stack Rust PWA for tracking macros and nutrients for an animal-based diet.

## Features

- Track meals and macronutrients for an animal-based diet (meat, fruit, dairy)
- Responsive, mobile-friendly interface with atomic-age/neubrutalist design
- SMS-based authentication using Twilio
- Daily, weekly, and monthly nutrient reports
- Works offline with PWA capabilities

## Tech Stack

- **Backend**: Rust with Actix Web
- **Frontend**: Rust with Yew framework
- **Database**: Sled embedded database
- **Authentication**: JWT + Twilio SMS OTP
- **Styling**: CSS-in-Rust with Stylist

## Project Structure

```
ab-macros/
├── src/               # Backend code
│   ├── main.rs        # Entry point
│   ├── api.rs         # API routes
│   ├── auth.rs        # Authentication logic
│   ├── db.rs          # Database operations
│   ├── models.rs      # Shared data models
│   └── utils.rs       # Utility functions
├── frontend/          # Frontend code (Yew)
│   ├── src/           # Frontend source code
│   ├── index.html     # HTML template
│   └── static/        # Static assets
└── Cargo.toml         # Project manifest
```

## Setup and Development

### Prerequisites

- Rust and Cargo (latest stable)
- [Trunk](https://trunkrs.dev/) for Yew development
- [wasm-pack](https://rustwasm.github.io/wasm-pack/) for WebAssembly compilation
- Twilio account for SMS functionality

### Environment Variables

Create a `.env` file in the project root with the following variables:

```
PORT=8080
JWT_SECRET=your_jwt_secret_key
TWILIO_ACCOUNT_SID=your_twilio_account_sid
TWILIO_AUTH_TOKEN=your_twilio_auth_token
TWILIO_FROM_NUMBER=your_twilio_phone_number
DB_PATH=./ab_macros_db
STATIC_FILES_PATH=./static
```

### Development Setup

1. **Install Nix** (optional, but recommended for consistent environment):
   
   ```bash
   # For Linux or macOS
   curl -L https://nixos.org/nix/install | sh
   
   # Enable flakes if not already enabled
   mkdir -p ~/.config/nix
   echo "experimental-features = nix-command flakes" >> ~/.config/nix/nix.conf
   ```

2. **Enter development shell** (if using Nix):
   
   ```bash
   nix develop
   ```

3. **Build frontend**:
   
   ```bash
   cd frontend
   trunk build --release
   ```

4. **Build backend**:
   
   ```bash
   cargo build --release
   ```

5. **Run the server**:
   
   ```bash
   ./target/release/ab-macros
   ```

6. **Visit the application** at `http://localhost:8080`

### Deployment

The project is designed to be deployed as a single binary with embedded static files.

1. **Build for production**:
   
   ```bash
   # Build frontend
   cd frontend
   trunk build --release
   
   # Build backend with frontend embedded
   cd ..
   cargo build --release
   ```

2. **Deploy the binary**:
   
   Simply copy the resulting binary from `./target/release/ab-macros` to your server.
   
3. **Set up environment variables on your server**:
   
   Make sure to configure the same environment variables on your server.

## License

This project is licensed under the MIT License - see the LICENSE file for details.
