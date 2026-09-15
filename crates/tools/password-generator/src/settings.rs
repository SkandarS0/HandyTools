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
