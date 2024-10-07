#[derive(Clone, Debug)]
pub enum ButtonVariant {
    Default,
    Secondary,
    Destructive,
    Outline,
    Ghost,
    Link,
}

impl ButtonVariant {
    pub fn to_class(&self) -> &str {
        match self {
            Self::Default => "bg-[#3b82f6] text-white hover:bg-[#3b82f6]/90", 
            Self::Secondary => "bg-[#6b7280] text-white hover:bg-[#6b7280]/80",
            Self::Destructive => "bg-[#ef4444] text-white hover:bg-[#ef4444]/90",
            Self::Outline => "border border-[#d1d5db] bg-[#f9fafb] hover:bg-[#f59e0b] hover:text-black",
            Self::Ghost => "hover:bg-[#f59e0b] hover:text-black",
            Self::Link => "text-[#3b82f6] underline-offset-4 hover:underline",
        }
    }
}
