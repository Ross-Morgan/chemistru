use strum_macros::IntoStaticStr;

/// Data pertaining to the element's physical properties
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub struct PhysicalData {
    pub(crate) boiling_point: Option<f64>,
    pub(crate) melting_point: Option<f64>,
    pub(crate) density: Option<f64>,
    pub(crate) molar_heat_capacity: Option<f64>,
    pub(crate) phase_in_standard_conditions: Phase,
}

impl PhysicalData {
    /// Density in g/cm³
    #[must_use]
    pub const fn density(&self) -> Option<f64> {
        self.density
    }

    /// Boiling point in K
    #[must_use]
    pub const fn boiling_point(&self) -> Option<f64> {
        self.boiling_point
    }

    /// Melting point in K
    #[must_use]
    pub const fn melting_point(&self) -> Option<f64> {
        self.melting_point
    }

    /// Heat capacity in J/(K⋅mol)
    #[must_use]
    pub const fn molar_heat_capacity(&self) -> Option<f64> {
        self.molar_heat_capacity
    }

    #[must_use]
    pub const fn phase_in_standard_conditions(&self) -> Phase {
        self.phase_in_standard_conditions
    }

    #[must_use]
    pub const fn new(
        boiling_point: Option<f64>,
        melting_point: Option<f64>,
        density: Option<f64>,
        molar_heat_capacity: Option<f64>,
        phase_in_standard_conditions: Phase,
    ) -> Self {
        Self {
            boiling_point,
            melting_point,
            density,
            molar_heat_capacity,
            phase_in_standard_conditions,
        }
    }
}

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord, IntoStaticStr)]
pub enum Phase {
    Solid,
    Liquid,
    Gas,
}

impl From<&str> for Phase {
    fn from(value: &str) -> Self {
        match value {
            "Solid" => Self::Solid,
            "Liquid" => Self::Liquid,
            "Gas" => Self::Gas,
            s => panic!("Expected Solid, Liquid, or Gas; found {s}"),
        }
    }
}
