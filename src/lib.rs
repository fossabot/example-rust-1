//! `pwcheck` — a small password strength and policy library.

mod blocklist;
mod entropy;
mod policy;
mod strength;

pub use blocklist::is_blocklisted;
pub use entropy::{entropy_rating, estimate_entropy_bits};
pub use policy::{PasswordPolicy, Violation};
pub use strength::{score_password, Strength};
