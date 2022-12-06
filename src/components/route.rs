use yew::prelude::*;
use yew_router::prelude::*;

use crate::components::Vywrs;

#[derive(Clone, Routable, PartialEq)]
enum Route {
    #[at("/")]
    Root,
    #[at("/*path")]
    WildCard { path: String },
}

fn switch(routes: Route) -> Html {
    let path = match routes {
        Route::Root => "/".to_string(),
        Route::WildCard { path } => format!("/{}", path),
    };

    html! {
        <Vywrs location={path} />
    }
}

#[function_component]
pub fn VywrsRoute() -> Html {
    html! {
        <HashRouter>
            <Switch<Route> render={switch} />
        </HashRouter>
    }
}
