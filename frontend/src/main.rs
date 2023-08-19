mod api;
mod app;
mod components;
mod composables;
mod ctx;
mod pages;
mod router;
mod store;

use app::App;

fn main() {
    log::trace!("Initializing yew...");
    yew::Renderer::<App>::new().render();
}
