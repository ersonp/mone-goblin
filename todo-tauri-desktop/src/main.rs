mod app;
mod components;
mod controllers;
mod helpers;
mod model;
mod state;
mod todo_api;

use app::App;

fn main() {
    yew::Renderer::<App>::new().render();
}
