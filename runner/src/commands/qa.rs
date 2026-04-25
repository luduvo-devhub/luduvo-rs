use anyhow::Result;
use colored::*;

use crate::commands::test;
use crate::utils::command::run_command;

const CRATES: &[&str] = &["crates/api", "crates/dom"];

pub fn run(verbose: bool, no_output: bool) -> Result<()> {
    println!("{}", "running qa pipeline.........".blue().bold());
    println!("{}", "\nbuilding all crates...".cyan().bold());

    for crate_path in CRATES {
        println!("{} {}", "building:".green(), crate_path);

        run_command(crate_path, "cargo", &build_args(verbose), no_output)?;
    }

    println!("{}", "\nrunning tests (via test command)...".cyan().bold());

    test::run(vec!["all".to_string()], verbose, no_output)?;

    println!("{}", "\nrunning clippy...".cyan().bold());

    for crate_path in CRATES {
        println!("{} {}", "clippy:".green(), crate_path);
        run_command(crate_path, "cargo", &clippy_args(verbose), no_output)?;
    }

    println!("{}", "\nqa completed!!!".green().bold());

    Ok(())
}

fn build_args(verbose: bool) -> Vec<&'static str> {
    let mut v = vec!["build"];

    if verbose {
        v.push("--verbose");
    }

    v
}

fn clippy_args(verbose: bool) -> Vec<&'static str> {
    let mut v = vec!["clippy", "--", "-D", "warnings"];

    if verbose {
        v.insert(1, "--verbose");
    }

    v
}
