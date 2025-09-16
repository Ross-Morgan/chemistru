#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct TableData {
    pub(crate) position: (u8, u8),
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
    pub const fn new(position: (u8, u8)) -> Self {
        Self { position }
    }
}
