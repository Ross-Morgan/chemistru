/// Non-numerical data about the element
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct MiscData {
    pub(crate) appearance: Option<&'static str>,
    pub(crate) discovered_by: Option<&'static str>,
    pub(crate) named_by: Option<&'static str>,
    pub(crate) spectral_img: Option<&'static str>,
    pub(crate) source: &'static str,
    pub(crate) cpk_color: Option<&'static str>,
}

impl MiscData {
    /// Short description of colour
    #[must_use]
    pub const fn appearance(&self) -> Option<&'static str> {
        self.appearance
    }

    /// Name of person or institution that discovered the element
    #[must_use]
    pub const fn discovered_by(&self) -> Option<&'static str> {
        self.discovered_by
    }

    /// Name of person or institution that named the element
    #[must_use]
    pub const fn named_by(&self) -> Option<&'static str> {
        self.named_by
    }

    /// Wikimedia url to page containing image of spectrum (not direct link to image)
    #[must_use]
    pub const fn spectral_image_url(&self) -> Option<&'static str> {
        self.spectral_img
    }

    /// Wikipedia page of element
    #[must_use]
    pub const fn source(&self) -> &'static str {
        self.source
    }

    /// Conventional ball-and-stick colour for element
    #[must_use]
    pub const fn cpk_color(&self) -> Option<&'static str> {
        self.cpk_color
    }

    #[allow(clippy::too_many_arguments)]
    #[must_use]
    pub const fn new(
        appearance: Option<&'static str>,
        discovered_by: Option<&'static str>,
        named_by: Option<&'static str>,
        spectral_img: Option<&'static str>,
        source: &'static str,
        cpk_color: Option<&'static str>,
    ) -> Self {
        Self {
            appearance,
            discovered_by,
            named_by,
            spectral_img,
            source,
            cpk_color,
        }
    }
}
