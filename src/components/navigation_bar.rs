use crate::looks::{VywrsMode, VywrsTheme};
use yew::prelude::*;

pub struct NavigationBar;

pub enum Message {
    ChangeMode(VywrsMode),
    ChangeTheme(VywrsTheme),
}

#[derive(Clone, Properties, PartialEq)]
pub struct Props {
    #[prop_or_default]
    pub is_fetching: bool,
    pub path: AttrValue,
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
                <a href={href_path}>
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
            .filter(|&fragment| !fragment.is_empty())
            .map(|fragment| fragment.to_string())
            .collect()
    }

    fn menu(&self, ctx: &Context<Self>) -> Html {
        let link = ctx.link();
        let into_thumbnail = link.callback(|_| Message::ChangeMode(VywrsMode::Tile));
        let into_list = link.callback(|_| Message::ChangeMode(VywrsMode::List));
        let use_dark_theme = link.callback(|_| Message::ChangeTheme(VywrsTheme::Dark));
        let use_light_theme = link.callback(|_| Message::ChangeTheme(VywrsTheme::Light));

        html! {
            <div class="navbar__menu">
                <a class="navbar__menu--icon" />
                <div class={classes!["navbar__menu--content", ctx.props().theme]}>
                    <div class="navbar__menu--viewmode">
                        {"view: "}
                        <a class="navbar__menu--viewmode--thumbnail" onclick={into_thumbnail} />
                        <a class="navbar__menu--viewmode--list" onclick={into_list} />
                    </div>
                    <div class="navbar__menu--theme">
                        {"theme: "}
                        <a class="navbar__menu--theme--dark" onclick={use_dark_theme} />
                        <a class="navbar__menu--theme--light" onclick={use_light_theme} />
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

    fn create(_ctx: &Context<Self>) -> Self {
        NavigationBar {}
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Message::ChangeMode(mode) => ctx.props().layout_changer.emit(mode),
            Message::ChangeTheme(theme) => {
                ctx.props().theme_changer.emit(theme);
                return theme != ctx.props().theme;
            }
        }

        false
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let mut last_slash = ctx.props().path.rfind('/').map(|pos| pos + 1).unwrap_or(0);
        if last_slash == 1 {
            last_slash = 2;
        }

        let mut path = String::from("#");
        path.push_str(&ctx.props().path);
        let back_href = path.get(0..last_slash).unwrap_or("#/").to_string();

        html! {
            <div class={classes!["navbar", ctx.props().theme]}>
                <a class="navbar__logo" href="https://github.com/fudanchii/vywrs" />
                <a class="navbar__back" href={back_href} />
                <div class="navbar__location" title={ctx.props().path.clone()}>
                    <a class={format!("navbar__location-home__fetching--{}", ctx.props().is_fetching)} href="#/" />
                    { for Self::directories(&ctx.props().path).iter().map(Self::directory_link) }
                </div>
                { self.menu(ctx) }
            </div>
        }
    }
}
