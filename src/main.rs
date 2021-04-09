#![deny(rust_2018_idioms, clippy::all)]
#![recursion_limit = "256"]

mod app;
mod de;
mod models;
mod widgets;

fn main() {
    console_error_panic_hook::set_once();
    console_log::init().ok();
    yew::start_app::<app::App>();
}
