use crate::vywrs::{VywrsMode, VywrsTheme};
use yew::prelude::*;
use yewtil::NeqAssign;

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
    fn directory_link(path: &String) -> Html {
        let lspos = (*path).rfind('/').map(|pos| pos + 1).unwrap_or(0);
        let mut href_path = String::from("#");
        href_path.push_str(path);
        html! {
            <>
                <span>{"/"}</span>
                <a href=href_path>
                    { (*path).get(lspos..path.len()).unwrap() }
                </a>
            </>
        }
    }

    fn directories(path: &str) -> Vec<String> {
        let mut path = String::from(path);
        path.push('/');
        path.match_indices('/')
            .map(|(idx, _)| path.get(..idx).unwrap_or(""))
            .filter(|&fragment| fragment != "")
            .map(|fragment| fragment.to_string())
            .collect()
    }

    fn menu(&self) -> Html {
        let into_thumbnail = self.link.callback(|_| Message::ChangeMode(VywrsMode::Tile));
        let into_list = self.link.callback(|_| Message::ChangeMode(VywrsMode::List));
        let use_dark_theme = self
            .link
            .callback(|_| Message::ChangeTheme(VywrsTheme::Dark));
        let use_light_theme = self
            .link
            .callback(|_| Message::ChangeTheme(VywrsTheme::Light));

        html! {
            <div class="navbar__menu">
                <a class="navbar__menu--icon" />
                <div class=classes!["navbar__menu--content", self.props.theme]>
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
        }
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
        let last_slash = self.props.path.rfind('/').map(|pos| pos + 1).unwrap_or(0);

        let mut path = String::from("#");
        path.push_str(&self.props.path);
        let back_href = if last_slash == 0 {
            "#"
        } else {
            path.get(0..last_slash).unwrap_or("#")
        };

        html! {
            <div class=classes!["navbar", self.props.theme]>
                <a class="navbar__logo" href="https://github.com/fudanchii/vywrs" />
                <a class="navbar__back" href=back_href.to_string() />
                <div class="navbar__location" title=self.props.path.clone()>
                    <a class="navbar__location-home" href="#" />
                    { for Self::directories(&self.props.path).iter().map(Self::directory_link) }
                </div>
                { self.menu() }
            </div>
        }
    }
}
