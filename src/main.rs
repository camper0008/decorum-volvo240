mod app;
mod models;
mod pages;
mod route;

use app::App;

fn main() {
    yew::Renderer::<App>::new().render();
}
