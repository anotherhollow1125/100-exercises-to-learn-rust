// TODO: based on what you learned in this section, replace `todo!()` with
//  the correct **stack size** for the respective type.
#[cfg(test)]
mod tests {
    use std::mem::size_of;

    #[test]
    fn u16_size() {
        // 16 / 8 = 2
        assert_eq!(size_of::<u16>(), 2);
    }

    #[test]
    fn i32_size() {
        // 32 / 8 = 4
        assert_eq!(size_of::<i32>(), 4);
    }

    #[test]
    fn bool_size() {
        // 1バイトあれば足りそう
        assert_eq!(size_of::<bool>(), 1);
    }
}
