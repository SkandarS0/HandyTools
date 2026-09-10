mod commands;

use clap::{Parser, Subcommand};
use commands::password_generator::{PasswordSubcommand, execute_pwd};

#[derive(Parser)]
#[command(
    name = "hnd",
    version,
    about = "Handy CLI developer and system utilities"
)]
#[command(propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Password generation and management utilities
    #[command(alias = "pwd")]
    Password(PasswordSubcommand),
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Password(args) => {
            execute_pwd(args)?;
        }
    }

    Ok(())
}
