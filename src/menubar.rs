use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;

use crate::invoke;

#[derive(Properties, PartialEq)]
pub struct MenubarProperties {
    pub window_maximize_icon_path: UseStateHandle<String>,
}
pub struct Menubar;
impl Component for Menubar {
    type Message = ();
    type Properties = MenubarProperties;

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }
    fn view(&self, ctx: &Context<Self>) -> Html {
        let minimize_window = |_| {
            invoke::minimize_window();
        };
        let maximize_window = {
            let window_maximize_icon_path = ctx.props().window_maximize_icon_path.clone();
            move |_| {
                invoke::maximize_window();
                spawn_local({
                    let window_maximize_icon_path = window_maximize_icon_path.clone();
                    async move {
                        if invoke::window_is_maximized().await {
                            window_maximize_icon_path.set(
                                "https://api.iconify.design/mdi:window-restore.svg".to_string(),
                            );
                        } else {
                            window_maximize_icon_path.set(
                                "https://api.iconify.design/mdi:window-maximize.svg".to_string(),
                            );
                        }
                    }
                });
            }
        };
        let close_window = |_| {
            invoke::close_window();
        };
        html!(
            <>
            <div class="menubar" data-tauri-drag-region="">
                <div class="button" onclick={minimize_window}>
                    <img src={"https://api.iconify.design/mdi:window-minimize.svg"} />
                </div>
                <div class="button" onclick={maximize_window}>
                    <img src={(*ctx.props().window_maximize_icon_path).clone()} />
                </div>
                <div class="button" id="button-close" onclick={close_window}>
                    <img src={"https://api.iconify.design/mdi:close.svg"} />
                </div>
            </div>
            </>
        )
    }
}
