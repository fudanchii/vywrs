use crate::{
    listing::{File, FileType},
    services::Config,
    vywrs::VywrsTheme,
};
use std::borrow::Borrow;
use std::rc::Rc;
use yew::prelude::*;

pub struct TileView;

#[derive(Clone, Properties, PartialEq)]
pub struct Props {
    pub listing: Rc<Vec<File>>,
    pub theme: VywrsTheme,
    pub path: String,
    pub config: Rc<Config>,
}

impl TileView {
    fn tile_view(&self, ctx: &Context<Self>, file: &File) -> Html {
        macro_rules! tile {
            ($tile:expr, $link:expr, $href:expr,) => {
                html! {
                    <div class={$tile}>
                        <a class={$link} href={$href} data-gallery="vywrs">
                            <div class="tiles__icon">
                                { self.thumbnail(ctx, file) }
                            </div>
                            <div class="tiles__label-wrapper">
                                <div class="tiles__label" title={file.name()}>
                                    {file.name()}
                                </div>
                            </div>
                        </a>
                    </div>
                }
            };
        }

        let config: &Config = ctx.props().config.borrow();
        match file.file_type(config) {
            FileType::Directory => tile! {
                "tiles__directory",
                "tiles__directory-link",
                config.directory_endpoint(&ctx.props().path, &file.name()),
            },
            FileType::File => tile! {
                "tiles__file",
                "tiles__file-link",
                config.file_endpoint(&ctx.props().path, &file.name()),
            },
            FileType::Image => tile! {
                "tiles__image",
                "tiles__image-link glightbox",
                config.file_endpoint(&ctx.props().path, &file.name()),
            },
        }
    }

    fn thumbnail(&self, ctx: &Context<Self>, file: &File) -> Html {
        let config: &Config = ctx.props().config.borrow();
        match file.file_type(config) {
            FileType::Directory => html! { <div class="icon-directory" /> },
            FileType::File => html! { <div class="icon-file" /> },
            FileType::Image => html! {
                <div class="icon-image"
                    style={self.background_image(ctx, &file.name())} />
            },
        }
    }

    fn background_image(&self, ctx: &Context<Self>, name: &str) -> String {
        let config: &Config = ctx.props().config.borrow();
        format!(
            "background-image: url(\"{}\")",
            config.thumbnailer(&ctx.props().path, name)
        )
    }
}

impl Component for TileView {
    type Properties = Props;
    type Message = ();

    fn create(_ctx: &Context<Self>) -> Self {
        TileView {}
    }

    fn update(&mut self, _ctx: &Context<Self>, _: Self::Message) -> bool {
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let file_listing: &Vec<File> = ctx.props().listing.borrow();
        html! {
            <div class={classes!["tiles", ctx.props().theme]}>
                { for file_listing.iter().map(|file| self.tile_view(ctx, file)) }
            </div>
        }
    }
}
