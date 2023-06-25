mod app;
mod components;
mod pages;
mod store;
mod api;
mod router;

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    log::trace!("Initializing yew...");
    yew::Renderer::<app::App>::new().render();
}
