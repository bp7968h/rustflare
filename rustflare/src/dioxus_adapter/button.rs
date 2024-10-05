use dioxus::prelude::*;

use crate::core::{ButtonProps, ButtonTrait};
use crate::components::Button;


#[cfg(feature = "dioxus")]
impl ButtonTrait for Button {
    type RenderType = Element;

    fn render(&self) -> Element {
        let on_click = self.props.on_click.clone();
        rsx! {
            button {
                class: "{self.props.class}",
                onclick: move |_| {
                    if let Some(f) = on_click.clone() {
                        f();
                    }
                },
                "{self.props.label}"
            }
        }
    }

    fn set_props(&mut self, props: ButtonProps) {
        self.props = props;
    }
}
