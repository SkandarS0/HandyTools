use rand::Rng;

use crate::errors::PasswordGeneratorError;
use crate::generator::{generate, generate_with_rng};

/// Configuration options for password generation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct PasswordGeneratorSettings {
    pub length: u8,
    pub include_numbers: bool,
    pub include_symbols: bool,
    pub include_uppercase: bool,
    pub include_lowercase: bool,
    pub include_similar_characters: bool,
    pub include_ambiguous_characters: bool,
}

impl Default for PasswordGeneratorSettings {
    fn default() -> Self {
        Self {
            length: 16,
            include_numbers: true,
            include_symbols: true,
            include_uppercase: true,
            include_lowercase: true,
            include_similar_characters: false,
            include_ambiguous_characters: false,
        }
    }
}

impl PasswordGeneratorSettings {
    /// Returns a new builder initialized with default settings.
    pub fn builder() -> PasswordGeneratorSettingsBuilder {
        PasswordGeneratorSettingsBuilder::default()
    }

    /// Generates a password using the default system entropy source.
    pub fn generate(&self) -> Result<String, PasswordGeneratorError> {
        generate(*self)
    }

    /// Generates a password using a provided random number generator.
    pub fn generate_with_rng<R: Rng + ?Sized>(
        &self,
        rng: &mut R,
    ) -> Result<String, PasswordGeneratorError> {
        generate_with_rng(*self, rng)
    }
}

/// Fluent builder for constructing [`PasswordGeneratorSettings`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PasswordGeneratorSettingsBuilder {
    length: u8,
    include_numbers: bool,
    include_symbols: bool,
    include_uppercase: bool,
    include_lowercase: bool,
    include_similar_characters: bool,
    include_ambiguous_characters: bool,
}

impl Default for PasswordGeneratorSettingsBuilder {
    fn default() -> Self {
        let defaults = PasswordGeneratorSettings::default();
        Self {
            length: defaults.length,
            include_numbers: defaults.include_numbers,
            include_symbols: defaults.include_symbols,
            include_uppercase: defaults.include_uppercase,
            include_lowercase: defaults.include_lowercase,
            include_similar_characters: defaults.include_similar_characters,
            include_ambiguous_characters: defaults.include_ambiguous_characters,
        }
    }
}

impl PasswordGeneratorSettingsBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn length(mut self, length: u8) -> Self {
        self.length = length;
        self
    }

    pub fn include_numbers(mut self, value: bool) -> Self {
        self.include_numbers = value;
        self
    }

    pub fn include_symbols(mut self, value: bool) -> Self {
        self.include_symbols = value;
        self
    }

    pub fn include_uppercase(mut self, value: bool) -> Self {
        self.include_uppercase = value;
        self
    }

    pub fn include_lowercase(mut self, value: bool) -> Self {
        self.include_lowercase = value;
        self
    }

    pub fn include_similar_characters(mut self, value: bool) -> Self {
        self.include_similar_characters = value;
        self
    }

    pub fn include_ambiguous_characters(mut self, value: bool) -> Self {
        self.include_ambiguous_characters = value;
        self
    }

    /// Convenience preset: letters and numbers only (disables symbols).
    pub fn alphanumeric_only(mut self) -> Self {
        self.include_symbols = false;
        self.include_uppercase = true;
        self.include_lowercase = true;
        self.include_numbers = true;
        self
    }

    /// Convenience preset: disables inclusion of visually similar and ambiguous characters.
    pub fn easy_to_read(mut self) -> Self {
        self.include_similar_characters = false;
        self.include_ambiguous_characters = false;
        self
    }

    /// Consumes the builder and returns the configured [`PasswordGeneratorSettings`].
    pub fn build(self) -> PasswordGeneratorSettings {
        PasswordGeneratorSettings {
            length: self.length,
            include_numbers: self.include_numbers,
            include_symbols: self.include_symbols,
            include_uppercase: self.include_uppercase,
            include_lowercase: self.include_lowercase,
            include_similar_characters: self.include_similar_characters,
            include_ambiguous_characters: self.include_ambiguous_characters,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn builder_alphanumeric_preset() {
        let settings = PasswordGeneratorSettings::builder()
            .alphanumeric_only()
            .build();
        assert!(!settings.include_symbols);
        assert!(settings.include_numbers);
        assert!(settings.include_uppercase);
        assert!(settings.include_lowercase);
    }
}
