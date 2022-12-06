use crate::{
    listing::{File, FileType},
    services::Config,
    vywrs::VywrsTheme,
};
use std::borrow::Borrow;
use std::rc::Rc;
use yew::prelude::*;

pub struct ListView;

#[derive(Clone, Properties, PartialEq)]
pub struct Props {
    pub listing: Rc<Vec<File>>,
    pub theme: VywrsTheme,
    pub path: AttrValue,
    pub config: Rc<Config>,
}

impl ListView {
    fn row_view(&self, ctx: &Context<Self>, file: &File) -> Html {
        html! {
            <div class="rows__item">
                <a href={self.location(ctx, file)} title={file.name()}>
                    <div class="rows__item-filename">
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

    fn location(&self, ctx: &Context<Self>, file: &File) -> String {
        let config: &Config = ctx.props().config.borrow();
        match file.file_type(config) {
            FileType::Directory => config.directory_endpoint(&ctx.props().path, &file.name()),
            _ => config.file_endpoint(&ctx.props().path, &file.name()),
        }
    }
}

impl Component for ListView {
    type Properties = Props;
    type Message = ();

    fn create(_ctx: &Context<Self>) -> Self {
        ListView {}
    }

    fn update(&mut self, _ctx: &Context<Self>, _: Self::Message) -> bool {
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let file_listing: &Vec<File> = ctx.props().listing.borrow();
        html! {
            <div class={classes!("rows", ctx.props().theme)}>
                { for file_listing.iter().map(|file| self.row_view(ctx, file)) }
            </div>
        }
    }
}
