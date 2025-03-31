use yew::prelude::*;

use crate::styles::card;

#[function_component(HomePage)]
pub fn home_page() -> Html {
    let card_style = card();
    
    html! {
        <div class="container" style="padding-top: 2rem;">
            <h1>{"Dashboard"}</h1>
            
            <div class={card_style}>
                <h2>{"Welcome to AB Macros"}</h2>
                <p>{"Your animal-based diet tracking app"}</p>
                
                <div style="margin-top: 2rem;">
                    <p>{"This is a placeholder for the dashboard content."}</p>
                    <p>{"Here you'll see your daily summary, recent meals, and nutrient status."}</p>
                </div>
            </div>
        </div>
    }
}
