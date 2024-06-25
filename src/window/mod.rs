pub mod eyebrow;
pub mod face;
pub mod jaw;

use eyebrow::{Eyebrow, EyebrowProperties};
use face::Face;
use jaw::Jaw;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct WindowProperties {
    pub menubar_properties: EyebrowProperties,
}
pub struct Window;
impl Component for Window {
    type Message = ();
    type Properties = WindowProperties;

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }
    fn view(&self, ctx: &Context<Self>) -> Html {
        let maximize_window_icon = ctx.props().menubar_properties.maximize_window_icon.clone();
        html!(<div class="Window">
            <Eyebrow {maximize_window_icon}/>
            <Face/>
            <Jaw/>
        </div>)
    }
}
