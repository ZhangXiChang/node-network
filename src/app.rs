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
        let minimize_window = |_| {
            invoke::minimize_window();
        };
        let maximize_window = |_| {
            invoke::maximize_window();
        };
        let close_window = |_| {
            invoke::close_window();
        };
        html!(
            <>
            <div class="titlebar" data-tauri-drag-region="">
                <div class="titlebar-button" onclick={minimize_window}>
                    <img src="https://api.iconify.design/mdi:window-minimize.svg"/>
                </div>
                <div
                class="titlebar-button" onclick={maximize_window}>
                    <img src="https://api.iconify.design/mdi:window-maximize.svg"/>
                </div>
                <div class="titlebar-button" onclick={close_window}>
                    <img src="https://api.iconify.design/mdi:close.svg"/>
                </div>
            </div>
            <canvas id="canvas"></canvas>
            </>
        )
    }
    fn rendered(&mut self, _ctx: &Context<Self>, first_render: bool) {
        if first_render {
            match js_sys::eval(include_str!("../script.js")) {
                Ok(_) => (),
                Err(_) => invoke::close_window(),
            }
        }
    }
}
