use yew::prelude::*;

#[function_component(Resume)]
pub fn resume() -> Html {
    html! {
        <div class="resume-page">
            <h1>{"My Resume"}</h1>
            <p>{"Here you can add your professional experience, skills, and education."}</p>
        </div>
    }
}