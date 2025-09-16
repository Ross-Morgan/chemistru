use strum_macros::IntoStaticStr;

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
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

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, IntoStaticStr)]
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
    Unknown { predicted: Box<Category> },
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
            s => {
                if s.starts_with("unknown") {
                    let inner = if s.contains("diatomic nonmetal") {
                        Self::DiatomicNonmetal
                    } else if s.contains("noble gas") {
                        Self::NobleGas
                    } else if s.contains("alkali metal") {
                        Self::AlkaliMetal
                    } else if s.contains("alkaline earth metal") {
                        Self::AlkalineEarthMetal
                    } else if s.contains("metalloid") {
                        Self::Metalloid
                    } else if s.contains("polyatomic nonmetal") {
                        Self::PolyatomicNonmetal
                    } else if s.contains("post-transition metal") {
                        Self::PostTransitionMetal
                    } else if s.contains("transition metal") {
                        Self::TransitionMetal
                    } else if s.contains("lanthanide") {
                        Self::Lanthanide
                    } else if s.contains("actinide") {
                        Self::Actinide
                    } else {
                        panic!("Unrecognised element category '{s}'");
                    };

                    Self::Unknown {
                        predicted: Box::new(inner),
                    }
                } else {
                    panic!("Unrecognised element category '{s}'")
                }
            }
        }
    }
}
