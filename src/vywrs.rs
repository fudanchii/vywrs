use crate::{
    components::{MainView, NavigationBar},
    services::BodyClassSetter,
    VywrsMessage, VywrsMode, VywrsTheme,
};
use std::path::PathBuf;
use yew::prelude::*;

pub struct Vywrs {
    link: ComponentLink<Self>,
    state: State,
}

struct State {
    path: PathBuf,
    theme: VywrsTheme,
    mode: VywrsMode,
}

impl Component for Vywrs {
    type Message = VywrsMessage;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        let state = State {
            path: PathBuf::from("/"),
            mode: VywrsMode::Tile,
            theme: VywrsTheme::Dark,
        };

        Vywrs { link, state }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        macro_rules! rerender_if_changed {
            ($a:ident, $b:ident) => {{
                let prev = self.state.$a;
                self.state.$a = $b;
                return prev != $b;
            }};
        }

        match msg {
            VywrsMessage::ChangeMode(new_mode) => rerender_if_changed!(mode, new_mode),
            VywrsMessage::ChangeTheme(new_theme) => rerender_if_changed!(theme, new_theme),
        }
    }

    fn view(&self) -> Html {
        let layout_change_callback = self
            .link
            .callback(|mode: VywrsMode| VywrsMessage::ChangeMode(mode));
        let theme_change_callback = self
            .link
            .callback(|theme: VywrsTheme| VywrsMessage::ChangeTheme(theme));

        BodyClassSetter::set(&self.state.theme).unwrap();

        html! {
            <>
                <NavigationBar
                    path=self.state.path.clone()
                    theme=self.state.theme
                    layout_changer=layout_change_callback
                    theme_changer=theme_change_callback />
                <MainView theme=self.state.theme />
            </>
        }
    }
}
