use yew::prelude::*;

use crate::menubar::Menubar;

#[function_component]
pub fn Root() -> Html {
    let maximize_window_icon = use_state(|| {
        Html::from_html_unchecked(include_str!("../assets/window-maximize.svg").into())
    });
    html!(<div class="window">
        <Menubar {maximize_window_icon}/>
    </div>)
}
