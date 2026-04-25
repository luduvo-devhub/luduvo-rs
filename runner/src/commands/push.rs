use anyhow::Result;
use colored::*;

use crate::utils::command::run_command;

pub fn run(title: &str, description: &[String], no_output: bool) -> Result<()> {
    let description = description.join(" ");

    println!("{}", "pushing to git...".blue().bold());

    run_command(".", "git", &["add", "-A"], no_output)?;
    run_command(
        ".",
        "git",
        &["commit", "-m", title, "-m", &description],
        no_output,
    )?;
    run_command(".", "git", &["push", "origin", "main"], no_output)?;

    println!("{}", "push completed ✔️".green().bold());

    Ok(())
}
