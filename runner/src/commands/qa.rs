use anyhow::Result;
use colored::*;

use crate::commands::test;
use crate::utils::command::run_command;
use crate::CRATES;

pub fn run(verbose: bool, no_output: bool) -> Result<()> {
    println!("{}", "running qa pipeline.........".blue().bold());
    println!("{}", "\nbuilding all crates...".cyan().bold());

    for crate_path in CRATES {
        println!("{} {}", "building:".green(), crate_path);

        run_command(
            crate_path.to_string(),
            "cargo".to_string(),
            &build_args(verbose),
            no_output,
        )?;
    }

    println!("{}", "\nrunning tests (via test command)...".cyan().bold());

    test::run(Some(vec!["all".to_string()]), verbose, no_output)?;

    println!("{}", "\nrunning clippy...".cyan().bold());

    for crate_path in CRATES {
        println!("{} {}", "clippy:".green(), crate_path);

        run_command(
            crate_path.to_string(),
            "cargo".to_string(),
            &clippy_args(verbose),
            no_output,
        )?;
    }

    println!("{}", "\nqa completed!!!".green().bold());

    Ok(())
}

fn build_args(verbose: bool) -> Vec<String> {
    let mut v = vec!["build".to_string()];

    if verbose {
        v.push("--verbose".to_string());
    }

    v
}

fn clippy_args(verbose: bool) -> Vec<String> {
    let mut v = vec![
        "clippy".to_string(),
        "--".to_string(),
        "-D".to_string(),
        "warnings".to_string(),
    ];

    if verbose {
        v.insert(1, "--verbose".to_string());
    }

    v
}
