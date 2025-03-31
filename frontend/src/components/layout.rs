use yew::prelude::*;
use yew_router::prelude::*;
use stylist::yew::styled_component;

use crate::app::Route;
use crate::styles::{colors, borders};
use crate::utils::storage::{is_logged_in, remove_token};

#[derive(Properties, PartialEq)]
pub struct Props {
    pub children: Children,
}

#[styled_component(MainLayout)]
pub fn main_layout(props: &Props) -> Html {
    let navigator = use_navigator().unwrap();
    let location = use_location().unwrap();
    let is_auth = use_state(|| is_logged_in());
    
    // Check authentication on location change
    {
        let is_auth = is_auth.clone();
        let path = location.path().to_string(); // Clone the path to avoid borrowing issues
        
        use_effect_with_deps(
            move |_| {
                is_auth.set(is_logged_in());
                || ()
            },
            path,
        );
    }
    
    // Handle logout
    let on_logout = {
        let navigator = navigator.clone();
        
        Callback::from(move |e: MouseEvent| {
            e.prevent_default();
            remove_token();
            navigator.push(&Route::Login);
        })
    };
    
    // Navigation link styles
    let nav_link_style = format!(
        "
        display: inline-block;
        padding: 0.75rem 1rem;
        color: {};
        text-decoration: none;
        font-weight: 700;
        font-family: 'Space Grotesk', sans-serif;
        border-bottom: 3px solid transparent;
        
        &:hover {{
            border-bottom: 3px solid {};
        }}
        ",
        colors::TEXT,
        colors::PRIMARY,
    );
    
    let active_nav_link_style = format!(
        "
        {}
        border-bottom: 3px solid {};
        ",
        nav_link_style,
        colors::PRIMARY,
    );
    
    // Create a nav link component with active class
    let nav_link = |to: Route, label: &str| {
        let is_active = location.path() == to.to_path();
        let style = if is_active { active_nav_link_style.clone() } else { nav_link_style.clone() };
        
        html! {
            <Link<Route> to={to} classes={classes!(style)}>
                {label}
            </Link<Route>>
        }
    };
    
    // Header styles
    let header_style = format!(
        "
        background-color: white;
        border-bottom: {} solid #000;
        padding: 0.5rem 0;
        box-shadow: {};
        position: sticky;
        top: 0;
        z-index: 100;
        ",
        borders::BORDER_WIDTH,
        borders::BOX_SHADOW,
    );
    
    html! {
        <>
            <header style={header_style}>
                <div class="container">
                    <div style="display: flex; justify-content: space-between; align-items: center;">
                        <div>
                            <h1 style="margin: 0; font-size: 1.75rem; color: #000;">
                                <Link<Route> to={Route::Home} classes={classes!("text-decoration-none")} >
                                    {"AB MACROS"}
                                </Link<Route>>
                            </h1>
                        </div>
                        
                        if *is_auth {
                            <nav>
                                <ul style="list-style: none; display: flex; margin: 0; padding: 0;">
                                    <li>
                                        {nav_link(Route::Home, "Dashboard")}
                                    </li>
                                    <li>
                                        {nav_link(Route::MealList, "Meals")}
                                    </li>
                                    <li>
                                        {nav_link(Route::Reports, "Reports")}
                                    </li>
                                    <li>
                                        {nav_link(Route::Profile, "Profile")}
                                    </li>
                                    <li>
                                        <a 
                                            href="#" 
                                            onclick={on_logout}
                                            style={nav_link_style.clone()}
                                        >
                                            {"Logout"}
                                        </a>
                                    </li>
                                </ul>
                            </nav>
                        }
                    </div>
                </div>
            </header>
            
            <main>
                {for props.children.iter()}
            </main>
            
            <footer style="margin-top: 4rem; padding: 2rem 0; background-color: #f7f7f7; border-top: 1px solid #eaeaea;">
                <div class="container" style="text-align: center;">
                    <p style="color: #666;">{"© 2025 AB Macros - Animal-Based Diet Tracker"}</p>
                </div>
            </footer>
        </>
    }
}
