mod app;
mod components;
mod route;
mod pages;
use app::App;

fn main() {
    yew::Renderer::<App>::new().render();
}
