use std::rc::Rc;
use yew::prelude::*;
use yew_router::prelude::*;

use crate::components::Vywrs;
use crate::components::VywrsWithFileListing;
use crate::services::Config;

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
        Route::WildCard { path } => format!("/{}", Config::url_decode(&path)),
    };

    let fallback =
        html! {<Vywrs location={path.clone()} listing={Rc::new(vec![])} is_fetching={true} />};

    html! {
        <Suspense {fallback}>
            <VywrsWithFileListing location={path} />
        </Suspense>
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
