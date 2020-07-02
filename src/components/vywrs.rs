use crate::{
    components::{ListView, NavigationBar, TileView},
    listing::File,
    services::{BodyClassSetter, Config},
    vywrs::{VywrsMode, VywrsTheme},
};
use anyhow::Error;
use std::rc::Rc;
use yew::format::{Json, Nothing};
use yew::prelude::*;
use yew::services::fetch::{FetchService, FetchTask, Request, Response};
use yew_router::prelude::RouteService;

pub struct Vywrs {
    link: ComponentLink<Self>,
    state: State,
    config: Rc<Config>,
    fetch_task: Option<FetchTask>,
    rs: RouteService<()>,
}

struct State {
    path: String,
    theme: VywrsTheme,
    mode: VywrsMode,
    listing: Rc<Vec<File>>,
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

    fn fetch_listing(&mut self, endpoint: &str) -> Result<(), Error> {
        self.fetch_task = Some(FetchService::fetch::<
            Nothing,
            Json<Result<Vec<File>, Error>>,
        >(
            Request::get(endpoint).body(Nothing).expect(endpoint),
            self.link
                .callback(|response: Response<Json<Result<Vec<File>, Error>>>| {
                    let (meta, Json(data)) = response.into_parts();
                    if meta.status.is_success() {
                        return VywrsMessage::UpdateListing(data.unwrap());
                    }
                    VywrsMessage::FetchFailed
                }),
        )?);
        Ok(())
    }

    fn setup_routing(&mut self) {
        self.rs
            .register_callback(self.link.callback(|_| VywrsMessage::FetchListing));
    }

    fn do_fetch_listing(&mut self) -> bool {
        let hashloc = self.rs.get_fragment();
        let hashloc = Config::url_decode(&hashloc.trim_start_matches('#'));
        let endpoint = self.config.list_endpoint(&hashloc);
        self.fetch_listing(&endpoint).unwrap();
        self.state.path = hashloc;
        false
    }

    fn do_update_listing(&mut self, new_listing: Vec<File>) -> bool {
        let prev = self.state.listing.clone();
        self.state.listing = Rc::new(new_listing);
        prev != self.state.listing
    }
}

impl Component for Vywrs {
    type Message = VywrsMessage;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        let state = State {
            path: String::from(""),
            mode: VywrsMode::Tile,
            theme: VywrsTheme::Dark,
            listing: Rc::new(vec![]),
        };

        let config = Config::new().unwrap();
        let config = Rc::new(config);

        let rs = RouteService::new();
        let mut app = Vywrs {
            config,
            link,
            state,
            fetch_task: None,
            rs,
        };

        app.setup_routing();
        app.update(VywrsMessage::FetchListing);

        app
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        false
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        macro_rules! rerender_if_changed {
            ($a:ident, $b:expr) => {{
                let prev = self.state.$a;
                self.state.$a = $b;
                return prev != $b;
            }};
        }

        match msg {
            VywrsMessage::ChangeMode(new_mode) => rerender_if_changed!(mode, new_mode),
            VywrsMessage::ChangeTheme(new_theme) => rerender_if_changed!(theme, new_theme),
            VywrsMessage::UpdateListing(new_listing) => self.do_update_listing(new_listing),
            VywrsMessage::FetchListing => self.do_fetch_listing(),
            VywrsMessage::FetchFailed => false,
        }
    }

    fn view(&self) -> Html {
        let layout_change_callback = self.link.callback(VywrsMessage::ChangeMode);
        let theme_change_callback = self.link.callback(VywrsMessage::ChangeTheme);

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
