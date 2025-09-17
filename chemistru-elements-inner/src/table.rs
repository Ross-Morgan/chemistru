use strum_macros::IntoStaticStr;

/// Data pertaining to the element's position in the periodic table
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct TableData {
    pub(crate) position: (u8, u8),
    pub(crate) category: Category,
}

impl TableData {
    #[must_use]
    pub const fn group(self) -> u8 {
        self.position.0
    }

    #[must_use]
    pub const fn period(self) -> u8 {
        self.position.1
    }

    #[must_use]
    pub const fn category(&self) -> &Category {
        &self.category
    }

    #[must_use]
    pub const fn new(position: (u8, u8), category: Category) -> Self {
        Self { position, category }
    }
}

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord, IntoStaticStr)]
pub enum Category {
    DiatomicNonmetal,
    NobleGas,
    AlkaliMetal,
    AlkalineEarthMetal,
    Metalloid,
    PolyatomicNonmetal,
    PostTransitionMetal,
    TransitionMetal,
    Lanthanide,
    Actinide,
    Unknown,
}

impl From<&str> for Category {
    fn from(value: &str) -> Self {
        match value {
            "diatomic nonmetal" => Self::DiatomicNonmetal,
            "noble gas" => Self::NobleGas,
            "alkali metal" => Self::AlkaliMetal,
            "alkaline earth metal" => Self::AlkalineEarthMetal,
            "metalloid" => Self::Metalloid,
            "polyatomic nonmetal" => Self::PolyatomicNonmetal,
            "post-transition metal" => Self::PostTransitionMetal,
            "transition metal" => Self::TransitionMetal,
            "lanthanide" => Self::Lanthanide,
            "actinide" => Self::Actinide,
            _ => Self::Unknown,
        }
    }
}
