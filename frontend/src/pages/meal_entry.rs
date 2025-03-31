use yew::prelude::*;
use yew_router::prelude::*;
use stylist::yew::styled_component;
use web_sys::{HtmlInputElement, HtmlSelectElement};
use chrono::{DateTime, Local, Utc};
use uuid::Uuid;

use crate::api;
use crate::app::Route;
use crate::models::{FoodItem, Meal, MealItem};
use crate::styles::{card, primary_button, secondary_button, colors};
use crate::components::food_item_card::FoodItemCard;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub id: Option<String>,
}

#[styled_component(MealEntryPage)]
pub fn meal_entry_page(props: &Props) -> Html {
    let navigator = use_navigator().unwrap();
    let is_loading = use_state(|| true);
    let is_submitting = use_state(|| false);
    let error_message = use_state(|| String::new());
    let foods = use_state(|| Vec::<FoodItem>::new());
    let selected_foods = use_state(|| Vec::<(String, f32)>::new()); // (food_id, amount)
    
    let meal_name = use_state(|| String::new());
    let meal_date = use_state(|| Local::now().format("%Y-%m-%d").to_string());
    let meal_time = use_state(|| Local::now().format("%H:%M").to_string());
    let meal_notes = use_state(|| String::new());
    
    let is_edit_mode = props.id.is_some();
    let selected_food_id = use_state(|| String::new());
    let selected_food_amount = use_state(|| 100.0);
    
    // Load food database
    {
        let foods = foods.clone();
        let is_loading = is_loading.clone();
        
        use_effect_with_deps(
            move |_| {
                let foods = foods.clone();
                let is_loading = is_loading.clone();
                
                wasm_bindgen_futures::spawn_local(async move {
                    match api::get_foods().await {
                        Ok(fetched_foods) => {
                            foods.set(fetched_foods);
                        }
                        Err(e) => {
                            log::error!("Failed to load foods: {:?}", e);
                        }
                    }
                    
                    is_loading.set(false);
                });
                
                || ()
            },
            (),
        );
    }
    
    // Load meal if in edit mode
    {
        let props_id = props.id.clone();
        let meal_name = meal_name.clone();
        let meal_date = meal_date.clone();
        let meal_time = meal_time.clone();
        let meal_notes = meal_notes.clone();
        let selected_foods = selected_foods.clone();
        
        use_effect_with_deps(
            move |_| {
                if let Some(id) = props_id {
                    // TODO: Implement loading meal for editing
                    // This would fetch the meal by ID and populate the form
                    log::info!("Loading meal with ID: {}", id);
                    
                    // Use these cloned values when implementing
                    let _meal_name = meal_name;
                    let _meal_date = meal_date;
                    let _meal_time = meal_time;
                    let _meal_notes = meal_notes;
                    let _selected_foods = selected_foods;
                }
                
                || ()
            },
            (),
        );
    }
    
    // Handle food selection
    let on_food_select = {
        let selected_food_id = selected_food_id.clone();
        
        Callback::from(move |e: Event| {
            let select: HtmlSelectElement = e.target_unchecked_into();
            selected_food_id.set(select.value());
        })
    };
    
    // Handle food amount change
    let on_amount_change = {
        let selected_food_amount = selected_food_amount.clone();
        
        Callback::from(move |e: Event| {
            let input: HtmlInputElement = e.target_unchecked_into();
            if let Ok(value) = input.value().parse::<f32>() {
                selected_food_amount.set(value);
            }
        })
    };
    
    // Handle adding food to meal
    let on_add_food = {
        let selected_food_id = selected_food_id.clone();
        let selected_food_amount = selected_food_amount.clone();
        let selected_foods = selected_foods.clone();
        let foods = foods.clone();
        
        Callback::from(move |e: MouseEvent| {
            e.prevent_default();
            
            let food_id = (*selected_food_id).clone();
            if food_id.is_empty() {
                return;
            }
            
            // Check if the food exists in database
            if !foods.iter().any(|f| f.id == food_id) {
                return;
            }
            
            let amount = *selected_food_amount;
            
            let mut updated_foods = (*selected_foods).clone();
            
            // Check if already added, update amount if so
            let mut found = false;
            for (id, amt) in updated_foods.iter_mut() {
                if *id == food_id {
                    *amt = amount;
                    found = true;
                    break;
                }
            }
            
            // Add new entry if not found
            if !found {
                updated_foods.push((food_id, amount));
            }
            
            selected_foods.set(updated_foods);
        })
    };
    
    // Handle removing food from meal
    let on_remove_food = {
        let selected_foods = selected_foods.clone();
        
        Callback::from(move |food_id: String| {
            let mut updated_foods = (*selected_foods).clone();
            updated_foods.retain(|(id, _)| id != &food_id);
            selected_foods.set(updated_foods);
        })
    };
    
    // Handle form input changes
    let on_name_change = {
        let meal_name = meal_name.clone();
        
        Callback::from(move |e: Event| {
            let input: HtmlInputElement = e.target_unchecked_into();
            meal_name.set(input.value());
        })
    };
    
    let on_date_change = {
        let meal_date = meal_date.clone();
        
        Callback::from(move |e: Event| {
            let input: HtmlInputElement = e.target_unchecked_into();
            meal_date.set(input.value());
        })
    };
    
    let on_time_change = {
        let meal_time = meal_time.clone();
        
        Callback::from(move |e: Event| {
            let input: HtmlInputElement = e.target_unchecked_into();
            meal_time.set(input.value());
        })
    };
    
    let on_notes_change = {
        let meal_notes = meal_notes.clone();
        
        Callback::from(move |e: Event| {
            let input: HtmlInputElement = e.target_unchecked_into();
            meal_notes.set(input.value());
        })
    };
    
    // Handle form submission
    let on_submit = {
        let meal_name = meal_name.clone();
        let meal_date = meal_date.clone();
        let meal_time = meal_time.clone();
        let meal_notes = meal_notes.clone();
        let selected_foods = selected_foods.clone();
        let foods = foods.clone();
        let navigator = navigator.clone();
        let is_submitting = is_submitting.clone();
        let error_message = error_message.clone();
        let props_id = props.id.clone();
        
        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();
            
            let meal_name_val = (*meal_name).clone();
            let meal_date_val = (*meal_date).clone();
            let meal_time_val = (*meal_time).clone();
            let meal_notes_val = (*meal_notes).clone();
            let selected_foods_val = (*selected_foods).clone();
            let _foods_val = (*foods).clone(); // Will be used later for nutrition calculations
            let navigator = navigator.clone();
            let is_submitting = is_submitting.clone();
            let error_message = error_message.clone();
            let props_id = props_id.clone();
            
            // Validation
            if meal_name_val.is_empty() {
                error_message.set("Please enter a meal name".to_string());
                return;
            }
            
            if selected_foods_val.is_empty() {
                error_message.set("Please add at least one food item".to_string());
                return;
            }
            
            // Parse datetime
            let datetime_str = format!("{}T{}:00", meal_date_val, meal_time_val);
            let datetime = match DateTime::parse_from_rfc3339(&datetime_str) {
                Ok(dt) => dt.with_timezone(&Utc),
                Err(_) => {
                    error_message.set("Invalid date or time format".to_string());
                    return;
                }
            };
            
            // Create meal items
            let meal_items: Vec<MealItem> = selected_foods_val
                .iter()
                .map(|(food_id, amount)| MealItem {
                    food_id: food_id.clone(),
                    amount: *amount,
                })
                .collect();
            
            // Create meal object
            let meal = Meal {
                id: props_id.unwrap_or_else(|| Uuid::new_v4().to_string()),
                user_id: "current_user".to_string(), // Will be set by backend
                name: meal_name_val,
                timestamp: datetime,
                items: meal_items,
                notes: if meal_notes_val.is_empty() { None } else { Some(meal_notes_val) },
            };
            
            is_submitting.set(true);
            
            // Submit meal to API
            wasm_bindgen_futures::spawn_local(async move {
                let result = api::add_meal(&meal).await;
                
                match result {
                    Ok(_) => {
                        // Redirect to meal list
                        navigator.push(&Route::MealList);
                    }
                    Err(e) => {
                        log::error!("Failed to save meal: {:?}", e);
                        error_message.set("Failed to save meal. Please try again.".to_string());
                    }
                }
                
                is_submitting.set(false);
            });
        })
    };
    
    // Prepare food options for select
    let food_options = foods.iter().map(|food| {
        html! {
            <option value={food.id.clone()}>
                {format!("{} ({}g)", food.name, food.serving_size)}
            </option>
        }
    }).collect::<Html>();
    
    // Get selected food details
    let selected_food_details = {
        let selected_id = (*selected_food_id).clone();
        foods.iter()
            .find(|f| f.id == selected_id)
            .cloned()
    };
    
    // Get food list for selected items
    let food_list = {
        let foods_ref = foods.clone();
        let selected_foods_ref = selected_foods.clone();
        
        selected_foods_ref.iter().map(|(food_id, amount)| {
            let food = foods_ref.iter().find(|f| f.id == *food_id).cloned();
            
            if let Some(food_item) = food {
                let food_id = food_item.id.clone();
                let on_remove = {
                    let food_id = food_id.clone();
                    let on_remove_food = on_remove_food.clone();
                    Callback::from(move |_| {
                        on_remove_food.emit(food_id.clone());
                    })
                };
                
                html! {
                    <FoodItemCard 
                        food={food_item}
                        amount={*amount}
                        on_remove={on_remove}
                    />
                }
            } else {
                html! {}
            }
        }).collect::<Html>()
    };
    
    let card_style = card();
    let primary_btn = primary_button();
    let secondary_btn = secondary_button();
    
    html! {
        <div class="container" style="padding-top: 2rem; padding-bottom: 2rem;">
            <h1>{if is_edit_mode { "Edit Meal" } else { "Add New Meal" }}</h1>
            
            if *is_loading {
                <p>{"Loading food database..."}</p>
            } else {
                <form onsubmit={on_submit}>
                    <div class={card_style.clone()}>
                        <h2>{"Meal Details"}</h2>
                        
                        if !(*error_message).is_empty() {
                            <div style={format!("background-color: {}; color: white; padding: 1rem; margin-bottom: 1rem; border-radius: 4px;", colors::ERROR)}>
                                { &*error_message }
                            </div>
                        }
                        
                        <div style="margin-bottom: 1.5rem;">
                            <label for="meal_name" style="display: block; margin-bottom: 0.5rem; font-weight: 500;">
                                {"Meal Name"}
                            </label>
                            <input
                                id="meal_name"
                                type="text"
                                value={(*meal_name).clone()}
                                onchange={on_name_change}
                                placeholder="e.g., Breakfast, Lunch, Dinner, Snack"
                            />
                        </div>
                        
                        <div style="display: flex; gap: 1rem; margin-bottom: 1.5rem;">
                            <div style="flex: 1;">
                                <label for="meal_date" style="display: block; margin-bottom: 0.5rem; font-weight: 500;">
                                    {"Date"}
                                </label>
                                <input
                                    id="meal_date"
                                    type="date"
                                    value={(*meal_date).clone()}
                                    onchange={on_date_change}
                                />
                            </div>
                            <div style="flex: 1;">
                                <label for="meal_time" style="display: block; margin-bottom: 0.5rem; font-weight: 500;">
                                    {"Time"}
                                </label>
                                <input
                                    id="meal_time"
                                    type="time"
                                    value={(*meal_time).clone()}
                                    onchange={on_time_change}
                                />
                            </div>
                        </div>
                        
                        <div style="margin-bottom: 1.5rem;">
                            <label for="meal_notes" style="display: block; margin-bottom: 0.5rem; font-weight: 500;">
                                {"Notes (Optional)"}
                            </label>
                            <textarea
                                id="meal_notes"
                                rows="3"
                                value={(*meal_notes).clone()}
                                onchange={on_notes_change}
                                placeholder="Any notes about this meal..."
                                style="width: 100%; padding: 0.75rem; border: 3px solid #000;"
                            ></textarea>
                        </div>
                    </div>
                    
                    <div class={card_style.clone()}>
                        <h2>{"Add Food Items"}</h2>
                        
                        <div style="display: flex; gap: 1rem; margin-bottom: 1.5rem;">
                            <div style="flex: 2;">
                                <label for="food_select" style="display: block; margin-bottom: 0.5rem; font-weight: 500;">
                                    {"Select Food"}
                                </label>
                                <select
                                    id="food_select"
                                    value={(*selected_food_id).clone()}
                                    onchange={on_food_select}
                                >
                                    <option value="" disabled=true selected=true>{"-- Select a food --"}</option>
                                    { food_options }
                                </select>
                            </div>
                            <div style="flex: 1;">
                                <label for="food_amount" style="display: block; margin-bottom: 0.5rem; font-weight: 500;">
                                    {"Amount (g)"}
                                </label>
                                <input
                                    id="food_amount"
                                    type="number"
                                    min="1"
                                    step="1"
                                    value={(*selected_food_amount).to_string()}
                                    onchange={on_amount_change}
                                />
                            </div>
                            <div style="flex: 0; align-self: flex-end; margin-bottom: 3px;">
                                <button
                                    type="button"
                                    class={primary_btn.clone()}
                                    onclick={on_add_food}
                                    disabled={(*selected_food_id).is_empty()}
                                >
                                    {"Add"}
                                </button>
                            </div>
                        </div>
                        
                        if let Some(food) = selected_food_details {
                            <div style="margin-bottom: 1.5rem; padding: 1rem; background-color: #f0f0f0; border: 2px dashed #ccc;">
                                <h3>{&food.name}</h3>
                                <p>{"Per "}{food.serving_size}{"g serving:"}</p>
                                <ul style="list-style: none; padding: 0; margin-top: 0.5rem;">
                                    <li>{format!("Calories: {:.0} kcal", food.macros.calories)}</li>
                                    <li>{format!("Protein: {:.1}g", food.macros.protein)}</li>
                                    <li>{format!("Fat: {:.1}g", food.macros.fat)}</li>
                                    <li>{format!("Carbs: {:.1}g", food.macros.carbs)}</li>
                                </ul>
                            </div>
                        }
                        
                        <div>
                            <h3>{"Selected Foods"}</h3>
                            
                            if selected_foods.is_empty() {
                                <p style="font-style: italic; color: #666; margin: 1rem 0;">
                                    {"No foods added yet. Select foods above to add them to your meal."}
                                </p>
                            } else {
                                <div style="margin-top: 1rem;">
                                    { food_list }
                                </div>
                            }
                        </div>
                    </div>
                    
                    <div style="display: flex; justify-content: space-between; margin-top: 2rem;">
                        <button
                            type="button"
                            class={secondary_btn}
                            onclick={
                                let navigator = navigator.clone();
                                Callback::from(move |_| {
                                    navigator.push(&Route::MealList);
                                })
                            }
                        >
                            {"Cancel"}
                        </button>
                        
                        <button
                            type="submit"
                            class={primary_btn}
                            disabled={*is_submitting}
                        >
                            if *is_submitting {
                                {"Saving..."}
                            } else if is_edit_mode {
                                {"Update Meal"}
                            } else {
                                {"Save Meal"}
                            }
                        </button>
                    </div>
                </form>
            }
        </div>
    }
}
