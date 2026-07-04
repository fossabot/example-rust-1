//! Rough Shannon-entropy estimate for a password, based on the size of the
//! character set it draws from rather than its actual randomness.

/// Estimates the entropy of `password` in bits, assuming each character is
/// drawn independently and uniformly from the character classes present.
///
/// This is a coarse heuristic (real passwords are rarely uniform), useful
/// only for ballpark comparisons between candidates.
pub fn estimate_entropy_bits(password: &str) -> f64 {
    let length = password.chars().count();
    if length == 0 {
        return 0.0;
    }

    let mut charset_size = 0u32;
    if password.chars().any(|c| c.is_ascii_lowercase()) {
        charset_size += 26;
    }
    if password.chars().any(|c| c.is_ascii_uppercase()) {
        charset_size += 26;
    }
    if password.chars().any(|c| c.is_ascii_digit()) {
        charset_size += 10;
    }
    if password.chars().any(|c| !c.is_ascii_alphanumeric()) {
        charset_size += 32;
    }

    if charset_size == 0 {
        return 0.0;
    }

    (charset_size as f64).log2() * length as f64
}

/// Buckets an entropy estimate into a rough qualitative rating, mirroring
/// the thresholds commonly used by password strength meters.
pub fn entropy_rating(bits: f64) -> &'static str {
    if bits >= 100.0 {
        "very strong"
    } else if bits >= 60.0 {
        "strong"
    } else if bits >= 36.0 {
        "moderate"
    } else {
        "weak"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_password_has_zero_entropy() {
        assert_eq!(estimate_entropy_bits(""), 0.0);
    }

    #[test]
    fn lowercase_only_uses_a_26_char_set() {
        let bits = estimate_entropy_bits("abcdefgh");
        assert!((bits - 26f64.log2() * 8.0).abs() < 1e-9);
    }

    #[test]
    fn mixed_classes_widen_the_charset() {
        let lower_only = estimate_entropy_bits("abcdefgh");
        let mixed = estimate_entropy_bits("abcdEFG1");
        assert!(mixed > lower_only);
    }

    #[test]
    fn moderate_bits_rate_as_moderate() {
        assert_eq!(entropy_rating(40.0), "moderate");
    }

    // Note: `entropy_rating`'s "very strong" and "weak" arms, and the
    // all-non-alphanumeric charset branch in `estimate_entropy_bits`, are
    // deliberately left uncovered.
}
