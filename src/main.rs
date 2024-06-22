mod invoke;
mod menubar;
mod root;

use root::Root;

fn main() {
    yew::Renderer::<Root>::new().render();
}
