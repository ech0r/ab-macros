use yew::{function_component, classes, html, Html};

#[function_component]
pub fn Progress() -> Html {
    html! {
      <section class={classes!("panel")}>
        <h2 class={classes!("panel-header")}>{"Today's Progress"}</h2>
        
        <div class={classes!("macro-display")}>
          <div class={classes!("macro-card")}>
            <div class={classes!("macro-title")}>{"Protein"}</div>
            <div class={classes!("macro-value")}>{"120g / 180g"}</div>
            <div class={classes!("progress-container")}>
              <div class={classes!("progress-bar")} style="width: 65%;"></div>
            </div>
          </div>
          
          <div class={classes!("macro-card")}>
            <div class={classes!("macro-title")}>{"Fat"}</div>
            <div class={classes!("macro-value")}>{"75g / 100g"}</div>
            <div class={classes!("progress-container")}>
              <div class={classes!("progress-bar")} style="width: 75%;"></div>
            </div>
          </div>
          
          <div class={classes!("macro-card")}>
            <div class={classes!("macro-title")}>{"Calories"}</div>
            <div class={classes!("macro-value")}>{"1500 / 2200"}</div>
            <div class={classes!("progress-container")}>
              <div class={classes!("progress-bar")} style="width: 68%;"></div>
            </div>
          </div>
          
          <div class={classes!("macro-card")}>
            <div class={classes!("macro-title")}>{"Animal Score"}</div>
            <div class={classes!("macro-value")}>{"72"}</div>
            <div class={classes!("progress-container")}>
              <div class={classes!("progress-bar")} style="width: 72%;"></div>
            </div>
          </div>
        </div>
      </section>
    }
}
