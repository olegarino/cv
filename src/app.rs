use yew::prelude::*;
use yew_router::prelude::*;
use crate::route::Route;
use crate::components::navbar::Navbar;
use crate::pages::{Home, About, Resume, Contact};

fn switch(route: Route) -> Html {
    match route {
        Route::Home => html! { <Home /> },
        Route::About => html! { <About /> },
        Route::Resume => html! { <Resume /> },
        Route::Contact => html! { <Contact /> },
    }
}

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <BrowserRouter>
            <main>
                <Navbar />
                <Switch<Route> render={switch} />
            </main>
        </BrowserRouter>
    }
}