mod app;
mod categories;
mod constants;
mod description;
mod gif;
mod help;
mod progress;
mod shortcut;
mod tutorial;

use app::App;

fn main() {
    yew::Renderer::<App>::new().render();
}
