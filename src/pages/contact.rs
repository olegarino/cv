use yew::prelude::*;

#[function_component(Contact)]
pub fn contact() -> Html {
    html! {
        <div class="contact-page">
            <h1>{"Contact Me"}</h1>
            <p>{"Here you can add your contact information and a contact form."}</p>
        </div>
    }
}