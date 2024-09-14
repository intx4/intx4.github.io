mod app;
mod contexts;
mod ui;
mod components;
mod routes;

use app::App;
use yew;

fn main() {
    yew::Renderer::<App>::new().render();
}
