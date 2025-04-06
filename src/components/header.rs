use yew::{function_component, classes, html, Html};

#[function_component]
pub fn Header() -> Html {
    html! {
        <header class={classes!("header")}>
            <div class={classes!("logo")}>
            {
                "/r/ANIMALBASED MACRO TRACKER"
            }
            </div>
            <button class={classes!("nav-button")}>
            {
                "MY PROFILE"
            }
            </button>
        </header>
    }
}
