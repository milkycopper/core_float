use crate::core_float_traits::{CoreFloat, CoreFloatAssociatedConsts, CoreFloatPopularConsts};
use crate::impl_core_float_macros::{
    impl_core_float, impl_core_float_associated_consts, impl_core_float_popular_consts,
};
use core::{cmp::Ordering, num::FpCategory};

impl_core_float_associated_consts!(f64);
impl_core_float_popular_consts!(f64);
impl_core_float!(f64, { value as f64 }, value, value);
