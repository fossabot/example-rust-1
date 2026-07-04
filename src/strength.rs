//! Heuristic password strength scoring based on length and character variety.

use crate::blocklist::is_blocklisted;

/// A coarse strength rating for a password.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Strength {
    Weak,
    Fair,
    Strong,
    Excellent,
}

/// Scores a password's strength.
///
/// Blocklisted or empty passwords are always `Weak`. Otherwise the score
/// weighs length against how many character classes (lowercase, uppercase,
/// digit, special) are present.
pub fn score_password(password: &str) -> Strength {
    if password.is_empty() || is_blocklisted(password) {
        return Strength::Weak;
    }

    let length = password.chars().count();
    let has_lower = password.chars().any(|c| c.is_ascii_lowercase());
    let has_upper = password.chars().any(|c| c.is_ascii_uppercase());
    let has_digit = password.chars().any(|c| c.is_ascii_digit());
    let has_special = password.chars().any(|c| !c.is_ascii_alphanumeric());

    let variety = [has_lower, has_upper, has_digit, has_special]
        .iter()
        .filter(|&&present| present)
        .count();

    if length < 8 {
        return Strength::Weak;
    }

    if length >= 16 && variety >= 4 {
        return Strength::Excellent;
    }

    if length >= 12 && variety >= 3 {
        return Strength::Strong;
    }

    if variety >= 2 {
        return Strength::Fair;
    }

    Strength::Weak
}

impl std::fmt::Display for Strength {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let label = match self {
            Strength::Weak => "weak",
            Strength::Fair => "fair",
            Strength::Strong => "strong",
            Strength::Excellent => "excellent",
        };
        write!(f, "{label}")
    }
}

impl Strength {
    /// A longer, user-facing description of what this rating means.
    pub fn description(&self) -> &'static str {
        match self {
            Strength::Weak => "This password is easy to guess. Choose another.",
            Strength::Fair => "This password is usable, but could be stronger.",
            Strength::Strong => "This password offers solid protection.",
            Strength::Excellent => "This password is very hard to crack.",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn formats_weak_as_a_label() {
        assert_eq!(Strength::Weak.to_string(), "weak");
    }

    #[test]
    fn empty_password_is_weak() {
        assert_eq!(score_password(""), Strength::Weak);
    }

    #[test]
    fn blocklisted_password_is_weak() {
        assert_eq!(score_password("password"), Strength::Weak);
    }

    #[test]
    fn short_password_is_weak() {
        assert_eq!(score_password("Ab1!"), Strength::Weak);
    }

    #[test]
    fn single_class_long_password_is_weak() {
        assert_eq!(score_password("lowercaseonly"), Strength::Weak);
    }

    #[test]
    fn two_classes_is_fair() {
        assert_eq!(score_password("lowercase123"), Strength::Fair);
    }

    #[test]
    fn twelve_chars_three_classes_is_strong() {
        assert_eq!(score_password("Lowercase123"), Strength::Strong);
    }

    // Note: the sixteen-char / four-class "Excellent" branch is deliberately
    // left uncovered, so branch coverage < line coverage on the dashboard.
}
