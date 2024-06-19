mod app;
mod invoke;

use app::App;

fn main() {
    yew::Renderer::<App>::new().render();
}
