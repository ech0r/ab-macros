use yew::prelude::*;

use crate::styles::card;

#[function_component(NutrientReportPage)]
pub fn nutrient_report_page() -> Html {
    let card_style = card();
    
    html! {
        <div class="container" style="padding-top: 2rem;">
            <h1>{"Nutrient Reports"}</h1>
            
            <div class={card_style}>
                <h2>{"Nutrient Summary"}</h2>
                <p>{"This is a placeholder for the nutrient reporting page."}</p>
                <p>{"Here you'll see charts and summaries of your nutrient intake over time."}</p>
            </div>
        </div>
    }
}
