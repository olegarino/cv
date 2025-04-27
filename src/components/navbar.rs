use yew::prelude::*;
use yew_router::prelude::*;
use crate::route::Route;

#[function_component(Navbar)]
pub fn navbar() -> Html {
    html! {
        <nav class="navbar">
            <p class="name-nav-bar">{"Oleg Tchernov / Fullstack Java Developer"}</p>
            <Link<Route> classes="nav-link" to={Route::Home}>{ "Home" }</Link<Route>>
            <Link<Route> classes="nav-link" to={Route::About}>{ "About Me" }</Link<Route>>
            <Link<Route> classes="nav-link" to={Route::Resume}>{ "Resume" }</Link<Route>>
            <Link<Route> classes="nav-link" to={Route::Contact}>{ "Contact" }</Link<Route>>
        </nav>
    }
}