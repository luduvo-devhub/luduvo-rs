use anyhow::Result;
use colored::*;

use crate::commands::publish;
use crate::utils::command::run_command;

pub fn run(title: &str, description: Option<&str>, verbose: bool, _publish: bool, no_output: bool) -> Result<()> {
    println!("{}", "pushing to git...".blue().bold());

    run_command(".", "git", &["add", "-A"], no_output)?;

    match description {
        Some(desc) => {
            run_command(".", "git", &["commit", "-m", title, "-m", desc], no_output)?;
        }

        None => {
            run_command(".", "git", &["commit", "-m", title], no_output)?;
        }
    }

    run_command(".", "git", &["push", "origin", "main"], no_output)?;

    if _publish {
        println!("{}", "publishing to crates.io via publish command...".blue().bold());

        publish::run(None, verbose, no_output)?;
    }

    println!("{}", "push completed!!!".green().bold());

    Ok(())
}
