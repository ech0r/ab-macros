use yew::{function_component, classes, html, Html};

#[function_component]
pub fn AddMeal() -> Html {
    html! {
      <section class={classes!("panel")}>
        <h2 class={classes!("panel-header")}>{"Add Meal"}</h2>
        <div class={classes!("food-selector")}>
            <label class={classes!("selector-label")}>{"Select Food Type"}</label>
            <div class={classes!("food-buttons")}>
              <button class={classes!("food-button", "active")}>
                <span class={classes!("food-icon")}>{"🥩"}</span>
                <span>{"Beef"}</span>
              </button>
              <button class={classes!("food-button")}>
                <span class={classes!("food-icon")}>{"🐓"}</span>
                <span>{"Poultry"}</span>
              </button>
              <button class={classes!("food-button")}>
                <span class={classes!("food-icon")}>{"🐟"}</span>
                <span>{"Fish"}</span>
              </button>
              <button class={classes!("food-button")}>
                <span class={classes!("food-icon")}>{"🥚"}</span>
                <span>{"Eggs"}</span>
              </button>
              <button class={classes!("food-button")}>
                <span class={classes!("food-icon")}>{"🥓"}</span>
                <span>{"Pork"}</span>
              </button>
              <button class={classes!("food-button")}>
                <span class={classes!("food-icon")}>{"🧀"}</span>
                <span>{"Dairy"}</span>
              </button>
              <button class={classes!("food-button")}>
                <span class={classes!("food-icon")}>{"🍎"}</span>
                <span>{"Fruits"}</span>
              </button>
              <button class={classes!("food-button")}>
                <span class={classes!("food-icon")}>{"🍯"}</span>
                <span>{"Honey"}</span>
              </button>
            </div>
          </div>
          
          <div class={classes!("meal-form")}>
            <div class={classes!("input-group")}>
              <label class={classes!("input-label")}>
                <span class={classes!("animal-icon")}>{"🍖"}</span>
                {"Food"}
              </label>
              <select class={classes!("select-field")}>
                <option>{"Ribeye Steak"} </option>
                <option>{"Ground Beef"}</option>
                <option>{"Beef Liver"}</option>
                <option>{"Brisket"}</option>
                <option>{"Custom..."}</option>
              </select>
            </div>
          
            <div class={classes!("input-group")}>
              <label class={classes!("input-label")}>
                <span class={classes!("animal-icon")}>{"⚖️"}</span>
                  {"Amount (g)"}
                </label>
                <input type="number" class="input-field" placeholder="0"/> 
            </div>
          
            <div class={classes!("macro-preview")}>
              <div class={classes!("macro-preview-item")}>
                <span class={classes!("preview-label")}>{"Protein"}</span>
                <span class={classes!("preview-value")}>{"45g"}</span>
              </div>
              <div class={classes!("macro-preview-item")}>
                <span class={classes!("preview-label")}>{"Fat"}</span>
                <span class={classes!("preview-value")}>{"38g"}</span>
              </div>
              <div class={classes!("macro-preview-item")}>
                <span class={classes!("preview-label")}>{"Calories"}</span>
                <span class={classes!("preview-value")}>{"530"}</span>
              </div>
            </div>
          </div>
          
          <button class={classes!("submit-button")}>{"ADD FOOD"}</button>
        
        <div class={classes!("animal-container")}>
          <div class={classes!("animal-graphic")} id="animal1">{"🐄"}</div>
          <div class={classes!("animal-graphic")} id="animal2">{"🐓"}</div>
          <div class={classes!("animal-graphic")} id="animal3">{"🐟"}</div>
          <div class={classes!("animal-graphic")} id="animal4">{"🍎"}</div>
          <div class={classes!("animal-graphic")} id="animal5">{"🍓"}</div>
          <div class={classes!("animal-graphic")} id="animal6">{"🧀"}</div>
          <div class={classes!("animal-graphic")} id="animal7">{"🥛"}</div>
          <div class={classes!("animal-graphic")} id="animal8">{"🍯"}</div>
        </div>
      </section>
    }
}
