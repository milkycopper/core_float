macro_rules! impl_core_float_associated_consts {
    ( $float_t: ty ) => {
        impl CoreFloatAssociatedConsts for $float_t {
            const DIGITS: u32 = <$float_t>::DIGITS;
            const EPSILON: Self = <$float_t>::EPSILON;
            const INFINITY: Self = <$float_t>::INFINITY;
            const MANTISSA_DIGITS: u32 = <$float_t>::MANTISSA_DIGITS;
            const MAX: Self = <$float_t>::MAX;
            const MAX_10_EXP: i32 = <$float_t>::MAX_10_EXP;
            const MAX_EXP: i32 = <$float_t>::MAX_EXP;
            const MIN: Self = <$float_t>::MIN;
            const MIN_10_EXP: i32 = <$float_t>::MIN_10_EXP;
            const MIN_EXP: i32 = <$float_t>::MIN_EXP;
            const MIN_POSITIVE: Self = <$float_t>::MIN_POSITIVE;
            const NAN: Self = <$float_t>::NAN;
            const NEG_INFINITY: Self = <$float_t>::NEG_INFINITY;
            const RADIX: u32 = <$float_t>::RADIX;
        }
    };
}

macro_rules! impl_core_float_popular_consts {
    ( $float_t: tt ) => {
        impl CoreFloatBasicMathConsts for $float_t {
            const ZERO: Self = 0.0;
            const ONE: Self = 1.0;
            const TWO: Self = 2.0;
            const HALF: Self = 0.5;
            const E: Self = core::$float_t::consts::E;
            const FRAC_1_PI: Self = core::$float_t::consts::FRAC_1_PI;
            const FRAC_1_SQRT_2: Self = core::$float_t::consts::FRAC_1_SQRT_2;
            const FRAC_2_PI: Self = core::$float_t::consts::FRAC_2_PI;
            const FRAC_2_SQRT_PI: Self = core::$float_t::consts::FRAC_2_SQRT_PI;
            const FRAC_PI_2: Self = core::$float_t::consts::FRAC_PI_2;
            const FRAC_PI_3: Self = core::$float_t::consts::FRAC_PI_3;
            const FRAC_PI_4: Self = core::$float_t::consts::FRAC_PI_4;
            const FRAC_PI_6: Self = core::$float_t::consts::FRAC_PI_6;
            const FRAC_PI_8: Self = core::$float_t::consts::FRAC_PI_8;
            const LN_2: Self = core::$float_t::consts::LN_2;
            const LN_10: Self = core::$float_t::consts::LN_10;
            const LOG2_10: Self = core::$float_t::consts::LOG2_10;
            const LOG2_E: Self = core::$float_t::consts::LOG2_E;
            const LOG10_2: Self = core::$float_t::consts::LOG10_2;
            const LOG10_E: Self = core::$float_t::consts::LOG10_E;
            const PI: Self = core::$float_t::consts::PI;
            const SQRT_2: Self = core::$float_t::consts::SQRT_2;
            const TAU: Self = core::$float_t::consts::TAU;
        }
    };
}

macro_rules! impl_core_float {
    ( $float_t: ty ) => {
        impl CoreFloat for $float_t {
            fn clamp(self, min: Self, max: Self) -> Self {
                <$float_t>::clamp(self, min, max)
            }
            fn classify(self) -> FpCategory {
                <$float_t>::classify(self)
            }
            fn is_finite(self) -> bool {
                <$float_t>::is_finite(self)
            }
            fn is_infinite(self) -> bool {
                <$float_t>::is_infinite(self)
            }
            fn is_nan(self) -> bool {
                <$float_t>::is_nan(self)
            }
            fn is_normal(self) -> bool {
                <$float_t>::is_normal(self)
            }
            fn is_sign_negative(self) -> bool {
                <$float_t>::is_sign_negative(self)
            }
            fn is_sign_positive(self) -> bool {
                <$float_t>::is_sign_positive(self)
            }
            fn is_subnormal(self) -> bool {
                <$float_t>::is_subnormal(self)
            }
            fn max(self, other: Self) -> Self {
                <$float_t>::max(self, other)
            }
            fn min(self, other: Self) -> Self {
                <$float_t>::min(self, other)
            }
            fn recip(self) -> Self {
                <$float_t>::recip(self)
            }
            fn to_degrees(self) -> Self {
                <$float_t>::to_degrees(self)
            }
            fn to_radians(self) -> Self {
                <$float_t>::to_radians(self)
            }
            fn total_cmp(&self, other: &Self) -> Ordering {
                <$float_t>::total_cmp(self, other)
            }
        }
    };
}

pub(crate) use impl_core_float_associated_consts;

pub(crate) use impl_core_float_popular_consts;

pub(crate) use impl_core_float;
