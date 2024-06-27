use yew::prelude::*;

pub struct Demo;
impl Component for Demo {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }
    fn view(&self, _ctx: &Context<Self>) -> Html {
        html!(<div class="Demo">
        </div>)
    }
}
