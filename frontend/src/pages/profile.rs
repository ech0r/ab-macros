use yew::prelude::*;

use crate::styles::card;

#[function_component(ProfilePage)]
pub fn profile_page() -> Html {
    let card_style = card();
    
    html! {
        <div class="container" style="padding-top: 2rem;">
            <h1>{"Your Profile"}</h1>
            
            <div class={card_style}>
                <h2>{"User Profile"}</h2>
                <p>{"This is a placeholder for the user profile page."}</p>
                <p>{"Here you'll be able to set your nutrient targets and view your account information."}</p>
            </div>
        </div>
    }
}
