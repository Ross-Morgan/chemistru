use std::hint::black_box;

use crate::{ELEMENTS, Element};

#[must_use]
pub fn element_from_atomic_number(n: u8) -> Option<&'static Element> {
    ELEMENTS.get(n as usize - 1)
}

/// Returns static reference to element of the given name
///
/// Names are case-insensitive and several spellings are accepted for the following:
/// - Caesium, Cesium
/// - Aluminium, Aluminum
/// - Sulphur, Sulfur
#[must_use]
pub fn element_from_name(name: &str) -> Option<&'static Element> {
    let name = {
        let lowercase = name.to_lowercase();

        match lowercase.as_str() {
            "aluminum" => "aluminium",
            "caesium" => "cesium",
            "sulphur" => "sulfur",
            _ => name,
        }
    };

    ELEMENTS
        .iter()
        .find(|&n| n.name().to_lowercase() == name.to_lowercase())
}

/// # Panics
///
/// Panics if element vector is empty (practically impossible)
pub fn preload_elements() {
    let a = &ELEMENTS;
    let b = a.first();
    let c = b.expect("Elements vec is empty").clone();
    let _d = black_box(c);
}
