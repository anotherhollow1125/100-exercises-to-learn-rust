// TODO: Implement the `From` trait for the `WrappingU32` type to make `example` compile.

#[derive(Debug, PartialEq, Eq)]
pub struct WrappingU32 {
    value: u32,
}

impl From<u32> for WrappingU32 {
    fn from(value: u32) -> Self {
        Self { value }
    }
}

pub fn example() {
    let u32_into_wp: WrappingU32 = 42.into();
    let wp_from_u32 = WrappingU32::from(42);

    println!(
        "{:?} == {:?}: {:?}",
        u32_into_wp,
        wp_from_u32,
        u32_into_wp == wp_from_u32
    );
}
