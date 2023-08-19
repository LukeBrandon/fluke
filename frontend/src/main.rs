mod api;
mod app;
mod components;
mod pages;
mod router;
mod store;

use app::App;

fn main() {
    log::trace!("Initializing yew...");
    yew::Renderer::<App>::new().render();
}
