use crate::{
    listing::{File, FileType},
    services::Config,
    vywrs::VywrsTheme,
};
use std::borrow::Borrow;
use std::rc::Rc;
use yew::prelude::*;

#[derive(Clone, Properties, PartialEq)]
pub struct Props {
    pub listing: Rc<Vec<File>>,
    pub theme: VywrsTheme,
    pub path: AttrValue,
    pub config: Rc<Config>,
}

fn tile_view(props: &Props, file: &File) -> Html {
    macro_rules! tile {
        ($tile:expr, $link:expr, $href:expr,) => {
            html! {
                <div class={$tile}>
                    <a class={$link} href={$href} data-gallery="vywrs">
                        <div class="tiles__icon">
                            { thumbnail(props, file) }
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

    let config: &Config = props.config.borrow();
    match file.file_type(config) {
        FileType::Directory => tile! {
            "tiles__directory",
            "tiles__directory-link",
            config.directory_endpoint(&props.path, &file.name()),
        },
        FileType::File => tile! {
            "tiles__file",
            "tiles__file-link",
            config.file_endpoint(&props.path, &file.name()),
        },
        FileType::Image => tile! {
            "tiles__image",
            "tiles__image-link glightbox",
            config.file_endpoint(&props.path, &file.name()),
        },
    }
}

fn thumbnail(props: &Props, file: &File) -> Html {
    let config: &Config = props.config.borrow();
    match file.file_type(config) {
        FileType::Directory => html! { <div class="icon-directory" /> },
        FileType::File => html! { <div class="icon-file" /> },
        FileType::Image => html! {
            <div class="icon-image"
                style={background_image(props, &file.name())} />
        },
    }
}

fn background_image(props: &Props, name: &str) -> String {
    let config: &Config = props.config.borrow();
    format!(
        "background-image: url(\"{}\")",
        config.thumbnailer(&props.path, name)
    )
}

#[function_component]
pub fn TileView(props: &Props) -> Html {
    let file_listing: &Vec<File> = props.listing.borrow();
    html! {
        <div class={classes!["tiles", props.theme]}>
            { for file_listing.iter().map(|file| tile_view(props, file)) }
        </div>
    }
}
