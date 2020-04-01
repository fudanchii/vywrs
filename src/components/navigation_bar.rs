use std::ffi::OsStr;
use std::path::{Path, PathBuf};
use yew::prelude::*;

use crate::neq_assign::NeqAssign;
use crate::{VywrsMessage, VywrsMode, VywrsTheme};

pub struct NavigationBar {
    link: ComponentLink<Self>,
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
    type Message = VywrsMessage;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        NavigationBar { link, props }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            VywrsMessage::ChangeMode(mode) => self.props.layout_changer.emit(mode),
            VywrsMessage::ChangeTheme(theme) => {
                self.props.theme_changer.emit(theme);
                return theme != self.props.theme;
            }
        }

        false
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.props.neq_assign(props)
    }

    fn view(&self) -> Html {
        let back_href = self
            .props
            .path
            .parent()
            .unwrap_or(Path::new("#/"))
            .to_string_lossy()
            .into_owned();

        let into_thumbnail = self
            .link
            .callback(|_| VywrsMessage::ChangeMode(VywrsMode::Tile));
        let into_list = self
            .link
            .callback(|_| VywrsMessage::ChangeMode(VywrsMode::List));
        let use_dark_theme = self
            .link
            .callback(|_| VywrsMessage::ChangeTheme(VywrsTheme::Dark));
        let use_light_theme = self
            .link
            .callback(|_| VywrsMessage::ChangeTheme(VywrsTheme::Light));

        html! {
            <div class=vec!["navbar", &self.props.theme]>
                <a class="navbar__logo" href="https://github.com/fudanchii/vywrs" />
                <a class="navbar__back" href=back_href />
                <div class="navbar__location" title=self.props.path.to_string_lossy()>
                    <a class="navbar__location-home" href="#/" />
                    { for Self::directories(&self.props.path).iter().map(Self::directory_link) }
                </div>
                <div class="navbar__menu">
                    <a class="navbar__menu--icon" />
                    <div class=vec!["navbar__menu--content", &self.props.theme]>
                        <div class="navbar__menu--viewmode">
                            {"view: "}
                            <a class="navbar__menu--viewmode--thumbnail" onclick=into_thumbnail />
                            <a class="navbar__menu--viewmode--list" onclick=into_list />
                        </div>
                        <div class="navbar__menu--theme">
                            {"theme: "}
                            <a class="navbar__menu--theme--dark" onclick=use_dark_theme />
                            <a class="navbar__menu--theme--light" onclick=use_light_theme />
                        </div>
                        <div class="navbar__menu--about">
                            {"made with 💙 by "}
                            <a href="https://fudan.ch">{"@fudanchii"}</a>
                        </div>
                    </div>
                </div>
            </div>
        }
    }
}
