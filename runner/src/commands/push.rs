use anyhow::Result;
use colored::*;

use crate::commands::publish;
use crate::utils::command::run_command;

pub fn run(
    title: String,
    description: Option<String>,
    verbose: bool,
    _publish: bool,
    no_output: bool,
) -> Result<()> {
    println!("{}", "pushing to git...".blue().bold());

    run_command(
        ".".to_string(),
        "git".to_string(),
        &["add".to_string(), "-A".to_string()],
        no_output,
    )?;

    match description {
        Some(desc) => {
            run_command(
                ".".to_string(),
                "git".to_string(),
                &[
                    "commit".to_string(),
                    "-m".to_string(),
                    title.to_string(),
                    "-m".to_string(),
                    desc.to_string(),
                ],
                no_output,
            )?;
        }

        None => {
            run_command(
                ".".to_string(),
                "git".to_string(),
                &["commit".to_string(), "-m".to_string(), title.to_string()],
                no_output,
            )?;
        }
    }

    run_command(
        ".".to_string(),
        "git".to_string(),
        &["push".to_string(), "origin".to_string(), "main".to_string()],
        no_output,
    )?;

    if _publish {
        println!(
            "{}",
            "publishing to crates.io via publish command..."
                .blue()
                .bold()
        );

        publish::run(None, verbose, no_output)?;
    }

    println!("{}", "push completed!!!".green().bold());

    Ok(())
}
