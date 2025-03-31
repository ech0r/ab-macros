use yew::prelude::*;
use stylist::yew::styled_component;

use crate::models::FoodItem;
use crate::styles::colors;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub food: FoodItem,
    pub amount: f32,
    pub on_remove: Callback<()>,
}

#[styled_component(FoodItemCard)]
pub fn food_item_card(props: &Props) -> Html {
    // Calculate scaled nutrients based on amount
    let scale_factor = props.amount / props.food.serving_size;
    
    let calories = props.food.macros.calories * scale_factor;
    let protein = props.food.macros.protein * scale_factor;
    let fat = props.food.macros.fat * scale_factor;
    let carbs = props.food.macros.carbs * scale_factor;
    
    // Handle remove button click
    let on_remove_click = {
        let on_remove = props.on_remove.clone();
        Callback::from(move |e: MouseEvent| {
            e.prevent_default();
            on_remove.emit(());
        })
    };
    
    // Get appropriate color based on food category
    let category_color = match props.food.category {
        crate::models::FoodCategory::Meat => "#FF6B6B",   // Red
        crate::models::FoodCategory::Organ => "#CC6666",  // Dark red
        crate::models::FoodCategory::Fish => "#6EB5FF",   // Blue
        crate::models::FoodCategory::Eggs => "#FFEA80",   // Yellow
        crate::models::FoodCategory::Dairy => "#E0E0E0",  // Light gray
        crate::models::FoodCategory::Fruit => "#A0DB8E",  // Green
        crate::models::FoodCategory::Honey => "#FFD07F",  // Orange
        crate::models::FoodCategory::Other => "#C0C0C0",  // Gray
    };
    
    // CSS styles
    let card_style = format!(
        "
        display: flex;
        margin-bottom: 1rem;
        border: 3px solid #000;
        box-shadow: 4px 4px 0 #000000;
        background-color: white;
        overflow: hidden;
        "
    );
    
    let category_indicator_style = format!(
        "
        width: 12px;
        background-color: {};
        ",
        category_color
    );
    
    let content_style = format!(
        "
        flex: 1;
        padding: 1rem;
        "
    );
    
    let action_style = format!(
        "
        display: flex;
        align-items: center;
        padding: 0 1rem;
        "
    );
    
    let remove_button_style = format!(
        "
        background-color: {};
        color: white;
        border: 2px solid #000;
        border-radius: 0;
        padding: 0.5rem;
        display: flex;
        align-items: center;
        justify-content: center;
        cursor: pointer;
        width: 32px;
        height: 32px;
        line-height: 1;
        font-family: sans-serif;
        ",
        colors::ERROR
    );
    
    html! {
        <div style={card_style}>
            <div style={category_indicator_style}></div>
            <div style={content_style}>
                <div style="display: flex; justify-content: space-between; margin-bottom: 0.5rem;">
                    <h4 style="margin: 0;">{&props.food.name}</h4>
                    <span style="font-weight: bold;">
                        {format!("{:.0}g", props.amount)}
                    </span>
                </div>
                
                <div style="display: flex; gap: 1rem; font-size: 0.9rem;">
                    <div>
                        <span style="font-weight: bold; color: #FF3D00;">
                            {format!("{:.0}", calories)}
                        </span>
                        {" kcal"}
                    </div>
                    <div>
                        <span style="font-weight: bold;">
                            {format!("{:.1}g", protein)}
                        </span>
                        {" protein"}
                    </div>
                    <div>
                        <span style="font-weight: bold;">
                            {format!("{:.1}g", fat)}
                        </span>
                        {" fat"}
                    </div>
                    <div>
                        <span style="font-weight: bold;">
                            {format!("{:.1}g", carbs)}
                        </span>
                        {" carbs"}
                    </div>
                </div>
            </div>
            <div style={action_style}>
                <button 
                    type="button"
                    style={remove_button_style}
                    onclick={on_remove_click}
                    title="Remove item"
                >
                    {"×"}
                </button>
            </div>
        </div>
    }
}
