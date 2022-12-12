use crate::components::vywrs::Vywrs;
use crate::listing::File;
use crate::services::Config;
use gloo_net::http::Request;
use std::rc::Rc;
use yew::prelude::*;
use yew::suspense::{use_future_with_deps, SuspensionResult, UseFutureHandle};

type HttpResult<T> = UseFutureHandle<Result<T, gloo_net::Error>>;

#[hook]
fn use_file_listing(location: AttrValue) -> SuspensionResult<HttpResult<Vec<File>>> {
    use_future_with_deps(
        |rc_loc| async move {
            let config = Config::new().unwrap();
            let response = Request::get(&config.list_endpoint(&rc_loc)).send().await?;
            response.json().await
        },
        location,
    )
}

#[derive(Default, PartialEq, Properties)]
pub struct Props {
    pub location: AttrValue,
}

#[function_component]
pub fn VywrsWithFileListing(props: &Props) -> HtmlResult {
    let listing = use_file_listing(props.location.clone())?;
    let listing: Rc<Vec<File>> = listing.as_ref().unwrap().clone().into();
    Ok(html! { <Vywrs listing={listing} location={props.location.clone()} /> })
}
