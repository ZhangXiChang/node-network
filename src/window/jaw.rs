use yew::prelude::*;

pub struct Jaw;
impl Component for Jaw {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }
    fn view(&self, _ctx: &Context<Self>) -> Html {
        html!(<div class="Jaw">
        </div>)
    }
}
