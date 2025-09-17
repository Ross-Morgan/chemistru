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

pub mod data {
    // pub use chemistru_elements_inner::{atomic, electron, misc, physical, table};
    pub use chemistru_elements_inner::{
        atomic::AtomicData,
        electron::{ElectronConfiguration, ElectronData, Suborbital},
        misc::MiscData,
        physical::{Phase, PhysicalData},
        table::{Category, TableData},
    };
}

#[cfg(feature = "constants")]
pub mod elements {
    chemistru_elements_macro::generate_elements!();
}

pub mod prelude {
    pub use super::utils::{element_from_atomic_number, element_from_name, preload_elements};
    pub use super::{ELEMENTS, Element};
}
