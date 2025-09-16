use crate::{
    anti_particle_pair, blank_impl, force::StrongForce, quark, subatomic_single_impl,
    traits::{Hadron, Decay}, charges_to_string_impl, decay::DecayEquation,
};

use super::pions::{Pion0, PionMinus, PionPlus};
use crate::leptons::{MuonPlus, Neutrino};

pub struct Kaon0;
pub struct KaonMinus;
pub struct KaonPlus;

subatomic_single_impl!(Kaon0, 497.611, 0);
subatomic_single_impl!(KaonMinus, 493.677, 0);
subatomic_single_impl!(KaonPlus, 493.677, 0);

blank_impl!(Kaon0, StrongForce);
blank_impl!(KaonMinus, StrongForce);
blank_impl!(KaonPlus, StrongForce);

anti_particle_pair!(KaonPlus, KaonMinus);

charges_to_string_impl!(base 'K', - KaonMinus, 0 Kaon0, + KaonPlus);

impl Hadron for Kaon0 {
    type Composition = (quark::Down, quark::AntiStrange);
}

impl Hadron for KaonMinus {
    type Composition = (quark::Strange, quark::AntiUp);
}

impl Hadron for KaonPlus {
    type Composition = (quark::Up, quark::AntiStrange);
}

impl Decay for KaonPlus {
    fn all_possible_decays() -> Vec<DecayEquation> {
        vec![
            DecayEquation::new(
                vec![Box::new(KaonPlus)],
                vec![Box::new(MuonPlus), Box::new(Neutrino::<Muon>)],
            ),
            DecayEquation::new(
                vec![Box::new(KaonPlus)],
                vec![Box::new(PionPlus), Box::new(Pion0)],
            ),
            DecayEquation::new(
                vec![],
                vec![],    
            ),
        ]
    }
}
