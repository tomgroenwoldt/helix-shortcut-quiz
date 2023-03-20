mod app;
mod constants;
mod description;
mod gif;
mod help;
mod shortcut;

use app::App;

fn main() {
    yew::Renderer::<App>::new().render();
}
