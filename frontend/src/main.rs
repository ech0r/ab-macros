use wasm_bindgen::JsCast;
use js_sys;

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
            if let Ok(service_worker) = js_sys::Reflect::get(&navigator, &"serviceWorker".into()) {
                if service_worker.is_undefined() || service_worker.is_null() {
                    log::warn!("Service Worker API not available");
                    return;
                }
                
                let _ = wasm_bindgen_futures::spawn_local(async move {
                    // Safely try to get the register method
                    if let Ok(register_method) = js_sys::Reflect::get(&service_worker, &"register".into()) {
                        if register_method.is_undefined() || register_method.is_null() {
                            log::error!("register method not found on serviceWorker");
                            return;
                        }
                        
                        if register_method.is_function() {
                            // Convert to function and call it
                            let register_fn = register_method.dyn_into::<js_sys::Function>().unwrap();
                            let sw_path = wasm_bindgen::JsValue::from_str("/service-worker.js");
                            
                            if let Ok(promise) = register_fn.call1(&service_worker, &sw_path) {
                                if promise.is_instance_of::<js_sys::Promise>() {
                                    let promise = promise.dyn_into::<js_sys::Promise>().unwrap();
                                    match wasm_bindgen_futures::JsFuture::from(promise).await {
                                        Ok(_reg) => log::info!("Service worker registered successfully"),
                                        Err(e) => log::error!("Failed to register service worker: {:?}", e),
                                    }
                                }
                            }
                        }
                    }
                });
            }
        }
    }
    
    log::info!("Starting AB Macros frontend application");
    
    // Start the Yew app
    yew::Renderer::<App>::new().render();
}
