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
        Commands::Publish => commands::publish::run(cli.verbose, cli.no_output),

        Commands::Push { title, description } => {
            commands::push::run(&title, &description, cli.no_output)
        }

        Commands::Test { crates } => commands::test::run(crates, cli.verbose, cli.no_output),
    }
}
