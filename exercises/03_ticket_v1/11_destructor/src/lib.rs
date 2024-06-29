// We need some more machinery to write a proper exercise for destructors.
// We'll pick the concept up again in a later chapter after covering traits and
// interior mutability.
fn outro() -> &'static str {
    "I have a basic understanding of destructors!"
}

// Dropping referencesのところ、ライフタイムの話がまだだからなのか参照に対する扱いについてお茶を濁している感がある

#[cfg(test)]
mod tests {
    use crate::outro;

    #[test]
    fn test_outro() {
        assert_eq!(outro(), "I have a basic understanding of destructors!");
    }
}
