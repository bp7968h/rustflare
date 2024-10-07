use super::{ButtonSize, ButtonVariant};
use crate::core::{ButtonProps, ButtonTrait};

pub struct Button {
    pub props: ButtonProps,
    pub variant: ButtonVariant,
    pub size: ButtonSize,
}

impl Button {
    pub fn new(label: &str, variant: ButtonVariant, size: ButtonSize) -> Self {
        Self {
            props: ButtonProps::new(label),
            variant,
            size,
        }
    }

    pub fn generate_class(&self) -> String {
        let base_class = format!("{} {}", self.variant.to_class(), self.size.to_class());

        if let Some(custom_class) = &self.props.class {
            format!("{} {}", base_class, custom_class)
        } else {
            base_class
        }
    }

    pub fn set_on_click<F: Fn() + 'static>(&mut self, callback: F) -> &mut Self {
        self.props.set_on_click(callback);
        self
    }

    pub fn add_class(&mut self, class: &str) -> &mut Self {
        self.props.add_class(class);
        self
    }
}

impl Default for Button {
    fn default() -> Self {
        Self {
            props: ButtonProps::default(),
            variant: ButtonVariant::Default,
            size: ButtonSize::Medium,
        }
    }
}


#[cfg(feature = "dioxus")]
use dioxus::prelude::*;

#[cfg(feature = "dioxus")]
impl ButtonTrait for Button {
    type RenderType = Element;

    fn render(&self) -> Element {
        let on_click = self.props.on_click.clone();
        let class_name = self.generate_class();

        rsx! {
            button {
                class: "{class_name}",
                onclick: move |_| {
                    if let Some(f) = on_click.clone() {
                        f();
                    }
                },
                "{self.props.label}"
            }
        }
    }

    fn set_props(&mut self, props: ButtonProps) -> &mut Self {
        self.props = props;
        self
    }

    fn add_class(&mut self, custom_class: &str) -> &mut Self {
        self.props.add_class(custom_class);
        self
    }
}

#[cfg(feature = "yew")]
use yew::prelude::*;

#[cfg(feature = "yew")]
impl ButtonTrait for Button {
    type RenderType = Html;

    fn render(&self) -> Html {
        let on_click = self.props.on_click.clone();
        let class_name = self.generate_class();

        html! {
            <button class={class_name}
                onclick={Callback::from(move |_| {
                    if let Some(f) = on_click.clone() {
                        f();
                    }
                })}>
                { &self.props.label }
            </button>
        }
    }

    fn set_props(&mut self, props: ButtonProps) -> &mut Self {
        self.props = props;
        self
    }

    fn add_class(&mut self, custom_class: &str) -> &mut Self {
        self.props.add_class(custom_class);
        self
    }
}