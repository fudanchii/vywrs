use crate::{listing::File, neq_assign::NeqAssign, services::Config, VywrsTheme};
use std::borrow::Borrow;
use std::path::PathBuf;
use std::rc::Rc;
use yew::prelude::*;

pub struct ListView {
    props: Props,
}

#[derive(Clone, Properties, PartialEq)]
pub struct Props {
    pub listing: Rc<Vec<File>>,
    pub theme: VywrsTheme,
    pub path: PathBuf,
    pub config: Rc<Config>,
}

impl ListView {
    fn row_view(&self, file: &File) -> Html {
        html! {
            <div class="rows__item">
                <div class="rows__item-filename">
                    <a href=file.location(self.props.path.clone()) title=file.name()>
                        { file.name() }
                    </a>
                </div>
                <div class="rows__item-meta">
                    <div class="rows__item-filesize" title=file.size()>
                        { file.size() }
                    </div>
                    <div class="rows__item-filedate" title=file.mtime()>
                        { file.mtime() }
                    </div>
                </div>
            </div>
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
