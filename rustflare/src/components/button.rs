use crate::core::ButtonProps;
use std::rc::Rc;

pub struct Button {
    pub props: ButtonProps,
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
        self.props.on_click = Some(Rc::new(Box::new(f)));
    }
}