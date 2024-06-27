mod invoke;
mod root;
mod window;

use root::Root;

fn main() {
    yew::Renderer::<Root>::new().render();
}
