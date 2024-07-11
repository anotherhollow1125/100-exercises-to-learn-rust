// TODO: Define a function named `lowercase` that converts all characters in a string to lowercase,
//  modifying the input in place.
//  Does it need to take a `&mut String`? Does a `&mut str` work? Why or why not?

// Answer:
// &mut str has `make_ascii_lowercase`, so we can take a &mut str argument.
// However, static string literal can't take mutable reference, so `&mut String` is enough for this purpose in many cases.
// In this exercise, test case calls `as_mut_str` method so I need to take the `&mut str` argument.

fn lowercase(s: &mut str) {
    // fn lowercase(s: &mut String) {
    s.make_ascii_lowercase();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty() {
        let mut s = String::from("");
        lowercase(&mut s);
        assert_eq!(s, "");
    }

    #[test]
    fn one_char() {
        let mut s = String::from("A");
        lowercase(&mut s);
        assert_eq!(s, "a");
    }

    #[test]
    fn multiple_chars() {
        let mut s = String::from("Hello, World!");
        lowercase(&mut s);
        assert_eq!(s, "hello, world!");
    }

    #[test]
    fn mut_slice() {
        let mut s = "Hello, World!".to_string();
        lowercase(s.as_mut_str());
        assert_eq!(s, "hello, world!");
    }
}
