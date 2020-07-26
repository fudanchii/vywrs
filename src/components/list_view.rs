use crate::{
    listing::{File, FileType},
    services::Config,
    vywrs::VywrsTheme,
};
use std::borrow::Borrow;
use std::rc::Rc;
use yew::prelude::*;
use yewtil::NeqAssign;

pub struct ListView {
    props: Props,
}

#[derive(Clone, Properties, PartialEq)]
pub struct Props {
    pub listing: Rc<Vec<File>>,
    pub theme: VywrsTheme,
    pub path: String,
    pub config: Rc<Config>,
}

impl ListView {
    fn row_view(&self, file: &File) -> Html {
        html! {
            <div class="rows__item">
                <a href=self.location(file) title=file.name()>
                    <div class="rows__item-filename">
                        { file.name() }
                    </div>
                    <div class="rows__item-meta">
                        <div class="rows__item-filesize" title=file.size()>
                            { file.size() }
                        </div>
                        <div class="rows__item-filedate" title=file.mtime()>
                            { file.mtime() }
                        </div>
                    </div>
                </a>
            </div>
        }
    }

    fn location(&self, file: &File) -> String {
        let config: &Config = self.props.config.borrow();
        match file.file_type(config) {
            FileType::Directory => config.directory_endpoint(&self.props.path, &file.name()),
            _ => config.file_endpoint(&self.props.path, &file.name()),
        }
    }
}

impl Component for ListView {
    type Properties = Props;
    type Message = ();

    fn create(props: Self::Properties, _: ComponentLink<Self>) -> Self {
        ListView { props }
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
            <div class=vec!["rows", &self.props.theme]>
                { for file_listing.iter().map(|file| self.row_view(file)) }
            </div>
        }
    }
}
