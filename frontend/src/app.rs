use yew::prelude::*;
use yew_router::prelude::*;

// Import your pages
use crate::pages::{
    HomePage, LoginPage, MealEntryPage, MealListPage, 
    NutrientReportPage, NotFoundPage, ProfilePage
};
use crate::components::layout::MainLayout;
use crate::components::auth_guard::AuthGuard;

// App routes
#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/login")]
    Login,
    #[at("/meals")]
    MealList,
    #[at("/meals/new")]
    NewMeal,
    #[at("/meals/:id")]
    MealDetail { id: String },
    #[at("/reports")]
    Reports,
    #[at("/profile")]
    Profile,
    #[not_found]
    #[at("/404")]
    NotFound,
}

// Define auth-protected routes
fn is_protected(route: &Route) -> bool {
    match route {
        Route::Login | Route::NotFound => false,
        _ => true,
    }
}

// Switch function to render the correct page based on route
fn switch(route: Route) -> Html {
    let route_clone = route.clone();
    
    if is_protected(&route) {
        html! {
            <AuthGuard>
                <MainLayout>
                    { render_route(route_clone) }
                </MainLayout>
            </AuthGuard>
        }
    } else {
        html! {
            <MainLayout>
                { render_route(route) }
            </MainLayout>
        }
    }
}

// Render the appropriate component for each route
fn render_route(route: Route) -> Html {
    match route {
        Route::Home => html! { <HomePage /> },
        Route::Login => html! { <LoginPage /> },
        Route::MealList => html! { <MealListPage /> },
        Route::NewMeal => html! { <MealEntryPage /> },
        Route::MealDetail { id } => html! { <MealEntryPage id={id} /> },
        Route::Reports => html! { <NutrientReportPage /> },
        Route::Profile => html! { <ProfilePage /> },
        Route::NotFound => html! { <NotFoundPage /> },
    }
}

// Main app component
#[function_component(App)]
pub fn app() -> Html {
    // Load global styles
    crate::styles::global_style();
    
    // Set up the app with routing
    html! {
        <BrowserRouter>
            <Switch<Route> render={switch} />
        </BrowserRouter>
    }
}
