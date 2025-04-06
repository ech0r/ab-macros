use yew::{function_component, classes, html, Html};

#[function_component]
pub fn History() -> Html {
    html! {
      <section class={classes!("panel", "history-panel")}>
        <h2 class={classes!("panel-header")}>{"Recent History"}</h2>
        <div class={classes!("history-cards")}>
          <div class={classes!("history-card")}>
            <div class={classes!("history-date")}>{"Yesterday"}</div>
            <div class={classes!("history-stats")}>
              <div class={classes!("history-stat")}>
                <span class={classes!("history-label")}>{"Protein"}</span>
                <span class={classes!("history-value")}>{"175g"}</span>
              </div>
              <div class={classes!("history-stat")}>
                <span class={classes!("history-label")}>{"Fat"}</span>
                <span class={classes!("history-value")}>{"95g"}</span>
              </div>
              <div class={classes!("history-stat")}>
                <span class={classes!("history-label")}>{"Calories"}</span>
                <span class={classes!("history-value")}>{"2150"}</span>
              </div>
              <div class={classes!("history-stat")}>
                <span class={classes!("history-label")}>{"Score"}</span>
                <span class={classes!("history-value")}>{"89"}</span>
              </div>
            </div>
          </div>
          
          <div class={classes!("history-card")}>
            <div class={classes!("history-date")}>{"April 4"}</div>
            <div class={classes!("history-stats")}>
              <div class={classes!("history-stat")}>
                <span class={classes!("history-label")}>{"Protein"}</span>
                <span class={classes!("history-value")}>{"162g"}</span>
              </div>
              <div class={classes!("history-stat")}>
                <span class={classes!("history-label")}>{"Fat"}</span>
                <span class={classes!("history-value")}>{"88g"}</span>
              </div>
              <div class={classes!("history-stat")}>
                <span class={classes!("history-label")}>{"Calories"}</span>
                <span class={classes!("history-value")}>{"2050"}</span>
              </div>
              <div class={classes!("history-stat")}>
                <span class={classes!("history-label")}>{"Score"}</span>
                <span class={classes!("history-value")}>{"84"}</span>
              </div>
            </div>
          </div>
          
          <div class={classes!("history-card")}>
            <div class={classes!("history-date")}>{"April 3"}</div>
            <div class={classes!("history-stats")}>
              <div class={classes!("history-stat")}>
                <span class={classes!("history-label")}>{"Protein"}</span>
                <span class={classes!("history-value")}>{"145g"}</span>
              </div>
              <div class={classes!("history-stat")}>
                <span class={classes!("history-label")}>{"Fat"}</span>
                <span class={classes!("history-value")}>{"78g"}</span>
              </div>
              <div class={classes!("history-stat")}>
                <span class={classes!("history-label")}>{"Calories"}</span>
                <span class={classes!("history-value")}>{"1920"}</span>
              </div>
              <div class={classes!("history-stat")}>
                <span class={classes!("history-label")}>{"Score"}</span>
                <span class={classes!("history-value")}>{"76"}</span>
              </div>
            </div>
          </div>
        </div>
      </section>
    }
}
