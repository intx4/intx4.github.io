mod app;
mod blog;
mod components;
mod contexts;
mod home;
mod routes;
mod ui;

use app::App;
use yew;

fn main() {
    yew::Renderer::<App>::new().render();
}
