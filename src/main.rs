#![deny(rust_2018_idioms, clippy::all, clippy::pedantic)]
#![warn(clippy::nursery)]
#![allow(clippy::needless_pass_by_value)]

use leptos::{mount_to_body, view};

use self::app::App;

mod app;
mod de;
mod models;
mod widgets;

fn main() {
    console_log::init().ok();
    console_error_panic_hook::set_once();
    mount_to_body(|| view! { <App/> });
}
