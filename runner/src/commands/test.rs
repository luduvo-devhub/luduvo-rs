use anyhow::Result;
use colored::*;

use crate::utils::command::run_command;

const CRATES: &[&str] = &["crates/api", "crates/dom"];

pub fn run(mut crates: Vec<String>, verbose: bool, no_output: bool) -> Result<()> {
    if crates.is_empty() {
        crates = CRATES.iter().map(|c| c.replace("crates/", "")).collect();
    }
    
    if crates.len() == 1 && crates[0] == "all" {
        crates = CRATES.iter().map(|c| c.replace("crates/", "")).collect();
    }

    for crate_name in crates {
        let path = format!("crates/{}", crate_name);

        if !CRATES.contains(&path.as_str()) {
            println!("unknown crate: {}", crate_name);
            
            std::process::exit(1);
        }

        println!(
            "{} {}",
            "running tests for crate:".blue().bold(),
            crate_name
        );

        run_command(&path, "cargo", &test_args(verbose), no_output)?;
    }

    println!("{}", "all tests completed ✔️".green().bold());

    Ok(())
}

fn test_args(verbose: bool) -> Vec<&'static str> {
    let mut v = vec!["test", "--tests"];
    if verbose {
        v.push("--verbose");
    }
    v
}
