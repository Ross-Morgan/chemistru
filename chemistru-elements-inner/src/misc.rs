use strum_macros::IntoStaticStr;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct MiscData {
    pub(crate) appearance: Option<&'static str>,
    pub(crate) category: Category,
    pub(crate) discovered_by: Option<&'static str>,
    pub(crate) named_by: Option<&'static str>,
    pub(crate) spectral_img: Option<&'static str>,
    pub(crate) source: &'static str,
    pub(crate) cpk_color: Option<&'static str>,
}

impl MiscData {
    #[must_use]
    pub const fn appearance(&self) -> Option<&'static str> {
        self.appearance
    }

    #[must_use]
    pub const fn category(&self) -> &Category {
        &self.category
    }

    #[must_use]
    pub const fn discovered_by(&self) -> Option<&'static str> {
        self.discovered_by
    }

    #[must_use]
    pub const fn named_by(&self) -> Option<&'static str> {
        self.named_by
    }

    #[must_use]
    pub const fn spectral_image_url(&self) -> Option<&'static str> {
        self.spectral_img
    }

    #[must_use]
    pub const fn source(&self) -> &'static str {
        self.source
    }

    #[must_use]
    pub const fn cpk_color(&self) -> Option<&'static str> {
        self.cpk_color
    }

    #[allow(clippy::too_many_arguments)]
    #[must_use]
    pub const fn new(
        appearance: Option<&'static str>,
        category: Category,
        discovered_by: Option<&'static str>,
        named_by: Option<&'static str>,
        spectral_img: Option<&'static str>,
        source: &'static str,
        cpk_color: Option<&'static str>,
    ) -> Self {
        Self {
            appearance,
            category,
            discovered_by,
            named_by,
            spectral_img,
            source,
            cpk_color,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, IntoStaticStr)]
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
