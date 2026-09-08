mod commands;

use clap::{Parser, Subcommand};
use commands::password_generator::{PwdArgs, execute_pwd};

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
    #[command(alias = "password")]
    Pwd(PwdArgs),
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Pwd(args) => {
            execute_pwd(args)?;
        }
    }

    Ok(())
}
