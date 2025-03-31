use yew::prelude::*;
use yew_router::prelude::*;

use crate::app::Route;
use crate::styles::{card, primary_button};

#[function_component(NotFoundPage)]
pub fn not_found_page() -> Html {
    let navigator = use_navigator().unwrap();
    let card_style = card();
    let button_style = primary_button();
    
    let go_home = {
        let navigator = navigator.clone();
        Callback::from(move |_| {
            navigator.push(&Route::Home);
        })
    };
    
    html! {
        <div class="container" style="padding-top: 4rem; text-align: center;">
            <div class={card_style}>
                <h1 style="font-size: 3rem; margin-bottom: 1rem;">{"404"}</h1>
                <h2>{"Page Not Found"}</h2>
                <p>{"Sorry, the page you're looking for doesn't exist."}</p>
                
                <div style="margin-top: 2rem;">
                    <button class={button_style} onclick={go_home}>
                        {"Go to Dashboard"}
                    </button>
                </div>
            </div>
        </div>
    }
}
