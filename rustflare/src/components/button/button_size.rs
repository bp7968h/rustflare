#[derive(Clone, Debug)]
pub enum ButtonSize {
    Small,
    Medium,
    Large,
    Icon,
}

impl ButtonSize {
    pub fn to_class(&self) -> &str {
        match self {
            ButtonSize::Small => "h-9 rounded-md px-3",
            ButtonSize::Medium => "h-10 px-4 py-2",
            ButtonSize::Large => "h-11 rounded-md px-8",
            ButtonSize::Icon => "h-10 w-10",
        }
    }
}
