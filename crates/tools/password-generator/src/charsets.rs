pub(crate) const LOWERCASE: &str = "abcdefghijklmnopqrstuvwxyz";
pub(crate) const UPPERCASE: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
pub(crate) const NUMBERS: &str = "0123456789";
pub(crate) const SYMBOLS: &str = "!@#$%^&*()-_=+[]{}<>?/.,~";

/// Visually similar characters that can easily be confused: `l`, `1`, `I`, `O`, `0`, `o`.
pub(crate) const SIMILAR: &[char] = &['l', '1', 'I', 'O', '0', 'o'];

/// Punctuation and bracket characters that may cause issues or confusion in certain fonts/shells.
pub(crate) const AMBIGUOUS: &[char] = &[
    '{', '}', '[', ']', '(', ')', '/', '\\', '\'', '"', '`', '~', ',', ';', ':', '.', '<', '>',
];
