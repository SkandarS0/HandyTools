use clap::{Args, Subcommand};
use hnd_password_generator::PasswordGeneratorSettings;

#[derive(Debug, Args, Clone)]
#[command(args_conflicts_with_subcommands = true)]
pub struct PasswordSubcommand {
    #[command(subcommand)]
    pub command: PasswordSubcommands,
}

#[derive(Debug, Subcommand, Clone)]
pub enum PasswordSubcommands {
    /// Generate secure random passwords
    #[command(alias = "gen")]
    Generate(PasswordGenerationArgs),
}

#[derive(Debug, Args, Clone)]
pub struct PasswordGenerationArgs {
    /// Length of the password
    #[arg(short = 'l', long, default_value_t = 16)]
    pub length: u8,

    /// Exclude lowercase letters (a-z)
    #[arg(long)]
    pub no_lowercase: bool,

    /// Exclude uppercase letters (A-Z)
    #[arg(long)]
    pub no_uppercase: bool,

    /// Exclude numbers (0-9)
    #[arg(long, visible_alias = "no-digits")]
    pub no_numbers: bool,

    /// Exclude special symbols (!@#$%^&*...)
    #[arg(long)]
    pub no_symbols: bool,

    /// Include visually similar characters (l, 1, I, O, 0, o) [default: excluded for readability]
    #[arg(long, visible_aliases = ["allow-similar", "similar", "with-similar"])]
    pub include_similar: bool,

    /// Include visually ambiguous characters ({}[]()/\'",.:;<>~) [default: excluded for readability]
    #[arg(long, visible_aliases = ["allow-ambiguous", "ambiguous", "with-ambiguous"])]
    pub include_ambiguous: bool,

    /// Include all characters (allow both similar and ambiguous characters)
    #[arg(long, visible_aliases = ["allow-all", "all-chars"])]
    pub include_all: bool,

    /// Generate alphanumeric characters only (letters and digits, no symbols)
    #[arg(short = 'a', long)]
    pub alphanumeric: bool,

    /// Number of passwords to generate
    #[arg(short = 'c', short_alias = 'n', long, default_value_t = 1)]
    pub count: usize,

    /// Display estimated password strength analysis
    #[arg(short = 's', long, visible_alias = "strength")]
    pub show_strength: bool,
}

impl From<&PasswordGenerationArgs> for PasswordGeneratorSettings {
    fn from(args: &PasswordGenerationArgs) -> Self {
        let mut builder = PasswordGeneratorSettings::builder();
        builder = builder.length(args.length);

        if args.no_lowercase {
            builder = builder.include_lowercase(false);
        }
        if args.no_uppercase {
            builder = builder.include_uppercase(false);
        }
        if args.no_numbers {
            builder = builder.include_numbers(false);
        }
        if args.no_symbols || args.alphanumeric {
            builder = builder.include_symbols(false);
        }
        if args.include_similar || args.include_all {
            builder = builder.include_similar_characters(true);
        }
        if args.include_ambiguous || args.include_all {
            builder = builder.include_ambiguous_characters(true);
        }

        builder.build()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use clap::Parser;

    #[derive(Parser, Debug)]
    struct TestCli {
        #[command(subcommand)]
        command: TestSubcommand,
    }

    #[derive(Subcommand, Debug)]
    enum TestSubcommand {
        #[command(alias = "password")]
        Pwd(PasswordSubcommand),
    }

    #[test]
    fn parses_default_generate_command() {
        let cli = TestCli::parse_from(["test", "pwd", "generate"]);
        let TestSubcommand::Pwd(pwd_args) = cli.command;
        let PasswordSubcommands::Generate(gen_args) = pwd_args.command;

        assert_eq!(gen_args.length, 16);
        assert_eq!(gen_args.count, 1);
        assert!(!gen_args.no_lowercase);
        assert!(!gen_args.no_uppercase);
        assert!(!gen_args.no_numbers);
        assert!(!gen_args.no_symbols);
        assert!(!gen_args.include_similar);
        assert!(!gen_args.include_ambiguous);
        assert!(!gen_args.include_all);
        assert!(!gen_args.alphanumeric);
        assert!(!gen_args.show_strength);

        let settings = PasswordGeneratorSettings::from(&gen_args);
        assert_eq!(settings.length, 16);
        assert!(settings.include_lowercase);
        assert!(settings.include_uppercase);
        assert!(settings.include_numbers);
        assert!(settings.include_symbols);
        // By default, similar and ambiguous characters are excluded for easy reading
        assert!(!settings.include_similar_characters);
        assert!(!settings.include_ambiguous_characters);
    }

    #[test]
    fn parses_custom_options_and_aliases() {
        let cli = TestCli::parse_from([
            "test",
            "password",
            "gen",
            "-l",
            "32",
            "-n",
            "4",
            "--no-digits",
            "--no-symbols",
            "--allow-similar",
            "--strength",
        ]);
        let TestSubcommand::Pwd(pwd_args) = cli.command;
        let PasswordSubcommands::Generate(gen_args) = pwd_args.command;

        assert_eq!(gen_args.length, 32);
        assert_eq!(gen_args.count, 4);
        assert!(gen_args.no_numbers);
        assert!(gen_args.no_symbols);
        assert!(gen_args.include_similar);
        assert!(gen_args.show_strength);

        let settings = PasswordGeneratorSettings::from(&gen_args);
        assert_eq!(settings.length, 32);
        assert!(!settings.include_numbers);
        assert!(!settings.include_symbols);
        assert!(settings.include_similar_characters);
        assert!(!settings.include_ambiguous_characters);
    }

    #[test]
    fn parses_include_all_and_alphanumeric() {
        let cli = TestCli::parse_from(["test", "pwd", "generate", "--alphanumeric", "--allow-all"]);
        let TestSubcommand::Pwd(pwd_args) = cli.command;
        let PasswordSubcommands::Generate(gen_args) = pwd_args.command;

        assert!(gen_args.alphanumeric);
        assert!(gen_args.include_all);

        let settings = PasswordGeneratorSettings::from(&gen_args);
        assert!(!settings.include_symbols);
        assert!(settings.include_similar_characters);
        assert!(settings.include_ambiguous_characters);
    }
}
