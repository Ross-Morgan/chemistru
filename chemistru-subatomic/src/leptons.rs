use std::marker::PhantomData;

use crate::{
    anti::Matter, anti_particle_pair, blank_impl, force::StrongForce, subatomic_impl, Fermion,
    Lepton,
};

pub struct Electron;
pub struct Positron;

pub struct Muon;
pub struct AntiMuon;

pub struct Tauon;
pub struct AntiTauon;

pub struct Neutrino<T: Lepton + Matter>(std::marker::PhantomData<T>);
pub struct AntiNeutrino<T: Lepton + Matter>(std::marker::PhantomData<T>);

impl<T: Lepton + Matter> Neutrino<T> {
    pub const fn new() -> Self {
        Self(PhantomData)
    }
}

impl<T: Lepton + Matter> AntiNeutrino<T> {
    pub const fn new() -> Self {
        Self(PhantomData)
    }
}

subatomic_impl!(Electron, Positron, 0.511, (1 / 2));
subatomic_impl!(Muon, AntiMuon, 105.66, (1 / 2));
subatomic_impl!(Tauon, AntiTauon, 1776.8, (1 / 2));

blank_impl!(Electron, Fermion, Lepton, StrongForce);
blank_impl!(Positron, Fermion, Lepton, StrongForce);
blank_impl!(Muon, Fermion, Lepton, StrongForce);
blank_impl!(AntiMuon, Fermion, Lepton, StrongForce);
blank_impl!(Tauon, Fermion, Lepton, StrongForce);
blank_impl!(AntiTauon, Fermion, Lepton, StrongForce);

anti_particle_pair!(Electron, Positron);
anti_particle_pair!(Muon, AntiMuon);
anti_particle_pair!(Tauon, AntiTauon);

subatomic_impl!(Neutrino<Electron>, AntiNeutrino<Electron>, 0.0000022, (0 / 1));
