#![allow(non_snake_case)]

use dioxus::prelude::*;
use rustflare::Button;
use rustflare::core::ButtonTrait;
use dioxus_logger::tracing::{Level, info};
const _TAILWIND_URL: &str = manganis::mg!(file("public/tailwind.css"));

fn main() {
    // Init logger
    dioxus_logger::init(Level::INFO).expect("failed to init logger");
    info!("starting app");
    launch(App);
}


fn App() -> Element {
    // Initialize a button component
    let mut button = Button::new("Click Me");
    
    // Set an on-click handler that logs to the console
    button.set_on_click(|| {
        web_sys::console::log_1(&"Button clicked!".into());
    });
    
    // Render the button inside Dioxus's rsx!
    button.render()
}

