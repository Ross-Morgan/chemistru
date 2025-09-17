use intentional::CastFrom;
use serde::Deserialize;

use crate::{
    Element,
    atomic::AtomicData,
    electron::{ElectronConfiguration, ElectronData},
    misc::MiscData,
    physical::{Phase, PhysicalData},
    table::{Category, TableData},
};

#[derive(Clone, Debug, Deserialize, PartialEq, PartialOrd)]
pub struct RawElement {
    pub(crate) name: &'static str,
    pub(crate) appearance: Option<&'static str>,
    pub(crate) atomic_mass: f64,
    pub(crate) boil: Option<f64>,
    pub(crate) category: &'static str,
    pub(crate) density: Option<f64>,
    pub(crate) discovered_by: Option<&'static str>,
    pub(crate) melt: Option<f64>,
    pub(crate) molar_heat: Option<f64>,
    pub(crate) named_by: Option<&'static str>,
    pub(crate) number: u8,
    pub(crate) period: u8,
    pub(crate) phase: &'static str,
    pub(crate) source: &'static str,
    pub(crate) spectral_img: Option<&'static str>,
    summary: String,
    pub(crate) symbol: &'static str,
    pub(crate) xpos: u8,
    pub(crate) ypos: u8,
    pub(crate) shells: Vec<u8>,
    pub(crate) electron_configuration: &'static str,
    pub(crate) electron_configuration_semantic: &'static str,
    pub(crate) electron_affinity: Option<f64>,
    pub(crate) electronegativity_pauling: Option<f64>,
    pub(crate) ionization_energies: Vec<f64>,
    pub(crate) cpk_hex: Option<&'static str>,
}

impl RawElement {
    pub fn sanitise(self) -> Element {
        let ionization_energies: Option<&'static [f64]> = if self.ionization_energies.is_empty() {
            None
        } else {
            Some(Box::new(self.ionization_energies).leak())
        };

        let shells = Box::new(self.shells).leak();

        let atomic_data = AtomicData {
            atomic_mass: self.atomic_mass,
            atomic_number: self.number,
            mass_number: CastFrom::from_cast(self.atomic_mass.floor()),
        };

        let physical_data = PhysicalData {
            boiling_point: self.boil,
            melting_point: self.melt,
            density: self.density,
            molar_heat_capacity: self.molar_heat,
            phase_in_standard_conditions: Phase::from(self.phase),
        };

        let electron_data = ElectronData {
            configuration: ElectronConfiguration::from(self.electron_configuration),
            affinity: self.electron_affinity,
            electronegativity: self.electronegativity_pauling,
            ionization_energies,
            shells,
        };

        let table_data = TableData {
            position: (self.xpos, self.ypos),
            category: Category::from(self.category),
        };

        let misc_data = MiscData {
            appearance: self.appearance,
            discovered_by: self.discovered_by,
            named_by: self.named_by,
            source: self.source,
            spectral_img: self.spectral_img,
            cpk_color: self.cpk_hex,
        };

        Element {
            name: self.name,
            symbol: self.symbol,
            atomic_data,
            physical_data,
            electron_data,
            table_data,
            misc_data,
        }
    }
}
