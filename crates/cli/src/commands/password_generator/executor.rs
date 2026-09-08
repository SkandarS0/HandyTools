use super::args::{GenerateArgs, PwdArgs, PwdCommands};
use super::errors::PasswordCliError;
use hnd_password_generator::{PasswordGeneratorSettings, estimate_strength};
use std::io::{self, Write};

pub fn execute_pwd(args: &PwdArgs) -> Result<(), PasswordCliError> {
    let mut stdout = io::stdout().lock();
    match &args.command {
        PwdCommands::Generate(gen_args) => execute_generate(gen_args, &mut stdout),
    }
}

pub fn execute_generate<W: Write>(
    args: &GenerateArgs,
    writer: &mut W,
) -> Result<(), PasswordCliError> {
    if args.count == 0 {
        return Err(PasswordCliError::ZeroCount);
    }

    let settings = PasswordGeneratorSettings::from(args);

    let mut passwords = Vec::with_capacity(args.count);
    for _ in 0..args.count {
        let password = settings.generate()?;
        let report = estimate_strength(&password);
        passwords.push((password, report));
    }

    passwords.sort_by(|a, b| b.1.entropy_bits.total_cmp(&a.1.entropy_bits));

    for (password, report) in passwords {
        if args.show_strength {
            writeln!(
                writer,
                "{password}\t[Strength: {}, Entropy: {:.1} bits]",
                report.strength, report.entropy_bits
            )?;
        } else {
            writeln!(writer, "{password}")?;
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generates_single_default_password() {
        let args = GenerateArgs {
            length: 16,
            no_lowercase: false,
            no_uppercase: false,
            no_numbers: false,
            no_symbols: false,
            include_similar: false,
            include_ambiguous: false,
            include_all: false,
            alphanumeric: false,
            count: 1,
            show_strength: false,
        };

        let mut output = Vec::new();
        execute_generate(&args, &mut output).unwrap();
        let text = String::from_utf8(output).unwrap();
        let lines: Vec<&str> = text.lines().collect();

        assert_eq!(lines.len(), 1);
        assert_eq!(lines[0].len(), 16);
    }

    #[test]
    fn generates_multiple_passwords() {
        let args = GenerateArgs {
            length: 20,
            no_lowercase: false,
            no_uppercase: false,
            no_numbers: false,
            no_symbols: false,
            include_similar: false,
            include_ambiguous: false,
            include_all: false,
            alphanumeric: false,
            count: 5,
            show_strength: false,
        };

        let mut output = Vec::new();
        execute_generate(&args, &mut output).unwrap();
        let text = String::from_utf8(output).unwrap();
        let lines: Vec<&str> = text.lines().collect();

        assert_eq!(lines.len(), 5);
        for line in lines {
            assert_eq!(line.len(), 20);
        }
    }

    #[test]
    fn generates_with_strength_info() {
        let args = GenerateArgs {
            length: 24,
            no_lowercase: false,
            no_uppercase: false,
            no_numbers: false,
            no_symbols: false,
            include_similar: false,
            include_ambiguous: false,
            include_all: false,
            alphanumeric: false,
            count: 2,
            show_strength: true,
        };

        let mut output = Vec::new();
        execute_generate(&args, &mut output).unwrap();
        let text = String::from_utf8(output).unwrap();
        let lines: Vec<&str> = text.lines().collect();

        assert_eq!(lines.len(), 2);
        for line in lines {
            assert!(line.contains("[Strength:"));
            assert!(line.contains("Entropy:"));
        }
    }

    #[test]
    fn outputs_passwords_sorted_by_descending_entropy() {
        let args = GenerateArgs {
            length: 16,
            no_lowercase: false,
            no_uppercase: false,
            no_numbers: false,
            no_symbols: false,
            include_similar: true,
            include_ambiguous: true,
            include_all: true,
            alphanumeric: false,
            count: 10,
            show_strength: true,
        };

        let mut output = Vec::new();
        execute_generate(&args, &mut output).unwrap();
        let text = String::from_utf8(output).unwrap();
        let lines: Vec<&str> = text.lines().collect();

        assert_eq!(lines.len(), 10);

        let mut prev_entropy = f64::MAX;
        for line in lines {
            let entropy_part = line.split("Entropy: ").nth(1).unwrap();
            let entropy_val: f64 = entropy_part
                .split_whitespace()
                .next()
                .unwrap()
                .parse()
                .unwrap();
            assert!(
                entropy_val <= prev_entropy,
                "Expected {entropy_val} <= {prev_entropy}"
            );
            prev_entropy = entropy_val;
        }
    }

    #[test]
    fn fails_on_zero_count() {
        let args = GenerateArgs {
            length: 16,
            no_lowercase: false,
            no_uppercase: false,
            no_numbers: false,
            no_symbols: false,
            include_similar: false,
            include_ambiguous: false,
            include_all: false,
            alphanumeric: false,
            count: 0,
            show_strength: false,
        };

        let mut output = Vec::new();
        let err = execute_generate(&args, &mut output).unwrap_err();
        assert!(matches!(err, PasswordCliError::ZeroCount));
    }

    #[test]
    fn fails_on_invalid_generator_settings() {
        let args = GenerateArgs {
            length: 16,
            no_lowercase: true,
            no_uppercase: true,
            no_numbers: true,
            no_symbols: true,
            include_similar: false,
            include_ambiguous: false,
            include_all: false,
            alphanumeric: false,
            count: 1,
            show_strength: false,
        };

        let mut output = Vec::new();
        let err = execute_generate(&args, &mut output).unwrap_err();
        assert!(matches!(err, PasswordCliError::Generator(_)));
    }
}
