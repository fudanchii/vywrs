use crate::{
    components::{ListView, NavigationBar, TileView},
    listing::File,
    services::{BodyClassSetter, Config},
    VywrsMessage, VywrsMode, VywrsTheme,
};
use std::path::PathBuf;
use std::rc::Rc;
use yew::prelude::*;

pub struct Vywrs {
    link: ComponentLink<Self>,
    state: State,
    config: Rc<Config>,
}

struct State {
    path: PathBuf,
    theme: VywrsTheme,
    mode: VywrsMode,
    listing: Rc<Vec<File>>,
}

impl Vywrs {
    fn main_view(&self) -> Html {
        match self.state.mode {
            VywrsMode::List => html! {
                <ListView
                    theme=self.state.theme
                    listing=self.state.listing.clone()
                    path=self.state.path.clone()
                    config=self.config.clone() />
            },
            VywrsMode::Tile => html! {
                <TileView
                    theme=self.state.theme
                    listing=self.state.listing.clone()
                    path=self.state.path.clone()
                    config=self.config.clone() />
            },
        }
    }
}

impl Component for Vywrs {
    type Message = VywrsMessage;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        let state = State {
            path: PathBuf::from("/"),
            mode: VywrsMode::Tile,
            theme: VywrsTheme::Dark,
            listing: Rc::new(vec![]),
        };

        let config = Config::new().unwrap(); // 💣
        let config = Rc::new(config);

        Vywrs {
            config,
            link,
            state,
        }
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
                { self.main_view() }
            </>
        }
    }
}
