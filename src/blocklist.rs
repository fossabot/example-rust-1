//! A tiny denylist of passwords that are common enough to reject outright,
//! regardless of how they score on length or character variety.

const COMMON_PASSWORDS: &[&str] = &[
    "password", "123456", "12345678", "qwerty", "letmein", "111111", "admin",
    "welcome", "iloveyou", "abc123",
];

/// Returns true if `password` (compared case-insensitively) is one of a
/// small set of well-known weak passwords.
pub fn is_blocklisted(password: &str) -> bool {
    let lower = password.to_lowercase();
    COMMON_PASSWORDS.iter().any(|&p| p == lower)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn flags_exact_match() {
        assert!(is_blocklisted("password"));
    }

    #[test]
    fn flags_case_insensitively() {
        assert!(is_blocklisted("PaSsWoRd"));
    }

    #[test]
    fn allows_uncommon_password() {
        assert!(!is_blocklisted("Tr0ub4dor&3xample"));
    }
}
