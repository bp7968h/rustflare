pub mod core;
pub mod components;

// #[cfg(all(feature = "dioxus", feature = "yew"))]
// compile_error!("feature \"dioxus\" and feature \"yew\" cannot be enabled at the same time");

#[cfg(feature = "dioxus")]
pub mod dioxus_adapter;

#[cfg(feature = "yew")]
pub mod yew_adapter;

pub use components::*;