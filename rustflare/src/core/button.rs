use std::rc::Rc;
pub struct ButtonProps {
    pub label: String,
    pub on_click: Option<Rc<Box<dyn Fn()>>>,
    pub class: String
}

pub trait ButtonTrait<T> {
    fn render(&self) -> T;
    fn set_props(&mut self, props: ButtonProps);
}