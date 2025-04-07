use yew::{function_component, classes, html, Html};
use crate::components::{AddMeal, Progress, History};

#[function_component]
pub fn Dashboard() -> Html {
    html! {
        <main class={classes!("dashboard")}>
            <AddMeal/>
            <Progress/>
            <History />
        </main>
    }
}
