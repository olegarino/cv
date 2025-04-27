use yew::prelude::*;

#[function_component(About)]
pub fn about() -> Html {
    html! {
        <div class="about-page">
            <h1>{"About Me"}</h1>
            <p>{"Here you can write your personal introduction and background."}</p>
        </div>
    }
}