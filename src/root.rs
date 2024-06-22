use yew::prelude::*;

use crate::menubar::Menubar;

#[function_component]
pub fn Root() -> Html {
    let window_maximize_icon_path =
        use_state(|| "https://api.iconify.design/mdi:window-maximize.svg".to_string());
    html!(<div class="window">
        <Menubar {window_maximize_icon_path}/>
    </div>)
}
