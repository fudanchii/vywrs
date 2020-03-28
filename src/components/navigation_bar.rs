use std::ffi::OsStr;
use std::path::{Path, PathBuf};
use yew::prelude::*;

use crate::{VywrsMode, VywrsTheme};

pub struct NavigationBar {
    props: Props,
}

#[derive(Clone, Properties, PartialEq)]
pub struct Props {
    pub path: PathBuf,
    pub theme: VywrsTheme,
    pub layout_changer: Callback<VywrsMode>,
    pub theme_changer: Callback<VywrsTheme>,
}

impl NavigationBar {
    fn directory_link(path: &PathBuf) -> Html {
        html! {
            <>
                <span>{"/"}</span>
                <a href=format!("#{}", path.to_string_lossy())>
                    { path.file_name().unwrap_or_else(|| OsStr::new("/")).to_string_lossy() }
                </a>
            </>
        }
    }

    fn directories(path: &Path) -> Vec<PathBuf> {
        path.ancestors()
            .collect::<Vec<&Path>>()
            .iter()
            .rev()
            .skip(1)
            .map(|fragment| fragment.to_path_buf())
            .collect()
    }
}

impl Component for NavigationBar {
    type Properties = Props;
    type Message = ();

    fn create(props: Self::Properties, _: ComponentLink<Self>) -> Self {
        NavigationBar { props }
    }

    fn update(&mut self, _: Self::Message) -> ShouldRender {
        true
    }

    fn view(&self) -> Html {
        let back_href = self
            .props
            .path
            .parent()
            .unwrap_or(Path::new("#/"))
            .to_string_lossy()
            .into_owned();

        html! {
            <div class=vec!["navbar", &self.props.theme]>
                <a class="navbar__logo" href="https://github.com/fudanchii/vywrs" />
                <a class="navbar__back" href=back_href />
                <div class="navbar__location" title=self.props.path.to_string_lossy()>
                    <a class="navbar__location-home" href="#/" />
                    { for Self::directories(&self.props.path).iter().map(Self::directory_link) }
                </div>
                <div class="navbar__menu">
                    <div class="navbar__menu__viewmode"></div>
                    <div class="navbar__menu__theme-select"></div>
                    <div class="navbar__menu__about"></div>
                </div>
            </div>
        }
    }
}
