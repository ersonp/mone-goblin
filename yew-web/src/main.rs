mod app;
mod components;
mod controllers;
mod inv_api;
mod state;

use app::App;

fn main() {
    yew::Renderer::<App>::new().render();
}
