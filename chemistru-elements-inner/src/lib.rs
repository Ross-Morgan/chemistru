//! # Chemistru Elements Inner
//!
//! > Provides same functionality as [`chemistru-elements`](https://docs.rs/chemistru-elements), minus the macro stuff
//!
//! Provides a static vec of all the elements, with data loaded from a JSON file.
//!
//! ## Static Vector
//!
//! The elements are stored in the lazily-initialised vector `chemistru_elements::ELEMENTS`
//!
//! ### Getting Elements By Atomic (Proton) Number
//!
//! ```rust
//! # use chemistru_elements_inner as chemistru_elements;
//! // Atomic (proton) number, in this case, hydrogen
//! let z = 1;
//!
//! // Static reference to the struct representing hydrogen
//! let element = chemistru_elements::utils::element_from_atomic_number(z);
//! ```
//!
//! ### Getting Elements By Name
//!
//! ```rust
//! # use chemistru_elements_inner as chemistru_elements;
//! // Name of element
//! // Case insensitive and accepts multiple spellings
//! // i.e. 'Cesium', 'Caesium', 'CaEsIuM' will all work
//! let name_1 = "caesium";
//! let name_2 = "cesium";
//!
//! let element_1 = chemistru_elements::utils::element_from_name(name_1);
//! let element_2 = chemistru_elements::utils::element_from_name(name_2);
//!
//! assert_eq!(element_1, element_2)
//! ```
//!
//! ### Preloading Elements
//!
//! Since the static vector of `Element`s is created using `lazy_static`, it will not be initialised until it is used (lazy initialisation)
//!
//! This ensures that the static vector of `Element`s is initialised. This is useful if initialising the element vector later would cause some tangible delay for the user.
//!
//! #### Without
//!
//! ```rust
//! # fn operation_user_sees() {}
//! # const ELEMENT: [(); 1] = [()];
//! operation_user_sees();
//!
//! // May cause a tangible delay (interacting with io)
//! let element = ELEMENT[0];
//!
//! operation_user_sees();
//! ```
//!
//! #### With
//!
//! ```rust
//! # fn operation_user_sees() {}
//! # const ELEMENT: [(); 1] = [()];
//! # use chemistru_elements_inner as chemistru_elements;
//! // Pre-initialise the vector of elements
//! chemistru_elements::utils::preload_elements();
//!
//! operation_user_sees();
//!
//! // Virually no delay (trivial operation)
//! let element = ELEMENT[0];
//!
//! operation_user_sees();
//! ```

#[cfg(feature = "to_tokens")]
mod tokens;

mod raw;

pub mod atomic;
pub mod electron;
pub mod misc;
pub mod physical;
pub mod table;
pub mod utils;

use crate::{
    atomic::AtomicData, electron::ElectronData, misc::MiscData, physical::PhysicalData,
    raw::RawElement, table::TableData,
};

use std::fmt::Display;
use std::sync::LazyLock;

#[rustfmt::skip]
pub static ELEMENTS: LazyLock<Vec<Element>> = LazyLock::new(|| {
    let raw_elements: Vec<RawElement> = serde_json::from_str(include_str!("../db.json")).expect("Failed to load json data");
    raw_elements.iter().map(|e| e.clone().sanitise()).collect()
});

/// Basic elemental representation containing a variety of pieces of data about that element.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Element {
    pub(crate) name: &'static str,
    pub(crate) symbol: &'static str,
    pub(crate) atomic_data: AtomicData,
    pub(crate) electron_data: ElectronData,
    pub(crate) physical_data: PhysicalData,
    pub(crate) table_data: TableData,
    pub(crate) misc_data: MiscData,
}

impl Element {
    #[must_use]
    pub const fn name(&self) -> &'static str {
        self.name
    }

    #[must_use]
    pub const fn symbol(&self) -> &'static str {
        self.symbol
    }

    #[must_use]
    pub const fn atomic_data(&self) -> &AtomicData {
        &self.atomic_data
    }

    #[must_use]
    pub const fn electron_data(&self) -> &ElectronData {
        &self.electron_data
    }

    #[must_use]
    pub const fn physical_data(&self) -> &PhysicalData {
        &self.physical_data
    }

    #[must_use]
    pub const fn table_data(&self) -> &TableData {
        &self.table_data
    }

    #[must_use]
    pub const fn misc_data(&self) -> &MiscData {
        &self.misc_data
    }

    #[must_use]
    pub const fn new(
        name: &'static str,
        symbol: &'static str,
        atomic_data: AtomicData,
        electron_data: ElectronData,
        physical_data: PhysicalData,
        table_data: TableData,
        misc_data: MiscData,
    ) -> Self {
        Self {
            name,
            symbol,
            atomic_data,
            electron_data,
            physical_data,
            table_data,
            misc_data,
        }
    }
}

impl Display for Element {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!(
            "{}-{}",
            self.name, self.atomic_data.atomic_number
        ))
    }
}

pub mod prelude {
    pub use super::ELEMENTS;
    pub use super::Element;
    pub use super::utils::element_from_atomic_number;
    pub use super::utils::element_from_name;
    pub use super::utils::preload_elements;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_getting_elements_by_atomic_number() {
        utils::preload_elements();

        let hydrogen = utils::element_from_atomic_number(1);
        let helium = utils::element_from_atomic_number(2);

        assert_eq!(
            hydrogen.expect("We know this exists").name().to_lowercase(),
            "hydrogen"
        );
        assert_eq!(
            helium.expect("We know this exists").name().to_lowercase(),
            "helium"
        );
    }

    #[test]
    fn test_getting_elements_by_name() {
        utils::preload_elements();

        let hydrogen = utils::element_from_name("hydrogen");
        let helium = utils::element_from_name("helium");

        assert_eq!(
            hydrogen
                .expect("We know this exists")
                .atomic_data
                .atomic_number,
            1
        );
        assert_eq!(
            helium
                .expect("We know this exists")
                .atomic_data
                .atomic_number,
            2
        );
    }
}
