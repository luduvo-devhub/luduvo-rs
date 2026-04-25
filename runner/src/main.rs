use anyhow::Result;
use clap::Parser;
use dotenvy::dotenv;

mod cli;
mod commands;
mod utils;

use cli::{Cli, Commands};

fn main() -> Result<()> {
    dotenv().ok();

    let cli = Cli::parse();

    match cli.command {
        Commands::Qa => commands::qa::run(cli.verbose, cli.no_output),
        Commands::Publish { crates } => commands::publish::run(crates, cli.verbose, cli.no_output),

        Commands::Push { title, description, publish } => {
            commands::push::run(&title, description.as_deref(), cli.verbose, publish, cli.no_output)
        }

        Commands::Test { crates } => {
            commands::test::run(crates, cli.verbose, cli.no_output)
        },
    }
}
