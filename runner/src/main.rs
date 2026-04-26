#![allow(unused)]

use anyhow::Result;
use clap::Parser;
use dotenvy::dotenv;

mod cli;
mod commands;
mod utils;

use cli::{Cli, Commands};

pub const CRATES: &[&str] = &["crates/api", "crates/dom", "crates/verify"];

fn main() -> Result<()> {
    dotenv().ok();

    let cli = Cli::parse();

    match cli.command {
        Commands::Qa => commands::qa::run(cli.verbose, cli.no_output),
        Commands::Docgen { no_serve } => commands::docgen::run(no_serve, cli.verbose, cli.no_output),

        Commands::Publish { crates } => commands::publish::run(crates, cli.verbose, cli.no_output),
        Commands::Test { crates } => commands::test::run(crates, cli.verbose, cli.no_output),

        Commands::Push {
            branch,
            title,
            description,
            publish,
        } => commands::push::run(branch, title, description, cli.verbose, publish, cli.no_output),

        Commands::New {
            crate_name,
            dependencies,
        } => commands::new::run(crate_name, dependencies, cli.verbose, cli.no_output),
    }
}
