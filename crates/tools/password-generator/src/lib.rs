//! A configurable and secure password generation library with strength estimation.

mod charsets;
mod errors;
mod generator;
mod settings;
mod strength;

pub use errors::PasswordGeneratorError;
pub use generator::{generate, generate_with_rng};
pub use settings::{PasswordGeneratorSettings, PasswordGeneratorSettingsBuilder};
pub use strength::{PasswordStrength, PasswordStrengthReport, estimate_strength};
