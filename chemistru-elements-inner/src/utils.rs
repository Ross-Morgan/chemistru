use crate::{ELEMENTS, Element};

// This import allows docs to link to it without having to fully qualify path
#[allow(unused_imports)]
use std::sync::LazyLock;

/// Returns [`Element`] with the given atomic (proton) number
#[must_use]
pub fn element_from_atomic_number(n: u8) -> Option<&'static Element> {
    ELEMENTS.get(n as usize - 1)
}

/// Returns [`Element`] with the given name
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
        .find(|n| n.name().to_lowercase() == name.to_lowercase())
}

/// Initialises the [`LazyLock`]-wrapped static vector of [`Element`]s
pub fn preload_elements() {
    // Get first
    let _ = std::hint::black_box(ELEMENTS.first());
}
