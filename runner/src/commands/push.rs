use anyhow::Result;
use colored::*;

use crate::utils::command::run_command;

pub fn run(title: &str, description: Option<&str>, no_output: bool) -> Result<()> {
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

    println!("{}", "push completed ✔️".green().bold());

    Ok(())
}
