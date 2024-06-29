// Define a trait named `IsEven` that has a method `is_even` that returns a `true` if `self` is
// even, otherwise `false`.
//
// Then implement the trait for `u32` and `i32`.

// PartialEqが問題では登場していないので、記事の方で言及しておきたいかも...？
// -> Operator overloading に登場しているため不要

// おそらく想定回答

/*
trait IsEven {
    fn is_even(&self) -> bool;
}

impl IsEven for u32 {
    fn is_even(&self) -> bool {
        *self % 2 == 0
    }
}

impl IsEven for i32 {
    fn is_even(&self) -> bool {
        *self % 2 == 0
    }
}
*/

// 改善した回答 1
// cargo add numしてCargo.tomlに追加されている

/*
use num::Num;

trait IsEven: Num + Clone + Copy + From<u8> {
    fn is_even(&self) -> bool {
        *self % 2_u8.into() == Self::zero()
    }
}

impl IsEven for u32 {}
impl IsEven for i32 {}
*/

// 改善した回答 2
trait IsEven {
    fn is_even(&self) -> bool;
}

macro_rules! impl_is_even {
    ( $t:ty ) => {
        impl IsEven for $t {
            fn is_even(&self) -> bool {
                *self % 2 == 0
            }
        }
    };
}

impl_is_even!(u32);
impl_is_even!(i32);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_u32_is_even() {
        assert!(42u32.is_even());
        assert!(!43u32.is_even());
    }

    #[test]
    fn test_i32_is_even() {
        assert!(42i32.is_even());
        assert!(!43i32.is_even());
        assert!(0i32.is_even());
        assert!(!(-1i32).is_even());
    }
}
