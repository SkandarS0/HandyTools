use std::fmt;

/// Password strength rating based on calculated entropy and character diversity.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum PasswordStrength {
    VeryWeak,
    Weak,
    Moderate,
    Strong,
    VeryStrong,
}

impl fmt::Display for PasswordStrength {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::VeryWeak => "Very Weak",
            Self::Weak => "Weak",
            Self::Moderate => "Moderate",
            Self::Strong => "Strong",
            Self::VeryStrong => "Very Strong",
        };
        write!(f, "{s}")
    }
}

/// Analysis report of a password's composition, character classes, and entropy.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct PasswordStrengthReport {
    /// Estimated Shannon entropy in bits (adjusted for character uniqueness).
    pub entropy_bits: f64,
    pub strength: PasswordStrength,
    pub has_lowercase: bool,
    pub has_uppercase: bool,
    pub has_numbers: bool,
    pub has_symbols: bool,
    pub length: usize,
}

/// Estimates password strength and entropy from its actual character composition.
pub fn estimate_strength(password: &str) -> PasswordStrengthReport {
    let mut length = 0usize;
    let mut has_lowercase = false;
    let mut has_uppercase = false;
    let mut has_numbers = false;
    let mut has_symbols = false;

    // Fast, allocation-free ASCII uniqueness tracking with a bitset
    let mut ascii_seen = [0u64; 2];
    let mut non_ascii_unique = 0usize;
    let mut unique_count = 0usize;

    for c in password.chars() {
        length += 1;
        if c.is_ascii_lowercase() {
            has_lowercase = true;
        } else if c.is_ascii_uppercase() {
            has_uppercase = true;
        } else if c.is_ascii_digit() {
            has_numbers = true;
        } else if !c.is_whitespace() {
            has_symbols = true;
        }

        let code = c as u32;
        if code < 128 {
            let idx = (code / 64) as usize;
            let bit = 1u64 << (code % 64);
            if (ascii_seen[idx] & bit) == 0 {
                ascii_seen[idx] |= bit;
                unique_count += 1;
            }
        } else {
            non_ascii_unique += 1;
        }
    }
    unique_count += non_ascii_unique;

    let mut pool_size: u32 = 0;
    if has_lowercase {
        pool_size += 26;
    }
    if has_uppercase {
        pool_size += 26;
    }
    if has_numbers {
        pool_size += 10;
    }
    if has_symbols {
        pool_size += 32;
    }

    let entropy_bits = if pool_size == 0 || length == 0 {
        0.0
    } else {
        length as f64 * (pool_size as f64).log2()
    };

    let uniqueness_ratio = if length > 0 {
        unique_count as f64 / length as f64
    } else {
        0.0
    };
    let adjusted_entropy = entropy_bits * uniqueness_ratio.clamp(0.3, 1.0);

    let strength = match adjusted_entropy {
        e if e < 28.0 => PasswordStrength::VeryWeak,
        e if e < 36.0 => PasswordStrength::Weak,
        e if e < 60.0 => PasswordStrength::Moderate,
        e if e < 128.0 => PasswordStrength::Strong,
        _ => PasswordStrength::VeryStrong,
    };

    PasswordStrengthReport {
        entropy_bits: adjusted_entropy,
        strength,
        has_lowercase,
        has_uppercase,
        has_numbers,
        has_symbols,
        length,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn short_simple_password_is_weak() {
        let report = estimate_strength("abc123");
        assert!(report.strength <= PasswordStrength::Weak);
        assert!(report.has_lowercase);
        assert!(report.has_numbers);
        assert!(!report.has_uppercase);
        assert!(!report.has_symbols);
        assert_eq!(report.length, 6);
    }

    #[test]
    fn long_varied_password_is_strong() {
        let report = estimate_strength("K9#mZ2$pL7!qR4&nX1@vT8");
        assert!(report.strength >= PasswordStrength::Strong);
        assert!(report.has_lowercase);
        assert!(report.has_uppercase);
        assert!(report.has_numbers);
        assert!(report.has_symbols);
    }

    #[test]
    fn repeated_characters_reduce_strength() {
        let varied = estimate_strength("aB3$aB3$aB3$aB3$");
        let unique = estimate_strength("qW7#zR2@mK9!nL4$");
        assert!(unique.entropy_bits > varied.entropy_bits);
    }
}
