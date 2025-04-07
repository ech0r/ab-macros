// src/components/header.rs
use yew::{function_component, classes, html, Html, use_effect_with, use_state, Callback};
use gloo_net::http::Request;
use serde::Deserialize;
use web_sys::console;

#[derive(Deserialize, Clone, PartialEq, Debug)]
struct UserInfo {
    username: String,
    id: String,
}

#[function_component]
pub fn Header() -> Html {
    let user = use_state(|| None::<UserInfo>);
    let error = use_state(|| None::<String>);
    
    // Fetch current user info when component mounts
    {
        let user = user.clone();
        let error = error.clone();
        
        use_effect_with(
            (),
            move |_| {
                wasm_bindgen_futures::spawn_local(async move {
                    match Request::get("/api/me")
                        .send()
                        .await
                    {
                        Ok(response) => {
                            if response.status() == 200 {
                                match response.json::<UserInfo>().await {
                                    Ok(data) => {
                                        user.set(Some(data));
                                    }
                                    Err(e) => {
                                        error.set(Some(format!("Failed to parse user info: {}", e)));
                                    }
                                }
                            }
                        }
                        Err(e) => {
                            // It's normal to get an error if not logged in - we can ignore it
                            console::log_1(&format!("Error fetching user: {}", e).into());
                        }
                    }
                });
                || ()
            },
        );
    }
    
    // Handle login click
    let on_login_click = {
        Callback::from(move |_| {
            wasm_bindgen_futures::spawn_local(async {
                match Request::get("/api/auth-url")
                    .send()
                    .await
                {
                    Ok(response) => {
                        if response.status() == 200 {
                            #[derive(Deserialize)]
                            struct AuthUrl {
                                url: String,
                            }
                            
                            match response.json::<AuthUrl>().await {
                                Ok(data) => {
                                    // Redirect to Reddit authorization page
                                    let window = web_sys::window().unwrap();
                                    let _ = window.location().set_href(&data.url);
                                }
                                Err(e) => {
                                    console::log_1(&format!("Failed to parse auth URL: {}", e).into());
                                }
                            }
                        }
                    }
                    Err(e) => {
                        console::log_1(&format!("Error fetching auth URL: {}", e).into());
                    }
                }
            });
        })
    };
    
    // Handle logout click
    let on_logout_click = {
        let user = user.clone();
        
        Callback::from(move |_| {
            let user = user.clone();
            
            wasm_bindgen_futures::spawn_local(async move {
                match Request::post("/api/logout")
                    .send()
                    .await
                {
                    Ok(_) => {
                        user.set(None);
                    }
                    Err(e) => {
                        console::log_1(&format!("Error logging out: {}", e).into());
                    }
                }
            });
        })
    };
    
    html! {
        <header class={classes!("header")}>
            <div class={classes!("logo")}>
                {"/r/ANIMALBASED MACRO TRACKER"}
            </div>
            
            <div class={classes!("nav-controls")}>
                if let Some(user_info) = (*user).clone() {
                    <span class={classes!("username")}>
                        {format!("Hi, {}", user_info.username)}
                    </span>
                    <button 
                        class={classes!("nav-button")}
                        onclick={on_logout_click}
                    >
                        {"LOGOUT"}
                    </button>
                } else {
                    <button 
                        class={classes!("nav-button")}
                        onclick={on_login_click}
                    >
                        {"LOGIN WITH REDDIT"}
                    </button>
                }
            </div>
        </header>
    }
}
