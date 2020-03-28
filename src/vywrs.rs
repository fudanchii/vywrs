use crate::{
    components::{MainView, NavigationBar},
    VywrsMode, VywrsTheme,
};
use std::path::PathBuf;
use yew::prelude::*;

pub struct Vywrs {
    link: ComponentLink<Self>,
    path: PathBuf,
    theme: VywrsTheme,
    mode: VywrsMode,
}

pub enum VywrsMessage {
    ChangeMode(VywrsMode),
    ChangeTheme(VywrsTheme),
}

impl Vywrs {}

impl Component for Vywrs {
    type Message = VywrsMessage;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        Vywrs {
            link,
            path: PathBuf::from("/"),
            mode: VywrsMode::Tile,
            theme: VywrsTheme::Dark,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            VywrsMessage::ChangeMode(new_mode) => {
                let prev_mode = self.mode;
                self.mode = new_mode;
                return prev_mode != new_mode;
            }
            VywrsMessage::ChangeTheme(new_theme) => {
                let prev_theme = self.theme;
                self.theme = new_theme;
                return prev_theme != new_theme;
            }
        }
    }

    fn view(&self) -> Html {
        let layout_change_callback = self
            .link
            .callback(|mode: VywrsMode| VywrsMessage::ChangeMode(mode));
        let theme_change_callback = self
            .link
            .callback(|theme: VywrsTheme| VywrsMessage::ChangeTheme(theme));

        html! {
            <>
                <NavigationBar
                    path=self.path.clone()
                    theme=self.theme
                    layout_changer=layout_change_callback
                    theme_changer=theme_change_callback />
                <MainView />
            </>
        }
    }
}
