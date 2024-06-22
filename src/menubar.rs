use wasm_bindgen_futures::spawn_local;
use yew::{prelude::*, virtual_dom::VNode};

use crate::invoke;

#[derive(Properties, PartialEq)]
pub struct MenubarProperties {
    pub maximize_window_icon: UseStateHandle<VNode>,
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
            let maximize_window_icon = ctx.props().maximize_window_icon.clone();
            move |_| {
                invoke::maximize_window();
                spawn_local({
                    let maximize_window_icon = maximize_window_icon.clone();
                    async move {
                        if invoke::window_is_maximized().await {
                            maximize_window_icon.set(Html::from_html_unchecked(
                                include_str!("../assets/window-restore.svg").into(),
                            ));
                        } else {
                            maximize_window_icon.set(Html::from_html_unchecked(
                                include_str!("../assets/window-maximize.svg").into(),
                            ));
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
                    {Html::from_html_unchecked(include_str!("../assets/window-minimize.svg").into())}
                </div>
                <div class="button" onclick={maximize_window}>
                    {(*ctx.props().maximize_window_icon).clone()}
                </div>
                <div class="button" id="button-close" onclick={close_window}>
                    {Html::from_html_unchecked(include_str!("../assets/window-close.svg").into())}
                </div>
            </div>
            </>
        )
    }
}
