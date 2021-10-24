mod components;
mod listing;
mod services;
mod vywrs;

use components::Vywrs;

#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

pub fn main() {
    yew::start_app::<Vywrs>();
}
