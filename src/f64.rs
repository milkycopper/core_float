use crate::core_float_traits::{
    CoreFloat, CoreFloatAssociatedConsts, CoreFloatBasicMathConsts, FromF32F64,
};
use crate::impl_core_float_macros::{
    impl_core_float, impl_core_float_associated_consts, impl_core_float_popular_consts,
};
use core::{cmp::Ordering, num::FpCategory};

impl FromF32F64 for f64 {
    fn from_f32(value: f32) -> Self {
        value as f64
    }
    fn from_f64(value: f64) -> Self {
        value
    }
}

impl_core_float_associated_consts!(f64);
impl_core_float_popular_consts!(f64);
impl_core_float!(f64);
