use yew::prelude::*;

use crate::core::{ButtonProps, ButtonTrait};
use crate::components::Button;

// Implement ButtonTrait for Yew with return type Html
#[cfg(feature = "yew")]
impl ButtonTrait for Button {
    type RenderType = Html;
    
    fn render(&self) -> Html {
        let on_click = self.props.on_click.clone();
        html! {
            <button class={self.props.class.clone()}
                onclick={Callback::from(move |_| {
                    if let Some(f) = on_click.clone() {
                        f();
                    }
                })}>
                { &self.props.label }
            </button>
        }
    }

    fn set_props(&mut self, props: ButtonProps) {
        self.props = props;
    }
}
