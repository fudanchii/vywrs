mod components;
mod config;
mod ext_bindings;
mod listing;
mod looks;

use components::VywrsRoute;

pub fn main() {
    yew::Renderer::<VywrsRoute>::new().render();
}
