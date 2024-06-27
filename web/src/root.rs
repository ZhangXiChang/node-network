use yew::prelude::*;

use crate::window::{
    eyebrow::EyebrowProperties,
    face::{landing::Landing, FaceProperties},
    Window,
};

#[function_component]
pub fn Root() -> Html {
    let menubar_properties = EyebrowProperties {
        maximize_window_icon: use_state(|| {
            Html::from_html_unchecked(
                include_str!("../assets/window/eyebrow/window-maximize.svg").into(),
            )
        }),
    };
    let face_properties = FaceProperties {
        face_content: use_state(|| html!(<Landing/>)),
    };
    html!(<Window {menubar_properties}{face_properties}/>)
}
