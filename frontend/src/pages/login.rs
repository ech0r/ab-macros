use yew::prelude::*;
use yew_router::prelude::*;
use stylist::yew::styled_component;
use web_sys::HtmlInputElement;
use gloo::storage::LocalStorage;
use gloo::storage::Storage;
use gloo::timers::callback::Timeout;

use crate::api;
use crate::app::Route;
use crate::styles::{card, colors, primary_button};

#[derive(Clone, PartialEq)]
enum AuthStep {
    EnterPhone,
    EnterCode,
}

#[derive(Properties, PartialEq)]
pub struct Props {}

#[styled_component(LoginPage)]
pub fn login_page(_props: &Props) -> Html {
    let navigator = use_navigator().unwrap();
    let auth_step = use_state(|| AuthStep::EnterPhone);
    let phone = use_state(|| String::new());
    let code = use_state(|| String::new());
    let error_message = use_state(|| String::new());
    let is_loading = use_state(|| false);
    
    let phone_input_ref = use_node_ref();
    let code_input_ref = use_node_ref();
    
    // Handle phone number input
    let on_phone_change = {
        let phone = phone.clone();
        Callback::from(move |e: Event| {
            let input: HtmlInputElement = e.target_unchecked_into();
            phone.set(input.value());
        })
    };
    
    // Handle verification code input
    let on_code_change = {
        let code = code.clone();
        Callback::from(move |e: Event| {
            let input: HtmlInputElement = e.target_unchecked_into();
            code.set(input.value());
        })
    };
    
    // Handle phone form submission
    let on_submit_phone = {
        let phone = phone.clone();
        let auth_step = auth_step.clone();
        let error_message = error_message.clone();
        let is_loading = is_loading.clone();
        
        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();
            
            let phone_val = (*phone).clone();
            let is_loading = is_loading.clone();
            let error_message = error_message.clone();
            let auth_step = auth_step.clone();
            
            if phone_val.is_empty() {
                error_message.set("Please enter your phone number".to_string());
                return;
            }
            
            is_loading.set(true);
            
            wasm_bindgen_futures::spawn_local(async move {
                match api::send_verification_code(&phone_val).await {
                    Ok(_) => {
                        log::info!("Verification code sent");
                        auth_step.set(AuthStep::EnterCode);
                    }
                    Err(e) => {
                        log::error!("Failed to send verification code: {:?}", e);
                        error_message.set("Failed to send verification code. Please try again.".to_string());
                    }
                }
                
                is_loading.set(false);
            });
        })
    };
    
    // Handle verification code form submission
    let on_submit_code = {
        let phone = phone.clone();
        let code = code.clone();
        let error_message = error_message.clone();
        let is_loading = is_loading.clone();
        let navigator = navigator.clone();
        
        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();
            
            let phone_val = (*phone).clone();
            let code_val = (*code).clone();
            let error_message = error_message.clone();
            let is_loading = is_loading.clone();
            let navigator = navigator.clone();
            
            if code_val.is_empty() {
                error_message.set("Please enter the verification code".to_string());
                return;
            }
            
            is_loading.set(true);
            
            wasm_bindgen_futures::spawn_local(async move {
                match api::verify_code(&phone_val, &code_val).await {
                    Ok(token) => {
                        log::info!("Login successful");
                        
                        // Store the token
                        LocalStorage::set("auth_token", token).expect("Failed to save token");
                        
                        // Redirect to homepage
                        navigator.push(&Route::Home);
                    }
                    Err(e) => {
                        log::error!("Verification failed: {:?}", e);
                        error_message.set("Invalid verification code. Please try again.".to_string());
                    }
                }
                
                is_loading.set(false);
            });
        })
    };
    
    // Auto-focus inputs
    {
        let phone_input_ref = phone_input_ref.clone();
        let code_input_ref = code_input_ref.clone();
        
        use_effect_with_deps(
            move |current_step| {
                let auth_step_value = **current_step;
                let timeout = Timeout::new(100, move || {
                    match auth_step_value {
                        AuthStep::EnterPhone => {
                            if let Some(input) = phone_input_ref.cast::<HtmlInputElement>() {
                                let _ = input.focus();
                            }
                        }
                        AuthStep::EnterCode => {
                            if let Some(input) = code_input_ref.cast::<HtmlInputElement>() {
                                let _ = input.focus();
                            }
                        }
                    }
                });
                
                || drop(timeout)
            },
            auth_step.clone(),
        );
    }
    
    // Clear error message after 5 seconds
    {
        let error_message_clone = error_message.clone();
        
        use_effect_with_deps(
            move |message| {
                if !message.is_empty() {
                    let error_message = error_message_clone.clone();
                    let timeout = Timeout::new(5000, move || {
                        error_message.set(String::new());
                    });
                    
                    // Return a cleanup function
                    Box::new(move || drop(timeout)) as Box<dyn FnOnce()>
                } else {
                    // Return a no-op cleanup function with the same type
                    Box::new(|| {}) as Box<dyn FnOnce()>
                }
            },
            (*error_message).clone(),
        );
    }
    
    let card_style = card();
    let button_style = primary_button();
    
    html! {
        <div class="container">
            <div class="row" style="display: flex; justify-content: center; padding-top: 4rem;">
                <div class="col" style="max-width: 480px; width: 100%;">
                    <div style="margin-bottom: 1rem; padding: 0.75rem; background-color: #FFD07F; border: 2px solid #FF9800; text-align: center;">
                        <p style="margin: 0; font-weight: bold;">
                            {"Development Mode: Use phone +15555555555 and code 123456"}
                        </p>
                    </div>
                    <div class={card_style}>
                        <h1 style="text-align: center;">{"AB Macros"}</h1>
                        <p style="text-align: center; margin-bottom: 2rem;">{"Track your animal-based diet"}</p>
                        
                        if !(*error_message).is_empty() {
                            <div style={format!("background-color: {}; color: white; padding: 1rem; margin-bottom: 1rem; border-radius: 4px;", colors::ERROR)}>
                                { &*error_message }
                            </div>
                        }
                        
                        {
                            match *auth_step {
                                AuthStep::EnterPhone => html! {
                                    <form onsubmit={on_submit_phone}>
                                        <div style="margin-bottom: 1.5rem;">
                                            <label for="phone" style="display: block; margin-bottom: 0.5rem; font-weight: 500;">
                                                {"Phone Number"}
                                            </label>
                                            <input 
                                                id="phone"
                                                type="tel"
                                                placeholder="Enter your phone number"
                                                value={(*phone).clone()}
                                                onchange={on_phone_change}
                                                ref={phone_input_ref}
                                                disabled={*is_loading}
                                            />
                                            <p style="font-size: 0.875rem; margin-top: 0.5rem; color: #666;">
                                                {"We'll send a verification code to this number."}
                                            </p>
                                        </div>
                                        
                                        <button 
                                            type="submit" 
                                            class={button_style.clone()}
                                            style="width: 100%;"
                                            disabled={*is_loading}
                                        >
                                            if *is_loading {
                                                {"Sending code..."}
                                            } else {
                                                {"Send Verification Code"}
                                            }
                                        </button>
                                    </form>
                                },
                                AuthStep::EnterCode => html! {
                                    <form onsubmit={on_submit_code}>
                                        <div style="margin-bottom: 1.5rem;">
                                            <label for="code" style="display: block; margin-bottom: 0.5rem; font-weight: 500;">
                                                {"Verification Code"}
                                            </label>
                                            <input 
                                                id="code"
                                                type="text"
                                                placeholder="Enter the verification code"
                                                value={(*code).clone()}
                                                onchange={on_code_change}
                                                ref={code_input_ref}
                                                disabled={*is_loading}
                                                maxlength="6"
                                                pattern="[0-9]*"
                                                inputmode="numeric"
                                            />
                                            <p style="font-size: 0.875rem; margin-top: 0.5rem; color: #666;">
                                                {"We sent a 6-digit code to "}<strong>{(*phone).clone()}</strong>
                                            </p>
                                        </div>
                                        
                                        <button 
                                            type="submit" 
                                            class={button_style.clone()}
                                            style="width: 100%; margin-bottom: 1rem;"
                                            disabled={*is_loading}
                                        >
                                            if *is_loading {
                                                {"Verifying..."}
                                            } else {
                                                {"Verify & Login"}
                                            }
                                        </button>
                                        
                                        <div style="text-align: center;">
                                            <a 
                                                href="#"
                                                onclick={
                                                    let auth_step = auth_step.clone();
                                                    Callback::from(move |e: MouseEvent| {
                                                        e.prevent_default();
                                                        auth_step.set(AuthStep::EnterPhone);
                                                    })
                                                }
                                                style="color: #666; text-decoration: underline; font-size: 0.875rem;"
                                            >
                                                {"Use a different phone number"}
                                            </a>
                                        </div>
                                    </form>
                                },
                            }
                        }
                    </div>
                </div>
            </div>
        </div>
    }
}
