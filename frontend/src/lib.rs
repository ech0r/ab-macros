use wasm_bindgen::prelude::*;

mod api;
mod app;
mod components;
mod models;
mod pages;
mod store;
mod styles;
mod utils;

// Export the main function for wasm-pack
#[wasm_bindgen]
pub fn run_app() -> Result<(), JsValue> {
    // Set up panic hook for better error messages
    console_error_panic_hook::set_once();
    
    // Initialize logger
    wasm_logger::init(wasm_logger::Config::default());
    
    log::info!("Starting AB Macros frontend application");
    web_sys::console::log_1(&"Starting AB Macros frontend application".into());
    
    // Start the Yew app
    yew::Renderer::<app::App>::new().render();
    
    Ok(())
}

// This function is automatically called when the wasm module is loaded
#[wasm_bindgen(start)]
pub fn start() {
    log::info!("AB Macros WebAssembly module initialized");
    web_sys::console::log_1(&"AB Macros WebAssembly module initialized".into());
}
