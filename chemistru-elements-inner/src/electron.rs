use std::fmt::Display;

use const_panic::concat_panic;

/// Data pertaining to the element's energy levels
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub struct ElectronData {
    pub(crate) configuration: ElectronConfiguration,
    pub(crate) affinity: Option<f64>,
    pub(crate) electronegativity: Option<f64>,
    pub(crate) ionization_energies: Option<&'static [f64]>,
    pub(crate) shells: &'static [u8],
}

impl ElectronData {
    #[must_use]
    pub const fn configuration(&self) -> &ElectronConfiguration {
        &self.configuration
    }

    #[must_use]
    pub const fn electron_affinity(&self) -> Option<f64> {
        self.affinity
    }

    #[must_use]
    pub const fn electronegativity(&self) -> Option<f64> {
        self.electronegativity
    }

    #[must_use]
    pub const fn ionization_energies(&self) -> Option<&[f64]> {
        self.ionization_energies
    }

    #[must_use]
    pub const fn shells(&self) -> &[u8] {
        self.shells
    }

    #[must_use]
    pub const fn new(
        configuration: ElectronConfiguration,
        affinity: Option<f64>,
        electronegativity: Option<f64>,
        ionization_energies: Option<&'static [f64]>,
        shells: &'static [u8],
    ) -> Self {
        Self {
            configuration,
            affinity,
            electronegativity,
            ionization_energies,
            shells,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ElectronConfiguration(&'static [Suborbital]);

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Suborbital(u8, u8, u8);

impl From<&str> for ElectronConfiguration {
    fn from(value: &str) -> Self {
        let inner = value
            .split(' ')
            .map(|suborbital| {
                let principal_quantum_number = suborbital
                    .chars()
                    .nth(0)
                    .expect("Suborbital in electron configuration missing principal quantum number")
                    .to_string()
                    .parse::<u8>()
                    .expect("Failed to parse principal quantum number from electron configuration as u8");
                let azimuthal_letter = suborbital
                    .chars()
                    .nth(1)
                    .expect("Suborbital in electron configuration missing azimuthal letter");
                let electron_number = suborbital
                    .get(2..suborbital.len())
                    .expect("Suborbital in electron configuration missing electron number")
                    .parse::<u8>()
                    .expect("Failed to parse electron number from electron configuration as u8");

                let azimuthal_quantum_number = match azimuthal_letter.to_ascii_lowercase() {
                    's' => 0,
                    'p' => 1,
                    'd' => 2,
                    'f' => 3,
                    _ => panic!("Expected s, p, d, or f; found {azimuthal_letter}"),
                };

                Suborbital(
                    principal_quantum_number,
                    azimuthal_quantum_number,
                    electron_number,
                )
            })
            .collect::<Vec<_>>();

        Self(Box::new(inner).leak())
    }
}

impl Suborbital {
    #[must_use]
    pub const fn principal_quantum_number(&self) -> u8 {
        self.0
    }

    #[must_use]
    pub const fn azimuthal_quantum_number(&self) -> u8 {
        self.1
    }

    #[must_use]
    pub const fn block(&self) -> char {
        azimuthal_number_to_char(self.1)
    }

    #[must_use]
    pub const fn electron_number(&self) -> u8 {
        self.2
    }

    #[must_use]
    pub fn to_string_stylized(&self) -> String {
        self.to_string()
    }

    #[must_use]
    pub fn to_string_nonstylized(&self) -> String {
        format!("{}{}{}", self.0, azimuthal_number_to_char(self.1), self.2)
    }

    #[must_use]
    pub const fn new(pqn: u8, aqn: u8, en: u8) -> Self {
        Self(pqn, aqn, en)
    }
}

impl Display for Suborbital {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!(
            "{}{}{}",
            self.0,
            self.1,
            to_superscript(self.2)
        ))
    }
}

impl Display for ElectronConfiguration {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut suborbitals = self.0.iter();

        if let Some(first) = suborbitals.next() {
            f.write_fmt(format_args!("{first}"))?;

            for suborbital in suborbitals {
                f.write_fmt(format_args!(" {suborbital}"))?;
            }
        }

        Ok(())
    }
}

impl ElectronConfiguration {
    #[must_use]
    pub const fn suborbitals(&self) -> &[Suborbital] {
        self.0
    }

    #[must_use]
    pub const fn new(suborbitals: &'static [Suborbital]) -> Self {
        Self(suborbitals)
    }
}

fn to_superscript(n: u8) -> String {
    const SUPERSCRIPT: [char; 10] = ['⁰', '¹', '²', '³', '⁴', '⁵', '⁶', '⁷', '⁸', '⁹'];

    let units_digit = n & 10;
    let tens_digit = ((n - units_digit) / 10) % 10;
    let hundreds_digit = ((n - units_digit - tens_digit) / 100) % 10;

    if hundreds_digit == 0 {
        if tens_digit == 0 {
            format!("{}", SUPERSCRIPT[units_digit as usize])
        } else {
            format!(
                "{}{}",
                SUPERSCRIPT[tens_digit as usize], SUPERSCRIPT[units_digit as usize]
            )
        }
    } else {
        format!(
            "{}{}{}",
            SUPERSCRIPT[hundreds_digit as usize],
            SUPERSCRIPT[tens_digit as usize],
            SUPERSCRIPT[units_digit as usize]
        )
    }
}

const fn azimuthal_number_to_char(n: u8) -> char {
    match n {
        0 => 's',
        1 => 'p',
        2 => 'd',
        3 => 'f',
        _ => concat_panic!("Expected s, p, d, or f; found ", n),
    }
}
