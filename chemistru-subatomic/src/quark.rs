use crate::{anti_particle_pair, blank_impl, force::StrongForce, subatomic_impl, traits::Fermion};

pub trait Quark: Fermion + StrongForce {}

pub struct Up;
pub struct Down;
pub struct Strange;
pub struct Charm;
pub struct Top;
pub struct Bottom;

pub struct AntiUp;
pub struct AntiDown;
pub struct AntiStrange;
pub struct AntiCharm;
pub struct AntiTop;
pub struct AntiBottom;

subatomic_impl!(Up, AntiUp, 2.2, (1 / 2));
subatomic_impl!(Down, AntiDown, 4.7,(1 / 2));
subatomic_impl!(Charm, AntiCharm, 1280.0, (1 / 2));
subatomic_impl!(Strange, AntiStrange, 96.0, (1 / 2));
subatomic_impl!(Top, AntiTop, 173100.0, (1 / 2));
subatomic_impl!(Bottom, AntiBottom, 4180.0, (1 / 2));

blank_impl!(Up, Fermion, StrongForce, Quark,);
blank_impl!(Down, Fermion, StrongForce, Quark);
blank_impl!(Charm, Fermion, StrongForce, Quark);
blank_impl!(Strange, Fermion, StrongForce, Quark);
blank_impl!(Top, Fermion, StrongForce, Quark);
blank_impl!(Bottom, Fermion, StrongForce, Quark);

blank_impl!(AntiUp, Fermion, StrongForce, Quark);
blank_impl!(AntiDown, Fermion, StrongForce, Quark);
blank_impl!(AntiCharm, Fermion, StrongForce, Quark);
blank_impl!(AntiStrange, Fermion, StrongForce, Quark);
blank_impl!(AntiTop, Fermion, StrongForce, Quark);
blank_impl!(AntiBottom, Fermion, StrongForce, Quark);

anti_particle_pair!(Up, AntiUp);
anti_particle_pair!(Down, AntiDown);
anti_particle_pair!(Charm, AntiCharm);
anti_particle_pair!(Strange, AntiStrange);
anti_particle_pair!(Top, AntiTop);
anti_particle_pair!(Bottom, AntiBottom);
