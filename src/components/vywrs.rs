use crate::{
    components::{ListView, NavigationBar, TileView},
    config::Config,
    ext_bindings::{glightbox, GLightbox},
    listing::File,
    looks::{VywrsMode, VywrsTheme},
};

use gloo_utils::{body, document};
use std::rc::Rc;
use yew::prelude::*;

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
    ApplyLightBox(bool),
}

macro_rules! rerender_if_changed {
    ($a:expr, $b:ident) => {{
        if $a != $b {
            $a = $b;
            return true;
        }
        false
    }};
}

impl Vywrs {
    fn apply_lightbox(&mut self, first_render: bool) -> bool {
        // at first render, rendered->update cycle will run
        // before the DOM is rendered to the page, this is causing
        // GLightbox not to detect any images.
        //
        // So do nothing here, and ask for rerender instead so we get the
        // DOM rendered to the page first.
        if first_render {
            return true;
        }

        if let Some(lb) = self.lightbox.replace(GLightbox()) {
            lb.destroy();
        }

        false
    }

    fn apply_theme(&mut self, new: VywrsTheme) -> bool {
        rerender_if_changed!(self.theme, new)
    }

    fn apply_mode(&mut self, new: VywrsMode) -> bool {
        rerender_if_changed!(self.mode, new)
    }
}

impl Component for Vywrs {
    type Message = VywrsMessage;
    type Properties = VywrsProps;

    fn create(_ctx: &Context<Self>) -> Self {
        let config = Config::new().unwrap();
        Self {
            config: Rc::new(config),
            mode: VywrsMode::Tile,
            theme: VywrsTheme::Dark,
            lightbox: None,
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            VywrsMessage::ChangeMode(nm) => self.apply_mode(nm),
            VywrsMessage::ChangeTheme(nt) => self.apply_theme(nt),
            VywrsMessage::ApplyLightBox(fr) => self.apply_lightbox(fr),
        }
    }

    fn rendered(&mut self, ctx: &Context<Self>, first_render: bool) {
        if ctx.props().is_fetching {
            return;
        }

        ctx.link()
            .send_message(VywrsMessage::ApplyLightBox(first_render));
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
                {
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
            </>
        }
    }
}
