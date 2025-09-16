#[macro_export]
macro_rules! blank_impl {
    ($ty:ty, $($tr:ty),+ $(,)?) => {
        $(
            impl $tr for $ty {}
        )+
    };
}

#[macro_export]
macro_rules! subatomic_impl {
    ($ty:ty, $at:ty, $mass:literal, ($spin_top:literal / $spin_bottom:literal)) => {
        $crate::subatomic_single_impl!($ty, $mass, ($spin_top / $spin_bottom));
        $crate::subatomic_single_impl!($at, $mass, ($spin_top / $spin_bottom));
    };
}

#[macro_export]
macro_rules! subatomic_single_impl {
    ($ty:ty, $mass:literal, ($spin_top:literal / $spin_bottom:literal)) => {
        impl $crate::traits::SubAtomic for $ty {
            fn spin_quantum_number(&self) -> $crate::spin::Spin {
                $crate::spin::Spin::new(fraction::Fraction::new($spin_top, $spin_bottom))
            }

            fn mass(&self) -> $crate::traits::MeVC2 {
                $mass
            }
        }
    };
}

#[macro_export]
macro_rules! charges_to_string_impl {
    (base $c:literal, - $t_neg:ty, 0 $t_neu:ty, + $t_pos:ty) => {
        impl ToString for $t_neu {
            fn to_string(&self) -> String {
                format!("{}⁰", $c)
            }
        }

        charges_to_string_impl!(base $c, - $t_neg, + $t_pos);
    };

    (base $c:literal, - $t_neg:ty, + $t_pos:ty) => {
        impl ToString for $t_neg {
            fn to_string(&self) -> String {
                format!("{}⁻", $c)
            }
        }

        impl ToString for $t_pos {
            fn to_string(&self) -> String {
                format!("{}⁺", $c)
            }
        }
    };
}
