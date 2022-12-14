mod config;
mod components;
mod listing;
mod ext_bindings;
mod looks;

use components::VywrsRoute;

pub fn main() {
    yew::Renderer::<VywrsRoute>::new().render();
}
