use crate::SubAtomic;

pub struct DecayEquation {
    pub lhs: Vec<Box<dyn SubAtomic>>,
    pub rhs: Vec<Box<dyn SubAtomic>>,
}

impl DecayEquation {
    pub const fn new(lhs: Vec<Box<dyn SubAtomic>>, rhs: Vec<Box<dyn SubAtomic>>) -> Self {
        Self { lhs, rhs }
    }
}
