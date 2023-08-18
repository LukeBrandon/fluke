mod api;
mod app;
mod components;
mod pages;
mod router;
mod store;

use app::App;

fn main() {
    yew::Renderer::<App>::new().render();
}
