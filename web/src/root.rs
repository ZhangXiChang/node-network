use yew::prelude::*;

use crate::window::{eyebrow::EyebrowProperties, Window};

#[function_component]
pub fn Root() -> Html {
    let menubar_properties = EyebrowProperties {
        maximize_window_icon: use_state(|| {
            Html::from_html_unchecked(
                include_str!("../assets/window/eyebrow/window-maximize.svg").into(),
            )
        }),
    };
    html!(<Window {menubar_properties}/>)
}
