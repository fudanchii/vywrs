use crate::{
    components::{ListView, NavigationBar, TileView},
    listing::File,
    services::{glightbox, BodyClassSetter, Config, GLightbox, TitleSetter},
    vywrs::{VywrsMode, VywrsTheme},
};

use gloo_net::http::Request;
use std::rc::Rc;
use yew::prelude::*;

#[derive(Default)]
pub struct Vywrs {
    config: Rc<Config>,
    path: AttrValue,
    theme: VywrsTheme,
    mode: VywrsMode,
    listing: Rc<Vec<File>>,
    lightbox: Option<glightbox::Instance>,
}

#[derive(Default, PartialEq, Properties)]
pub struct VywrsProps {
    pub location: AttrValue,
}

pub enum VywrsMessage {
    ChangeMode(VywrsMode),
    ChangeTheme(VywrsTheme),
    UpdateListing(Vec<File>),
    FetchListing,
    FetchFailed,
}

impl Vywrs {
    fn main_view(&self) -> Html {
        match self.mode {
            VywrsMode::List => html! {
                <ListView
                    theme={self.theme}
                    listing={self.listing.clone()}
                    path={self.path.clone()}
                    config={self.config.clone()} />
            },
            VywrsMode::Tile => html! {
                <TileView
                    theme={self.theme}
                    listing={self.listing.clone()}
                    path={self.path.clone()}
                    config={self.config.clone()} />
            },
        }
    }

    fn do_fetch_listing(&mut self, ctx: &Context<Self>) -> bool {
        let hashloc = Config::url_decode(&ctx.props().location);
        let endpoint = self.config.list_endpoint(&hashloc);

        ctx.link().send_future(async move {
            let listing = Request::get(&endpoint)
                .send()
                .await
                .unwrap()
                .json::<Vec<File>>()
                .await
                .unwrap();
            VywrsMessage::UpdateListing(listing)
        });

        self.path = hashloc.into();
        false
    }

    fn do_update_listing(&mut self, new_listing: Vec<File>) -> bool {
        let prev = self.listing.clone();
        self.listing = Rc::new(new_listing);
        prev != self.listing
    }
}

impl Component for Vywrs {
    type Message = VywrsMessage;
    type Properties = VywrsProps;

    fn create(ctx: &Context<Self>) -> Self {
        let config = Config::new().unwrap();
        let app = Self {
            config: Rc::new(config),
            path: ctx.props().location.clone(),
            mode: VywrsMode::Tile,
            theme: VywrsTheme::Dark,
            listing: Rc::new(vec![]),
            lightbox: None,
        };

        ctx.link().send_message(VywrsMessage::FetchListing);

        app
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
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
            VywrsMessage::UpdateListing(new_listing) => self.do_update_listing(new_listing),
            VywrsMessage::FetchListing => self.do_fetch_listing(ctx),
            VywrsMessage::FetchFailed => false,
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

        let location = AttrValue::from(Config::url_decode(&ctx.props().location));
        if location != self.path {
            link.send_message(VywrsMessage::FetchListing);
        }

        BodyClassSetter::set(&self.theme).unwrap();
        TitleSetter::set(&self.path).unwrap();

        html! {
            <>
                <NavigationBar
                    path={self.path.clone()}
                    theme={self.theme}
                    layout_changer={layout_change_callback}
                    theme_changer={theme_change_callback} />
                { self.main_view() }
            </>
        }
    }
}
