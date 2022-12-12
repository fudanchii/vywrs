use crate::{
    components::{ListView, NavigationBar, TileView},
    listing::File,
    services::{glightbox, Config, GLightbox},
    vywrs::{VywrsMode, VywrsTheme},
};

use std::rc::Rc;
use yew::prelude::*;
use gloo_utils::{body, document};

#[derive(Default)]
pub struct Vywrs {
    config: Rc<Config>,
    theme: VywrsTheme,
    mode: VywrsMode,
    lightbox: Option<glightbox::Instance>,
}

#[derive(Default, PartialEq, Properties)]
pub struct VywrsProps {
    pub location: AttrValue,
    pub listing: Rc<Vec<File>>,
    #[prop_or_default]
    pub is_fetching: bool,
}

pub enum VywrsMessage {
    ChangeMode(VywrsMode),
    ChangeTheme(VywrsTheme),
}

impl Vywrs {
    fn main_view(&self, ctx: &Context<Self>) -> Html {
        match self.mode {
            VywrsMode::List => html! {
                <ListView
                    theme={self.theme}
                    listing={ctx.props().listing.clone()}
                    path={ctx.props().location.clone()}
                    config={self.config.clone()} />
            },
            VywrsMode::Tile => html! {
                <TileView
                    theme={self.theme}
                    listing={ctx.props().listing.clone()}
                    path={ctx.props().location.clone()}
                    config={self.config.clone()} />
            },
        }
    }
}

impl Component for Vywrs {
    type Message = VywrsMessage;
    type Properties = VywrsProps;

    fn create(_ctx: &Context<Self>) -> Self {
        let config = Config::new().unwrap();
        let app = Self {
            config: Rc::new(config),
            mode: VywrsMode::Tile,
            theme: VywrsTheme::Dark,
            lightbox: None,
        };

        app
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        macro_rules! rerender_if_changed {
            ($a:ident, $b:ident) => {{
                if self.$a != $b {
                    self.$a = $b;
                    return true;
                }
                false
            }};
        }

        match msg {
            VywrsMessage::ChangeMode(new_mode) => {
                rerender_if_changed!(mode, new_mode)
            }
            VywrsMessage::ChangeTheme(new_theme) => {
                rerender_if_changed!(theme, new_theme)
            }
        }
    }

    fn rendered(&mut self, _ctx: &Context<Self>, _first_render: bool) {
        if let Some(lb) = self.lightbox.take() {
            lb.destroy();
        }
        self.lightbox.replace(GLightbox());
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let link = ctx.link();
        let layout_change_callback = link.callback(VywrsMessage::ChangeMode);
        let theme_change_callback = link.callback(VywrsMessage::ChangeTheme);

        body().set_class_name(&self.theme);
        document().set_title(&ctx.props().location);

        html! {
            <>
                <NavigationBar
                    is_fetching={ctx.props().is_fetching}
                    path={ctx.props().location.clone()}
                    theme={self.theme}
                    layout_changer={layout_change_callback}
                    theme_changer={theme_change_callback} />
                { self.main_view(ctx) }
            </>
        }
    }
}
