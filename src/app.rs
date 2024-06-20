use yew::prelude::*;

use crate::invoke;

pub struct App {}
impl yew::Component for App {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {}
    }
    fn view(&self, _ctx: &Context<Self>) -> Html {
        html!(
            <>
            <button class="bubbly-button">{"点我吧！"}</button>
            </>
        )
    }
    fn rendered(&mut self, _ctx: &Context<Self>, first_render: bool) {
        if first_render {
            match js_sys::eval(include_str!("../script.js")) {
                Ok(_) => (),
                Err(_) => invoke::exit(),
            }
        }
    }
}
