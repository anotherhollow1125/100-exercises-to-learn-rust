// Not much to be exercised on `Sync`, just a thing to remember.
fn outro() -> &'static str {
    "I have a good understanding of Send and Sync!"
}

// about MutexGuard
// https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&gist=d731e57a8ace21df06fff7c620fcb22d
// static require Sync.
// https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&gist=d2a12ac70cf803f86392b7f08c70d8fd
// about RefCell
// https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&gist=2d4d3fae444a5b7055af0f6692030b02

#[cfg(test)]
mod tests {
    use crate::outro;

    #[test]
    fn test_outro() {
        assert_eq!(outro(), "I have a good understanding of Send and Sync!");
    }
}
