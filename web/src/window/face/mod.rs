pub mod demo;
pub mod landing;

use demo::Demo;
use wasm_bindgen_futures::spawn_local;
use yew::{prelude::*, virtual_dom::VNode};

use crate::invoke;

#[derive(Properties, PartialEq)]
pub struct FaceProperties {
    pub face_content: UseStateHandle<VNode>,
}
pub struct Face;
impl Component for Face {
    type Message = ();
    type Properties = FaceProperties;

    fn create(ctx: &Context<Self>) -> Self {
        spawn_local({
            let face_content = ctx.props().face_content.clone();
            async move {
                invoke::connect_server().await;
                face_content.set(html!(<Demo/>));
            }
        });
        Self
    }
    fn view(&self, ctx: &Context<Self>) -> Html {
        html!(<div class="Face">
            {(*ctx.props().face_content).clone()}
        </div>)
    }
}
