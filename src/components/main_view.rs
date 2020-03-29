use crate::VywrsTheme;
use yew::prelude::*;

pub struct MainView;

#[derive(Copy, Clone, Properties, PartialEq)]
pub struct Props {
    pub theme: VywrsTheme,
}

impl Component for MainView {
    type Properties = Props;
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
