use yew::prelude::*;

#[function_component(Home)]
pub fn home() -> Html {
    html! {
        <div class="home-page">
            <h1>{"Welcome to My Portfolio"}</h1>
            <p>{"This is the home page of my personal website."}</p>
        </div>
    }
}