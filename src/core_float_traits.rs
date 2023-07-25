use core::{
    cmp::Ordering,
    fmt::{Debug, Display, UpperExp},
    iter::{Product, Sum},
    num::FpCategory,
    ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Rem, RemAssign, Sub, SubAssign},
};

/// Associated constants for f32/f64
pub trait CoreFloatAssociatedConsts {
    const DIGITS: u32;
    const EPSILON: Self;
    const INFINITY: Self;
    const MANTISSA_DIGITS: u32;
    const MAX: Self;
    const MAX_10_EXP: i32;
    const MAX_EXP: i32;
    const MIN: Self;
    const MIN_10_EXP: i32;
    const MIN_EXP: i32;
    const MIN_POSITIVE: Self;
    const NAN: Self;
    const NEG_INFINITY: Self;
    const RADIX: u32;
}

/// Often used constants for f32/f64
pub trait CoreFloatPopularConsts {
    const ZERO: Self;
    const ONE: Self;
    const TWO: Self;
    const HALF: Self;
}

/// Float type can be converted from f32/f64
pub trait FromF32F64: Sized {
    fn from_f32(value: f32) -> Self;
    fn from_f64(value: f64) -> Self;
}

/// Trait implemented by f32 and f64, providing cross-platform consistent functions for f32/f64 float type.
pub trait CoreFloat:
    CoreFloatAssociatedConsts
    + CoreFloatPopularConsts
    + Copy
    + Clone
    + Debug
    + Default
    + Display
    + Add<Self, Output = Self>
    + AddAssign<Self>
    + Sub<Self, Output = Self>
    + SubAssign<Self>
    + Mul<Self, Output = Self>
    + MulAssign<Self>
    + Div<Self, Output = Self>
    + DivAssign<Self>
    + Rem<Self, Output = Self>
    + RemAssign<Self>
    + Neg<Output = Self>
    + PartialEq
    + PartialOrd
    + Product
    + for<'a> Product<&'a Self>
    + Sum
    + for<'a> Sum<&'a Self>
    + UpperExp
{
    fn clamp(self, min: Self, max: Self) -> Self;
    fn classify(self) -> FpCategory;
    fn is_finite(self) -> bool;
    fn is_infinite(self) -> bool;
    fn is_nan(self) -> bool;
    fn is_normal(self) -> bool;
    fn is_sign_negative(self) -> bool;
    fn is_sign_positive(self) -> bool;
    fn is_subnormal(self) -> bool;
    fn max(self, other: Self) -> Self;
    fn min(self, other: Self) -> Self;
    fn recip(self) -> Self;
    fn to_degrees(self) -> Self;
    fn to_radians(self) -> Self;
    fn total_cmp(&self, other: &Self) -> Ordering;

    fn double(self) -> Self {
        self * Self::TWO
    }

    fn half(self) -> Self {
        self * Self::HALF
    }
}
