use wasm_bindgen::JsCast;

mod api;
mod app;
mod components;
mod models;
mod pages;
mod store;
mod styles;
mod utils;

use app::App;

fn main() {
    // Initialize logger
    wasm_logger::init(wasm_logger::Config::default());
    
    // Check if service worker is supported and register it for PWA capabilities
    if let Some(window) = web_sys::window() {
        if let Some(navigator) = window.navigator().dyn_ref::<web_sys::Navigator>() {
            let service_worker = navigator.service_worker();
            let _ = wasm_bindgen_futures::spawn_local(async move {
                if let Ok(registration_promise) = service_worker.register("/service-worker.js") {
                    match wasm_bindgen_futures::JsFuture::from(registration_promise).await {
                        Ok(_) => log::info!("Service worker registered successfully"),
                        Err(e) => log::error!("Failed to register service worker: {:?}", e),
                    }
                } else {
                    log::error!("Error creating service worker registration promise");
                }
            });
        }
    }
    
    log::info!("Starting AB Macros frontend application");
    
    // Start the Yew app
    yew::Renderer::<App>::new().render();
}
