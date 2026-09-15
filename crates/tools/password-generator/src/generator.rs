use rand::prelude::*;

use crate::charsets::{AMBIGUOUS, LOWERCASE, NUMBERS, SIMILAR, SYMBOLS, UPPERCASE};
use crate::errors::PasswordGeneratorError;
use crate::settings::PasswordGeneratorSettings;

impl PasswordGeneratorSettings {
    /// Generates a password using the default system entropy source.
    pub fn generate(&self) -> Result<String, PasswordGeneratorError> {
        let mut rng = rand::rng();
        self.generate_with_rng(&mut rng)
    }

    /// Generates a password using a provided random number generator.
    pub fn generate_with_rng<R: Rng + ?Sized>(
        &self,
        rng: &mut R,
    ) -> Result<String, PasswordGeneratorError> {
        if self.length == 0 {
            return Err(PasswordGeneratorError::LengthTooShort);
        }

        let mut pool: Vec<char> = Vec::with_capacity(96);

        if self.include_lowercase {
            pool.extend(LOWERCASE.chars());
        }
        if self.include_uppercase {
            pool.extend(UPPERCASE.chars());
        }
        if self.include_numbers {
            pool.extend(NUMBERS.chars());
        }
        if self.include_symbols {
            pool.extend(SYMBOLS.chars());
        }

        if !self.include_similar_characters {
            pool.retain(|c| !SIMILAR.contains(c));
        }
        if !self.include_ambiguous_characters {
            pool.retain(|c| !AMBIGUOUS.contains(c));
        }

        if pool.is_empty() {
            return Err(PasswordGeneratorError::NoCharacterSetsSelected);
        }

        let mut required_sets: Vec<&str> = Vec::with_capacity(4);
        if self.include_lowercase {
            required_sets.push(LOWERCASE);
        }
        if self.include_uppercase {
            required_sets.push(UPPERCASE);
        }
        if self.include_numbers {
            required_sets.push(NUMBERS);
        }
        if self.include_symbols {
            required_sets.push(SYMBOLS);
        }

        if self.length < (required_sets.len() as u8) {
            return Err(PasswordGeneratorError::LengthInsufficientForRequiredSets);
        }

        // First pass: select random characters from the combined pool
        let mut password: Vec<char> = (0..self.length)
            .map(|_| *pool.choose(rng).unwrap())
            .collect();

        // Second pass: guarantee at least one character from each selected character set
        for (i, set) in required_sets.iter().enumerate() {
            let filtered: Vec<char> = set
                .chars()
                .filter(|c| {
                    (self.include_similar_characters || !SIMILAR.contains(c))
                        && (self.include_ambiguous_characters || !AMBIGUOUS.contains(c))
                })
                .collect();
            if let Some(&ch) = filtered.choose(rng) {
                password[i] = ch;
            }
        }
        password.shuffle(rng);

        Ok(password.into_iter().collect())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::SeedableRng;
    use rand::rngs::StdRng;

    #[test]
    fn generates_correct_length() {
        let settings = PasswordGeneratorSettings {
            length: 24,
            ..Default::default()
        };
        let password = settings.generate().unwrap();
        assert_eq!(password.len(), 24);
    }

    #[test]
    fn deterministic_generation_with_custom_rng() {
        let settings = PasswordGeneratorSettings {
            length: 12,
            ..Default::default()
        };
        let mut rng1 = StdRng::seed_from_u64(42);
        let mut rng2 = StdRng::seed_from_u64(42);

        let pw1 = settings.generate_with_rng(&mut rng1).unwrap();
        let pw2 = settings.generate_with_rng(&mut rng2).unwrap();

        assert_eq!(pw1, pw2);
        assert_eq!(pw1.len(), 12);
    }

    #[test]
    fn contains_all_selected_character_types() {
        let settings = PasswordGeneratorSettings {
            length: 100,
            include_numbers: true,
            include_symbols: true,
            include_uppercase: true,
            include_lowercase: true,
            include_similar_characters: true,
            include_ambiguous_characters: true,
        };
        let password = settings.generate().unwrap();

        assert!(password.chars().any(|c| LOWERCASE.contains(c)));
        assert!(password.chars().any(|c| UPPERCASE.contains(c)));
        assert!(password.chars().any(|c| NUMBERS.contains(c)));
        assert!(password.chars().any(|c| SYMBOLS.contains(c)));
    }

    #[test]
    fn errors_when_no_character_sets_selected() {
        let settings = PasswordGeneratorSettings {
            include_numbers: false,
            include_symbols: false,
            include_uppercase: false,
            include_lowercase: false,
            ..Default::default()
        };
        assert_eq!(
            settings.generate(),
            Err(PasswordGeneratorError::NoCharacterSetsSelected)
        );
    }

    #[test]
    fn errors_when_length_is_zero() {
        let settings = PasswordGeneratorSettings {
            length: 0,
            ..Default::default()
        };
        assert_eq!(
            settings.generate(),
            Err(PasswordGeneratorError::LengthTooShort)
        );
    }

    #[test]
    fn errors_when_length_insufficient_for_selected_sets() {
        let settings = PasswordGeneratorSettings {
            length: 2,
            include_numbers: true,
            include_symbols: true,
            include_uppercase: true,
            include_lowercase: true,
            ..Default::default()
        };
        assert_eq!(
            settings.generate(),
            Err(PasswordGeneratorError::LengthInsufficientForRequiredSets)
        );
    }

    #[test]
    fn excludes_similar_characters() {
        let settings = PasswordGeneratorSettings {
            length: 200,
            include_similar_characters: false,
            ..Default::default()
        };
        let password = settings.generate().unwrap();
        assert!(password.chars().all(|c| !SIMILAR.contains(&c)));
    }

    #[test]
    fn excludes_ambiguous_characters() {
        let settings = PasswordGeneratorSettings {
            length: 200,
            include_ambiguous_characters: false,
            ..Default::default()
        };
        let password = settings.generate().unwrap();
        assert!(password.chars().all(|c| !AMBIGUOUS.contains(&c)));
    }

    #[test]
    fn default_settings_exclude_similar_and_ambiguous_characters() {
        let password = PasswordGeneratorSettings::default().generate().unwrap();
        assert!(password.chars().all(|c| !SIMILAR.contains(&c)));
        assert!(password.chars().all(|c| !AMBIGUOUS.contains(&c)));
    }
}
