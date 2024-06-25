use wasm_bindgen_futures::spawn_local;
use yew::{prelude::*, virtual_dom::VNode};

use crate::invoke;

#[derive(Properties, PartialEq)]
pub struct EyebrowProperties {
    pub maximize_window_icon: UseStateHandle<VNode>,
}
pub struct Eyebrow;
impl Component for Eyebrow {
    type Message = ();
    type Properties = EyebrowProperties;

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
                                include_str!("../../assets/window/eyebrow/window-restore.svg")
                                    .into(),
                            ));
                        } else {
                            maximize_window_icon.set(Html::from_html_unchecked(
                                include_str!("../../assets/window/eyebrow/window-maximize.svg")
                                    .into(),
                            ));
                        }
                    }
                });
            }
        };
        let close_window = |_| {
            invoke::close_window();
        };
        let github_link = |_| {
            invoke::open("https://github.com/ZhangXiChang");
        };
        let setting_button = |_| {};
        html!(
            <div class="Eyebrow" data-tauri-drag-region="">
                <div class="MenuBar" data-tauri-drag-region="">
                    <div class="GithubLogo" onclick={github_link}>
                        {Html::from_html_unchecked(include_str!("../../assets/window/eyebrow/github-loop.svg").into())}
                    </div>
                    <div class="SettingButton" onclick={setting_button}>
                        {Html::from_html_unchecked(include_str!("../../assets/window/eyebrow/setting.svg").into())}
                    </div>
                    <div class="Title" data-tauri-drag-region="">
                        <p>{"牌佬助手"}</p>
                    </div>
                </div>
                <div class="ControlBar" data-tauri-drag-region="">
                    <div class="Button" onclick={minimize_window}>
                        {Html::from_html_unchecked(include_str!("../../assets/window/eyebrow/window-minimize.svg").into())}
                    </div>
                    <div class="Button" onclick={maximize_window}>
                        {(*ctx.props().maximize_window_icon).clone()}
                    </div>
                    <div class="Button" onclick={close_window}>
                        {Html::from_html_unchecked(include_str!("../../assets/window/eyebrow/window-close.svg").into())}
                    </div>
                </div>
            </div>
        )
    }
}
