use thiserror::Error;

/// Errors that can occur during password generation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Error)]
pub enum PasswordGeneratorError {
    /// No character sets (lowercase, uppercase, numbers, symbols) were selected.
    #[error("no character sets were selected")]
    NoCharacterSetsSelected,
    /// Password length must be greater than zero.
    #[error("password length must be greater than zero")]
    LengthTooShort,
    /// Password length is shorter than the number of selected character classes.
    #[error("password length is shorter than the number of required character classes")]
    LengthInsufficientForRequiredSets,
}
