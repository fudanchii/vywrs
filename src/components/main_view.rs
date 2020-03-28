use yew::prelude::*;

pub struct MainView;

impl Component for MainView {
    type Properties = ();
    type Message = ();

    fn create(_: Self::Properties, _link: ComponentLink<Self>) -> Self {
        MainView
    }

    fn update(&mut self, _: Self::Message) -> ShouldRender {
        true
    }

    fn view(&self) -> Html {
        html! {
            <h1>{"Howdy!"}</h1>
        }
    }
}
