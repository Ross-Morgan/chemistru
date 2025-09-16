use crate::{
    anti_particle_pair, blank_impl, charges_to_string_impl,
    decay::DecayEquation,
    force::StrongForce,
    leptons::{AntiNeutrino, Electron, Neutrino, Positron},
    quark, subatomic_single_impl,
    traits::{Decay, Hadron},
};

pub struct Pion0;
pub struct PionMinus;
pub struct PionPlus;

subatomic_single_impl!(Pion0, 497.611);
subatomic_single_impl!(PionMinus, 493.677);
subatomic_single_impl!(PionPlus, 493.677, ());

blank_impl!(Pion0, StrongForce);
blank_impl!(PionMinus, StrongForce);
blank_impl!(PionPlus, StrongForce);

anti_particle_pair!(PionPlus, PionMinus);

impl Hadron for Pion0 {
    type Composition = ((quark::Up, quark::AntiUp), (quark::Down, quark::AntiDown));
}

impl Hadron for PionMinus {
    type Composition = (quark::Down, quark::AntiUp);
}

impl Hadron for PionPlus {
    type Composition = (quark::Up, quark::AntiDown);
}

charges_to_string_impl!(base 'π', - PionMinus, 0 Pion0, + PionPlus);

impl Decay for PionPlus {
    fn all_possible_decays() -> Vec<DecayEquation> {
        vec![
            DecayEquation::new(
                vec![Box::new(PionPlus)],
                vec![Box::new(Positron), Box::new(Neutrino::<Electron>::new())],
            ),
            DecayEquation::new(
                vec![Box::new(PionPlus)],
                vec![Box::new(Pion0), Box::new(Positron), Box::new(Neutrino::<Electron>::new())],
            )
        ]
    }
}

impl Decay for PionMinus {
    fn all_possible_decays() -> Vec<DecayEquation> {
        vec![
            DecayEquation::new(
                vec![Box::new(PionMinus)],
                vec![
                    Box::new(Electron),
                    Box::new(AntiNeutrino::<Electron>::new()),
                ],
            ),
            DecayEquation::new(
                vec![Box::new(PionMinus)],
                vec![Box::new(Pion0), Box::new(Positron), Box::new(Neutrino::<Electron>::new())],
            )
        ]
    }
}
