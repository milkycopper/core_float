use crate::core_float_traits::{CoreFloat, CoreFloatAssociatedConsts, CoreFloatPopularConsts};
use crate::impl_core_float_macros::{
    impl_core_float, impl_core_float_associated_consts, impl_core_float_popular_consts,
};
use core::{cmp::Ordering, num::FpCategory};

impl_core_float_associated_consts!(f32);
impl_core_float_popular_consts!(f32);
impl_core_float!(f32, value, { value as f32 }, value);
