use crate::neq_assign::NeqAssign;
use crate::vywrs::{VywrsMode, VywrsTheme};
use yew::prelude::*;

pub struct NavigationBar {
    link: ComponentLink<Self>,
    props: Props,
}

pub enum Message {
    ChangeMode(VywrsMode),
    ChangeTheme(VywrsTheme),
}

#[derive(Clone, Properties, PartialEq)]
pub struct Props {
    pub path: String,
    pub theme: VywrsTheme,
    pub layout_changer: Callback<VywrsMode>,
    pub theme_changer: Callback<VywrsTheme>,
}

impl NavigationBar {
    fn directory_link<T: Into<String>>(path: T) -> Html {
        let path: String = path.into();
        html! {
            <>
                <span>{"/"}</span>
                <a href=format!("#/{}", path)>
                    { path.split('/').rev().take(1).collect::<Vec<&str>>()[0] }
                </a>
            </>
        }
    }

    fn directories(path: &str) -> Vec<String> {
        let fragments: Vec<&str> = path.split('/').filter(|&s| s != "").collect();

        fragments
            .iter()
            .enumerate()
            .map(|(idx, &fragment)| {
                fragments
                    .iter()
                    .take(idx)
                    .chain(&[fragment])
                    .map(|pfrag| (*pfrag).to_string())
                    .collect::<Vec<_>>()
            })
            .map(|path| path.join("/"))
            .collect()
    }
}

impl Component for NavigationBar {
    type Properties = Props;
    type Message = Message;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        NavigationBar { link, props }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Message::ChangeMode(mode) => self.props.layout_changer.emit(mode),
            Message::ChangeTheme(theme) => {
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
        let fragments: Vec<String> = self
            .props
            .path
            .split_terminator('/')
            .map(|fragment| fragment.to_string())
            .collect::<Vec<_>>();

        let back_href = format!(
            "#{}",
            fragments
                .iter()
                .take(if fragments.iter().count() > 0 {
                    fragments.iter().count() - 1
                } else {
                    0
                })
                .map(|fragment| fragment.to_string())
                .collect::<Vec<String>>()
                .join("/")
        );

        let into_thumbnail = self.link.callback(|_| Message::ChangeMode(VywrsMode::Tile));
        let into_list = self.link.callback(|_| Message::ChangeMode(VywrsMode::List));
        let use_dark_theme = self
            .link
            .callback(|_| Message::ChangeTheme(VywrsTheme::Dark));
        let use_light_theme = self
            .link
            .callback(|_| Message::ChangeTheme(VywrsTheme::Light));

        html! {
            <div class=vec!["navbar", &self.props.theme]>
                <a class="navbar__logo" href="https://github.com/fudanchii/vywrs" />
                <a class="navbar__back" href=back_href />
                <div class="navbar__location" title=self.props.path>
                    <a class="navbar__location-home" href="#" />
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
