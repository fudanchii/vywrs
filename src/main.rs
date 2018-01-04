#[macro_use]
extern crate yew;

#[macro_use]
extern crate stdweb;

extern crate serde;
extern crate serde_json;

extern crate urlencoding;

#[macro_use]
extern crate serde_derive;

extern crate bytesize;

mod vyw;

use urlencoding::decode;
use yew::html::*;
use yew::format::Json;
use yew::services::fetch::{FetchService, Method};
use vyw::*;

enum Msg {
    GotoDir,
    ChangeMode(ViewMode),
    FileList(Vec<File>),
}

fn update(ctx: &mut Context, fl: &mut FileListing, msg: Msg) {
    match msg {
        Msg::GotoDir => {
            ctx.web.fetch(Method::Get, &fl.listing_endpoint(), None, |Json(lst)| Msg::FileList(lst.unwrap()));
        },
        Msg::FileList(lst) => {
            fl.set_listing(&lst);
        },
        Msg::ChangeMode(v) => {
            fl.set_view_mode(&v)
        }
    }
}

fn view(fl: &FileListing) -> Html<Msg> {
    html! {
        <div id="entrypoint", >
            { navigation_bar(fl) }
            { main_view(fl) }
        </div>
    }
}

fn main_view(fl: &FileListing) -> Html<Msg> {
    match fl.get_view_mode() {
        ViewMode::Thumbnail => thumbnail_view(fl),
        ViewMode::List => list_view(fl),
    }
}

fn list_view(fl: &FileListing) -> Html<Msg> {
    html! {
        <div class="rows", >
            {for fl.map_listing(row_view).into_iter() }
        </div>
    }
}

fn row_view(fl: &FileListing, f: &File) -> Html<Msg> {
    html! {
        <div class="rows__item", >
            <div class="rows__item-filename", >
                <a href=fl.file_endpoint(f), title=decode(&f.name()).unwrap(), >
                    { decode(&f.name()).unwrap() }
                </a>
            </div>
            <div class="rows__item-meta", >
                <div class="rows__item-filesize", title=f.size(), >{ f.size() }</div>
                <div class="rows__item-filedate", title=f.mtime(), >{ f.mtime() }</div>
            </div>
        </div>
    }
}

fn thumbnail_view(fl: &FileListing) -> Html<Msg> {
    html! {
        <div class="tiles", >
            { for fl.map_listing(tile_view).into_iter() }
        </div>
    }
}

fn tile_view(fl: &FileListing, item: &File) -> Html<Msg> {
    match item.file_type() {
        ref t if t == "image" => {
            tile_view_html(fl, item, "tiles__image", "tiles__image-link")
        },
        ref t if t == "directory" => {
            tile_view_html(fl, item, "tiles__directory", "tiles__directory-link")
        },
        ref _t => {
            tile_view_html(fl, item, "tiles__file", "tiles__file-link")
        },
    }
}

fn tile_view_html(fl: &FileListing, item: &File, tcl: &'static str, lcl: &'static str) -> Html<Msg> {
    html! {
        <div class=tcl, >
            <a class=lcl, href=fl.file_endpoint(item), >
                { icon_view(fl, item) }
                <div class="tiles__label-wrapper", >
                    <div class="tiles__label", title=item.name(), >
                        { item.name() }
                    </div>
                </div>
            </a>
        </div>
    }
}

fn icon_view(fl: &FileListing, item: &File) -> Html<Msg> {
    match item.file_type() {
        ref t if t == "image" => html! {
            <div class="tiles__icon", >
                <div class="icon-image",
                     style=format!("background-image: url(\"{}\")", fl.thumbnail_endpoint(item)), >
                </div>
            </div>
        },
        ref t if t == "directory" => html! {
            <div class="tiles__icon", >
                <div class="icon-directory", > </div>
            </div>
        },
        ref _t => html! {
            <div class="tiles__icon", >
                <div class="icon-file", > </div>
            </div>
        }
    }
}

fn location_a_href(path: &str) -> Vec<Html<Msg>> {
    let mut hrefs = Vec::new();
    let arpath = path.split("/");
    let mut count = 1;
    for frp in arpath.clone() {
        let cpath = arpath.clone()
            .take(count)
            .fold(String::new(), |s, c| match c {
                e if e == "" => "".to_owned(),
                _ => format!("{}/{}", s, c),
            });
        if cpath != "" {
            hrefs.push(html! {
                <span>{"/"}</span>
                <a href=format!("#{}", cpath),>{ frp }</a>
            });
        }
        count += 1;
    }
    hrefs
}

fn navigation_bar(_fl: &FileListing) -> Html<Msg> {
    let fpath = decode(&get_location_hash()).unwrap();
    html! {
        <div class="navbar", >
            <a href="https://github.com/fudanchii/vywrs", >
                <div class="navbar__logo", ></div>
            </a>
            <a href=FileListing::parent_dir_endpoint(), >
                <div class="navbar__back", ></div>
            </a>
            <div class="navbar__location", title=&fpath, >
                <a class="navbar__location-home", href="#/", ></a>
                { for location_a_href(&fpath).into_iter() }
            </div>
            <div class="navbar__viewmode", >
                <div class="navbar__viewmode-thumbnail-container", >
                    <div class="navbar__viewmode-thumbnail",
                         onclick=|_ev|Msg::ChangeMode(ViewMode::Thumbnail), ></div>
                </div>
                <div class="navbar__viewmode-list-container", >
                    <div class="navbar__viewmode-list",
                         onclick=|_ev|Msg::ChangeMode(ViewMode::List), ></div>
                </div>
            </div>
        </div>
    }
}

struct Context {
    web: FetchService<Msg>,
}

fn main() {
    let mut app = App::new();
    let file_lst = FileListing::create();
    let ctx = Context {
        web: FetchService::new(app.sender()),
    };
    let mut sender = app.sender();
    let popstate_handler = move || {
        sender.send(Msg::GotoDir);
    };

    // FIXME: remove js macro when
    // https://github.com/koute/stdweb/pull/49 merged
    js! {
        var handler = @{popstate_handler};
        window.onpopstate = (ev) => { handler() };
    }
    app.sender().send(Msg::GotoDir);
    app.run(ctx, file_lst, update, view);
}
