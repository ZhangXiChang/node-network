use yew::prelude::*;

pub struct Face;
impl Component for Face {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }
    fn view(&self, _ctx: &Context<Self>) -> Html {
        let face_content = html!(<div class="Landing">
            {Html::from_html_unchecked(include_str!("../../assets/window/face/landing.svg").into())}
        </div>);
        html!(<div class="Face">
            {face_content}
        </div>)
    }
}
