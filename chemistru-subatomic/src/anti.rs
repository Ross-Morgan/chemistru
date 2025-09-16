use crate::SubAtomic;

pub trait Particle {
    type ANTI;
}

pub trait AntiParticle {
    type ANTI;
}

#[macro_export]
macro_rules! anti_particle_pair {
    ($t1:ty, $t2:ty) => {
        impl $crate::anti::Particle for $t1 {
            type ANTI = $t2;
        }

        impl $crate::anti::AntiParticle for $t2 {
            type ANTI = $t1;
        }
    };
}
