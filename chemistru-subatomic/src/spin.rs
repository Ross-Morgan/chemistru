use crate::traits::SubAtomic;

use fraction::GenericFraction;

#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Spin {
    spin: GenericFraction<u8>,
    isospin: GenericFraction<u8>,
    weak_isospin: GenericFraction<u8>,
}

impl Spin {
    pub const fn new(spin: (i8, u8), isospin: (i8, u8), weak_isospin: (i8, u8)) -> Self {
        match spin.0.cmp(&0i8) {
            std::cmp::Ordering::Greater => 
        }
        
        Self {
            spin: GenericFraction::new()
        }
    }
}

fn tuple_to_fraction(tuple: (i8, u8)) -> GenericFraction<u8> {
    match tuple.0.signum() {
        0 | 1 => GenericFraction::new(tuple),
        -1 => GenericFraction::new_neg(),
        _ => unreachable!(),
    }
}
