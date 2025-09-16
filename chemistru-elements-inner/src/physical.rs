#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub struct PhysicalData {
    pub(crate) boiling_point: Option<f64>,
    pub(crate) melting_point: Option<f64>,
    pub(crate) density: Option<f64>,
    pub(crate) molar_heat_capacity: Option<f64>,
    pub(crate) phase_in_standard_conditions: &'static str,
}

impl PhysicalData {
    #[must_use]
    pub const fn density(&self) -> Option<f64> {
        self.density
    }

    #[must_use]
    pub const fn boiling_point(&self) -> Option<f64> {
        self.boiling_point
    }

    #[must_use]
    pub const fn melting_point(&self) -> Option<f64> {
        self.melting_point
    }

    #[must_use]
    pub const fn molar_heat_capacity(&self) -> Option<f64> {
        self.molar_heat_capacity
    }

    #[must_use]
    pub const fn phase_in_standard_conditions(&self) -> &'static str {
        self.phase_in_standard_conditions
    }

    #[must_use]
    pub const fn new(
        boiling_point: Option<f64>,
        melting_point: Option<f64>,
        density: Option<f64>,
        molar_heat_capacity: Option<f64>,
        phase_in_standard_conditions: &'static str,
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
