use std::rc::Rc;
pub struct ButtonProps {
    pub label: String,
    pub on_click: Option<Rc<Box<dyn Fn()>>>,
    pub class: Option<String>,
}

impl Default for ButtonProps {
    fn default() -> Self {
        Self {
            label: String::new(),
            on_click: None,
            class: None,
        }
    }
}

impl ButtonProps {
    pub fn new(label: &str) -> Self {
        Self {
            label: label.to_string(),
            on_click: None,
            class: None,
        }
    }

    pub fn add_class(&mut self, class: &str) -> &mut Self {
        if let Some(existing_class) = &mut self.class {
            existing_class.push_str(&format!(" {}", class));
        } else {
            self.class = Some(class.to_string());
        }

        self
    }

    pub fn set_on_click<F: Fn() + 'static>(&mut self, callback: F) -> &mut Self {
        self.on_click = Some(Rc::new(Box::new(callback)));
        self
    }
}

pub trait ButtonTrait {
    type RenderType;

    fn render(&self) -> Self::RenderType;

    fn set_props(&mut self, props: ButtonProps) -> &mut Self;

    fn add_class(&mut self, custom_class: &str) -> &mut Self;
}