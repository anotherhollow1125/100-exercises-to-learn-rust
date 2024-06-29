// TODO: Define a new trait, `Power`, that has a method `power` that raises `self`
//  to the power of `n`.
//  The trait definition and its implementations should be enough to get
//  the tests to compile and pass.
//
// Recommendation: you may be tempted to write a generic implementation to handle
// all cases at once. However, this is fairly complicated and requires the use of
// additional crates (i.e. `num-traits`).
// Even then, it might be preferable to use a simple macro instead to avoid
// the complexity of a highly generic implementation. Check out the
// "Little book of Rust macros" (https://veykril.github.io/tlborm/) if you're
// interested in learning more about it.
// You don't have to though: it's perfectly okay to write three separate
// implementations manually. Venture further only if you're curious.

use num::Num;
use std::collections::HashMap;
use std::hash::Hash;

// キャッシュ要らなかったかも
fn power_inner<N>(n: N, e: u32, memo: &mut HashMap<(N, u32), N>) -> N
where
    N: Num + Hash + Eq + Clone + Copy + std::fmt::Debug,
{
    if e == 0 {
        return N::one();
    }

    if e == 1 {
        return n;
    }

    if let Some(v) = memo.get(&(n, e)) {
        eprintln!("HIT!: {:?}", v);
        return *v;
    }

    let vv = power_inner(n, e / 2, memo);

    let v = vv * vv * if e % 2 == 0 { N::one() } else { n };

    eprintln!("v: {:?}", v);

    memo.insert((n, e), v);
    v
}

trait Power<RHS = Self>: Num + Hash + Eq + Clone + Copy {
    fn power(self, rhs: RHS) -> Self;
}

macro_rules! power_impl {
    ( $(($t:ty, $r:ty))* ) => {$(
        impl Power<$r> for $t {
            fn power(self, rhs: $r) -> Self {
                let mut memo = HashMap::new();

                power_inner(self, rhs.clone().into(), &mut memo)
            }
        }
    )*}
}

power_impl!((u32, u32)(u32, u16)(u32, &u32)(u128, u32));

#[cfg(test)]
mod tests {
    use super::Power;

    #[test]
    fn test_power_u16() {
        let x: u32 = 2_u32.power(3u16);
        assert_eq!(x, 8);
    }

    #[test]
    fn test_power_u32() {
        let x: u32 = 2_u32.power(3u32);
        assert_eq!(x, 8);
    }

    #[test]
    fn test_power_ref_u32() {
        let x: u32 = 2_u32.power(&3u32);
        assert_eq!(x, 8);
    }

    #[test]
    fn test_big_power_u32() {
        let x: u128 = 2_u128.power(123u32);
        assert_eq!(x % 1000_000_007, 914988515u128);
    }
}
