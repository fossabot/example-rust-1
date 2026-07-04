//! Configurable password policy validation, independent of the heuristic
//! strength score in [`crate::strength`].

use crate::blocklist::is_blocklisted;

/// A single way a password can fail a [`PasswordPolicy`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Violation {
    TooShort,
    MissingUpper,
    MissingLower,
    MissingDigit,
    MissingSpecial,
    Blocklisted,
}

/// A configurable set of password requirements.
pub struct PasswordPolicy {
    pub min_length: usize,
    pub require_upper: bool,
    pub require_lower: bool,
    pub require_digit: bool,
    pub require_special: bool,
}

impl Default for PasswordPolicy {
    fn default() -> Self {
        PasswordPolicy {
            min_length: 8,
            require_upper: true,
            require_lower: true,
            require_digit: true,
            require_special: false,
        }
    }
}

impl std::fmt::Display for Violation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let message = match self {
            Violation::TooShort => "password is too short",
            Violation::MissingUpper => "missing an uppercase letter",
            Violation::MissingLower => "missing a lowercase letter",
            Violation::MissingDigit => "missing a digit",
            Violation::MissingSpecial => "missing a special character",
            Violation::Blocklisted => "password is too common",
        };
        write!(f, "{message}")
    }
}

impl PasswordPolicy {
    /// A demanding policy: twelve characters minimum, every character class
    /// required.
    pub fn strict() -> Self {
        PasswordPolicy {
            min_length: 12,
            require_upper: true,
            require_lower: true,
            require_digit: true,
            require_special: true,
        }
    }

    /// A permissive policy for low-stakes accounts: only a minimum length.
    pub fn relaxed() -> Self {
        PasswordPolicy {
            min_length: 6,
            require_upper: false,
            require_lower: false,
            require_digit: false,
            require_special: false,
        }
    }

    /// Returns every requirement `password` fails to meet, in a fixed order.
    pub fn validate(&self, password: &str) -> Vec<Violation> {
        let mut violations = Vec::new();

        if is_blocklisted(password) {
            violations.push(Violation::Blocklisted);
        }

        if password.chars().count() < self.min_length {
            violations.push(Violation::TooShort);
        }

        if self.require_upper && !password.chars().any(|c| c.is_ascii_uppercase()) {
            violations.push(Violation::MissingUpper);
        }

        if self.require_lower && !password.chars().any(|c| c.is_ascii_lowercase()) {
            violations.push(Violation::MissingLower);
        }

        if self.require_digit && !password.chars().any(|c| c.is_ascii_digit()) {
            violations.push(Violation::MissingDigit);
        }

        if self.require_special && !password.chars().any(|c| !c.is_ascii_alphanumeric()) {
            violations.push(Violation::MissingSpecial);
        }

        violations
    }

    /// Returns true if `password` satisfies every requirement.
    pub fn is_valid(&self, password: &str) -> bool {
        self.validate(password).is_empty()
    }

    /// Renders a human-readable summary of this policy's requirements, e.g.
    /// for display in a signup form's password hint text.
    pub fn describe(&self) -> String {
        let mut parts = vec![format!("at least {} characters", self.min_length)];

        if self.require_upper {
            parts.push("an uppercase letter".to_string());
        }
        if self.require_lower {
            parts.push("a lowercase letter".to_string());
        }
        if self.require_digit {
            parts.push("a digit".to_string());
        }
        if self.require_special {
            parts.push("a special character".to_string());
        }

        format!("Requires {}.", parts.join(", "))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_policy_accepts_a_good_password() {
        let policy = PasswordPolicy::default();
        assert!(policy.is_valid("Correct1Horse"));
    }

    #[test]
    fn flags_too_short() {
        let policy = PasswordPolicy::default();
        assert!(policy.validate("Ab1").contains(&Violation::TooShort));
    }

    #[test]
    fn flags_missing_upper() {
        let policy = PasswordPolicy::default();
        assert!(policy
            .validate("lowercase1")
            .contains(&Violation::MissingUpper));
    }

    #[test]
    fn flags_missing_lower() {
        let policy = PasswordPolicy::default();
        assert!(policy
            .validate("UPPERCASE1")
            .contains(&Violation::MissingLower));
    }

    #[test]
    fn flags_missing_digit() {
        let policy = PasswordPolicy::default();
        assert!(policy
            .validate("NoDigitsHere")
            .contains(&Violation::MissingDigit));
    }

    #[test]
    fn flags_blocklisted() {
        let policy = PasswordPolicy::default();
        assert!(policy.validate("password").contains(&Violation::Blocklisted));
    }

    #[test]
    fn formats_too_short_as_a_message() {
        assert_eq!(Violation::TooShort.to_string(), "password is too short");
    }

    // Note: `require_special` defaults to false and its "true" branch
    // (MissingSpecial) is deliberately left uncovered. `PasswordPolicy::strict`
    // and `::relaxed` are real convenience constructors left untested, and
    // most `Violation` display messages are only reachable through them.
}
