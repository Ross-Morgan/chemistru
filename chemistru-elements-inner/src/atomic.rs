use intentional::CastFrom;

#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub struct AtomicData {
    pub(crate) atomic_mass: f64,
    pub(crate) atomic_number: u8,
    pub(crate) mass_number: u16,
}

impl AtomicData {
    #[must_use]
    pub const fn proton_count(&self) -> u8 {
        self.atomic_number
    }

    #[must_use]
    pub fn neutron_count(&self) -> u8 {
        CastFrom::from_cast(self.mass_number - u16::from(self.proton_count()))
    }

    #[must_use]
    pub const fn atomic_mass(&self) -> f64 {
        self.atomic_mass
    }

    #[must_use]
    pub const fn nucleon_count(&self) -> u16 {
        self.mass_number
    }

    #[must_use]
    pub const fn new(atomic_mass: f64, atomic_number: u8, mass_number: u16) -> Self {
        Self {
            atomic_mass,
            atomic_number,
            mass_number,
        }
    }
}
