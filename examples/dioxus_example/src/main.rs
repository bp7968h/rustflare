#![allow(non_snake_case)]

use rustflare::components::{Button, ButtonSize, ButtonVariant};
use rustflare::core::ButtonTrait;

use dioxus::prelude::*;
use dioxus_logger::tracing::{Level, info};
const _TAILWIND_URL: &str = manganis::mg!(file("public/tailwind.css"));

fn main() {
    // Init logger
    dioxus_logger::init(Level::INFO).expect("failed to init logger");
    info!("starting app");
    launch(App);
}


fn App() -> Element {
    // Create buttons with different variants and sizes
    let mut primary_button = Button::new("Primary Button", ButtonVariant::Default, ButtonSize::Medium);
    let mut secondary_button = Button::new("Secondary Button", ButtonVariant::Secondary, ButtonSize::Large);
    let mut destructive_button = Button::new("Destructive Button", ButtonVariant::Destructive, ButtonSize::Small);

    // Set on-click handlers for each button
    primary_button.set_on_click(|| {
        web_sys::console::log_1(&"Primary Button clicked!".into());
    });

    secondary_button.set_on_click(|| {
        web_sys::console::log_1(&"Secondary Button clicked!".into());
    });

    destructive_button.set_on_click(|| {
        web_sys::console::log_1(&"Destructive Button clicked!".into());
    });


    // primary_button.render()
    rsx!{
        div {
            class: "flex flex-col space-y-4",
            {primary_button.render()},
            {secondary_button.render()},
            {destructive_button.render()},
        }
    }
}

