use yew::prelude::*;

mod components;

use components::{Header, Dashboard};

#[function_component]
fn App() -> Html {
    let counter = use_state(|| 0);
    let onclick = {
        let counter = counter.clone();
        move |_: u32| {
            let value = *counter + 1;
            counter.set(value);
        }
    };

    html! {
        <div class={classes!("container")}>
            <Header/>
            <Dashboard/>
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
