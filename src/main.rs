mod components;
mod listing;
mod services;
mod vywrs;

use components::VywrsRoute;

pub fn main() {
    yew::Renderer::<VywrsRoute>::new().render();
}
