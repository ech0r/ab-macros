use yew::prelude::*;
use yew_router::prelude::*;

use crate::app::Route;
use crate::utils::storage::get_token;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub children: Children,
}

#[function_component(AuthGuard)]
pub fn auth_guard(props: &Props) -> Html {
    let navigator = use_navigator().unwrap();
    
    // Check if user is authenticated
    let is_authenticated = get_token().is_some();
    
    // If not authenticated, redirect to login
    if !is_authenticated {
        navigator.push(&Route::Login);
        return html! {};
    }
    
    // If authenticated, render children
    html! {
        <>
            { for props.children.iter() }
        </>
    }
}
