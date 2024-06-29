// TODO: Define a new `SaturatingU16` type.
//   It should hold a `u16` value.
//   It should provide conversions from `u16`, `u8`, `&u16` and `&u8`.
//   It should support addition with a right-hand side of type
//   SaturatingU16, u16, &u16, and &SaturatingU16. Addition should saturate at the
//   maximum value for `u16`.
//   It should be possible to compare it with another `SaturatingU16` or a `u16`.
//   It should be possible to print its debug representation.
//
// Tests are located in the `tests` folder—pay attention to the visibility of your types and methods.

#[derive(Debug, Clone, Copy)]
pub struct SaturatingU16(u16);

macro_rules! impl_from {
    ($t:ty) => {
        impl From<$t> for SaturatingU16 {
            fn from(val: $t) -> Self {
                Self(val.clone() as u16)
            }
        }
    };
}

impl_from!(u16);
impl_from!(u8);
impl_from!(&u16);
impl_from!(&u8);

use std::ops::Add;

macro_rules! impl_add {
    ($t:ty) => {
        impl Add<$t> for SaturatingU16 {
            type Output = Self;

            fn add(self, rhs: $t) -> Self::Output {
                self.0
                    .saturating_add(SaturatingU16::from(rhs.clone()).0)
                    .into()
            }
        }
    };
}

impl_add!(SaturatingU16);
impl_add!(&SaturatingU16);
impl_add!(u16);
impl_add!(&u16);

macro_rules! impl_eq {
    ($t:ty) => {
        impl PartialEq<$t> for SaturatingU16 {
            fn eq(&self, other: &$t) -> bool {
                self.0 == SaturatingU16::from(other.clone()).0
            }
        }
    };
}

impl_eq!(SaturatingU16);

impl Eq for SaturatingU16 {}

impl_eq!(u16);
