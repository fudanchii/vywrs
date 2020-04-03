use crate::{
    listing::{File, FileType},
    neq_assign::NeqAssign,
    services::Config,
    vywrs::VywrsTheme,
};
use std::borrow::Borrow;
use std::rc::Rc;
use yew::prelude::*;

pub struct TileView {
    props: Props,
}

#[derive(Clone, Properties, PartialEq)]
pub struct Props {
    pub listing: Rc<Vec<File>>,
    pub theme: VywrsTheme,
    pub path: String,
    pub config: Rc<Config>,
}

impl TileView {
    fn tile_view(&self, file: &File) -> Html {
        macro_rules! tile {
            ($tile:expr, $link:expr, $href:expr,) => {
                html! {
                    <div class=$tile>
                        <a class=$link href=$href>
                            <div class="tiles__icon">
                                { self.thumbnail(file) }
                            </div>
                            <div class="tiles__label-wrapper">
                                <div class="tiles__label" title=file.name()>
                                    {file.name()}
                                </div>
                            </div>
                        </a>
                    </div>
                }
            };
        }

        let config: &Config = self.props.config.borrow();
        match file.file_type() {
            FileType::Directory => tile! {
                "tiles__directory",
                "tiles__directory-link",
                file.location(&self.props.path),
            },
            FileType::File => tile! {
                "tiles__file",
                "tiles__file-link",
                config.file_endpoint(&self.props.path, &file.name()),
            },
            FileType::Image => tile! {
                "tiles__image",
                "tiles__image-link",
                config.file_endpoint(&self.props.path, &file.name()),
            },
        }
    }

    fn thumbnail(&self, file: &File) -> Html {
        match file.file_type() {
            FileType::Directory => html! { <div class="icon-directory" /> },
            FileType::File => html! { <div class="icon-file" /> },
            FileType::Image => html! {
                <div class="icon-image"
                    style=self.background_image(&file.name()) />
            },
        }
    }

    fn background_image(&self, name: &str) -> String {
        let config: &Config = self.props.config.borrow();
        format!(
            "backgroud-image: url(\"{}\")",
            config.thumbnailer(&self.props.path, name)
        )
    }
}

impl Component for TileView {
    type Properties = Props;
    type Message = ();

    fn create(props: Self::Properties, _: ComponentLink<Self>) -> Self {
        TileView { props }
    }

    fn update(&mut self, _: Self::Message) -> ShouldRender {
        true
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.props.neq_assign(props)
    }

    fn view(&self) -> Html {
        let file_listing: &Vec<File> = self.props.listing.borrow();
        html! {
            <div class=vec!["tiles", &self.props.theme]>
                { for file_listing.iter().map(|file| self.tile_view(file)) }
            </div>
        }
    }
}
