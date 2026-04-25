use anyhow::{bail, Result};
use colored::*;

use crate::utils::command::run_command;

pub fn run(parts: &[String], no_output: bool) -> Result<()> {
    let split = parts.iter().position(|p| p == "--");

    let (title, description) = match split {
        Some(idx) => {
            let title = parts[..idx].join(" ");
            let description = parts[idx + 1..].join(" ");

            (title, description)
        }

        None => (parts.join(" "), String::new()),
    };

    if title.is_empty() {
        bail!("commit message cannot be empty");
    }

    println!("{}", "pushing to git...".blue().bold());

    run_command(".", "git", &["add", "-A"], no_output)?;

    if description.is_empty() {
        run_command(".", "git", &["commit", "-m", &title], no_output)?;
    } else {
        run_command(
            ".",
            "git",
            &["commit", "-m", &title, "-m", &description],
            no_output,
        )?;
    }

    run_command(".", "git", &["push", "origin", "main"], no_output)?;

    println!("{}", "push completed ✔️".green().bold());

    Ok(())
}
