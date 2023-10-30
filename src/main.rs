mod api;
mod app;
mod models;
mod pages;
mod permission_verification;
mod route;

use app::App;

fn main() {
    yew::Renderer::<App>::new().render();
}
