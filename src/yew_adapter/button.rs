use crate::core::{ButtonTrait, ButtonProps};
use yew::prelude::*;

pub struct Button {
    props: ButtonProps,
}

impl Button {
    pub fn new(label: &str) -> Self {
        Self {
            props: ButtonProps {
                label: label.to_string(),
                on_click: None,
                class: "px-4 py-2 bg-blue-500 text-white rounded hover:bg-blue-600".to_string(),
            },
        }
    }

    pub fn set_on_click<F: Fn() + 'static>(&mut self, f: F) {
        self.props.on_click = Some(Box::new(f));
    }
}

impl ButtonTrait for Button {
    fn render(&self) -> Html {
        let on_click = self.props.on_click.clone();
        html! {
            <button class={self.props.class.clone()}
                onclick={Callback::from(move |_| {
                    if let Some(f) = on_click {
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