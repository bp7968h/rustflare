pub struct ButtonProps {
    pub label: String,
    pub on_click: Option<Box<dyn Fn()>>,
    pub class: String
}

pub trait ButtonTrait {
    fn render(&self) -> String;
    fn set_props(&mut self, props: ButtonProps);
}