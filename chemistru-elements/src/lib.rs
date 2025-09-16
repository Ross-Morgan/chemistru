//! # Chemistru Elements
//!
//!
//!
//!
//!
//!
//!
//!
//!
//!
//!
//!
//!
//!
//!
//!
//!
//!
//!

pub use chemistru_elements_inner::ELEMENTS;
pub use chemistru_elements_inner::Element;
pub use chemistru_elements_inner::utils;

#[cfg(feature = "constants")]
use chemistru_elements_macro::generate_elements;

pub mod prelude {
    pub use super::utils::{element_from_atomic_number, element_from_name, preload_elements};
    pub use super::{ELEMENTS, Element};
}

pub mod data {
    pub use chemistru_elements_inner::{atomic, electron, misc, physical, table};
}

#[cfg(feature = "constants")]
pub mod elements {
    generate_elements!();
}
