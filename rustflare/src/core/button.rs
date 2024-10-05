use std::rc::Rc;
pub struct ButtonProps {
    pub label: String,
    pub on_click: Option<Rc<Box<dyn Fn()>>>,
    pub class: String
}

pub trait ButtonTrait {
    type RenderType;

    fn render(&self) -> Self::RenderType;
    fn set_props(&mut self, props: ButtonProps);
}