/// Checks if the given identifier is a valid identifier. Identifiers
/// should satisfy the regex `[a-zA-z][a-zA-Z0-9]*`.
///
/// # Parameters
/// - `id`: The identifier to check.
///
/// # Returns
/// `true` if the identifier is valid, and `false` otherwise.
pub fn is_valid_identifier(id: impl AsRef<str>) -> bool {
    let s = id.as_ref();
    if s.is_empty() {
        return false;
    }

    let mut is_first = true;
    for c in s.chars() {
        if is_first {
            if !c.is_ascii_alphabetic() {
                return false;
            }

            is_first = false;
        }

        // Otherwise, not first character
        if !c.is_ascii_alphanumeric() {
            return false;
        }
    }

    true
}

#[cfg(test)]
mod util_tests {
    use super::*;

    #[test]
    fn test_valid_identifiers() {
        assert!(is_valid_identifier("hello"));
        assert!(is_valid_identifier("TEST"));
        assert!(is_valid_identifier("someVariableName"));
        assert!(is_valid_identifier("something5Times"));
        assert!(is_valid_identifier("cse131"));
    }

    #[test]
    fn test_invalid_identifiers() {
        assert!(!is_valid_identifier("8ball"));
        assert!(!is_valid_identifier("hello world"));
        assert!(!is_valid_identifier(""));
        assert!(!is_valid_identifier("hello!"));
    }
}
