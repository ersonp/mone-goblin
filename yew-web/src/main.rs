mod app;
mod components;
mod controllers;
mod inv_api;
mod state;

use app::App;

fn main() {
    wasm_logger::init(wasm_logger::Config::default());

    yew::Renderer::<App>::new().render();
}
