use yew::prelude::*;

pub struct Landing;
impl Component for Landing {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }
    fn view(&self, _ctx: &Context<Self>) -> Html {
        html!(<div class="Landing">
            {Html::from_html_unchecked(include_str!("../../../assets/window/face/landing.svg").into())}
        </div>)
    }
}
