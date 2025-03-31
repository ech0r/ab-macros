use yew::prelude::*;
use yew_router::prelude::*;

use crate::app::Route;
use crate::styles::{card, primary_button};

#[function_component(MealListPage)]
pub fn meal_list_page() -> Html {
    let navigator = use_navigator().unwrap();
    let card_style = card();
    let button_style = primary_button();
    
    let on_add_meal = {
        let navigator = navigator.clone();
        Callback::from(move |_| {
            navigator.push(&Route::NewMeal);
        })
    };
    
    html! {
        <div class="container" style="padding-top: 2rem;">
            <div style="display: flex; justify-content: space-between; align-items: center; margin-bottom: 1.5rem;">
                <h1 style="margin: 0;">{"Your Meals"}</h1>
                <button class={button_style} onclick={on_add_meal}>
                    {"+ Add Meal"}
                </button>
            </div>
            
            <div class={card_style}>
                <p>{"This is a placeholder for the meal list."}</p>
                <p>{"Here you'll see your recorded meals with the ability to edit or delete them."}</p>
            </div>
        </div>
    }
}
