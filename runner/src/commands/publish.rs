use anyhow::Result;
use colored::*;
use std::env;

use crate::utils::command::run_command_with_env;

const CRATES: &[&str] = &["crates/api", "crates/dom"];

pub fn run(verbose: bool, no_output: bool) -> Result<()> {
    println!("{}", "publishing crates....".blue().bold());

    let token = env::var("CARGO_REGISTRY_TOKEN")?;

    for crate_path in CRATES {
        println!("{} {}", "publishing crate:".yellow(), crate_path);

        run_command_with_env(
            crate_path,
            "cargo",
            &publish_args(verbose),
            &[("CARGO_REGISTRY_TOKEN", &token)],
            no_output,
        )?;
    }

    println!("{}", "publish completed!!!".green().bold());
    
    Ok(())
}

fn publish_args(verbose: bool) -> Vec<&'static str> {
    let mut v = vec!["publish", "--allow-dirty"];
    
    if verbose {
        v.push("--verbose");
    }
    v
}
