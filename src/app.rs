use yew::prelude::*;

pub struct App {}
impl yew::Component for App {
    type Message = ();
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        Self {}
    }
    fn view(&self, ctx: &Context<Self>) -> Html {
        html!(
            <>
            <button class="bubbly-button">{"Click me!"}</button>
            </>
        )
    }
    fn rendered(&mut self, ctx: &Context<Self>, first_render: bool) {
        if first_render {
            js_sys::eval(include_str!("../script.js")).unwrap();
        }
    }
}
