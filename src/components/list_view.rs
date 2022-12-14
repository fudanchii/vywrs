use crate::{
    config::Config,
    listing::{File, FileType},
    looks::VywrsTheme,
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

fn row_view(props: &Props, file: &File) -> Html {
    let ft = file.file_type(props.config.borrow());
    html! {
        <div class="rows__item">
            <a href={location(props, file)} title={file.name()}>
                <div class={classes!["rows__item-filename", ft]}>
                    { file.name() }
                </div>
                <div class="rows__item-meta">
                    <div class="rows__item-filesize" title={file.size()}>
                        { file.size() }
                    </div>
                    <div class="rows__item-filedate" title={file.mtime()}>
                        { file.mtime() }
                    </div>
                </div>
            </a>
        </div>
    }
}

fn location(props: &Props, file: &File) -> String {
    let config: &Config = props.config.borrow();
    match file.file_type(config) {
        FileType::Directory => config.directory_endpoint(&props.path, &file.name()),
        _ => config.file_endpoint(&props.path, &file.name()),
    }
}

#[function_component]
pub fn ListView(props: &Props) -> Html {
    let file_listing: &Vec<File> = props.listing.borrow();
    html! {
        <div class={classes!["rows", props.theme]}>
            { for file_listing.iter().map(|file| row_view(props, file)) }
        </div>
    }
}
