use crate::blank_impl;

pub trait Boson {}

#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Gluon;
#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Photon;
#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Z0;
#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct WPlus;
#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct WMinus;

blank_impl!(Gluon, Boson);
blank_impl!(Photon, Boson);
blank_impl!(Z0, Boson);
blank_impl!(WPlus, Boson);
blank_impl!(WMinus, Boson);
