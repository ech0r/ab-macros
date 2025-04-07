use wasm_bindgen::prelude::*;
use yew::prelude::*;

mod components;
use components::{Header, Dashboard};

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <div class={classes!("container")}>
            <Header/>
            <Dashboard/>
        </div>
    }
}

// This function is called when the wasm module is instantiated
#[wasm_bindgen(start)]
pub fn run_app() -> Result<(), JsValue> {
    // Initialize panic hook for better error messages
    console_error_panic_hook::set_once();
    
    yew::Renderer::<App>::new().render();
    Ok(())
}
