mod app;
mod components;
mod models;

use app::App;

fn main() {
    yew::Renderer::<App>::new().render();
}
