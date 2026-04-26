use anyhow::Result;
use colored::*;
use std::env;

use crate::utils::command::run_command_with_env;
use crate::CRATES;

pub fn run(crates: Option<Vec<String>>, verbose: bool, no_output: bool) -> Result<()> {
    let mut crates: Vec<String> = match crates {
        Some(c) => c,
        None => CRATES.iter().map(|c| c.replace("crates/", "")).collect(),
    };

    if crates.len() == 1 && crates[0] == "all" {
        crates = CRATES.iter().map(|c| c.replace("crates/", "")).collect();
    }

    println!("{}", "publishing crates....".blue().bold());

    let token = env::var("CARGO_REGISTRY_TOKEN")?;

    for crate_name in crates {
        let crate_path = format!("crates/{}", crate_name);

        println!("{} {}", "publishing crate:".yellow(), crate_name);

        run_command_with_env(
            crate_path,
            "cargo".to_string(),
            &publish_args(verbose),
            &[("CARGO_REGISTRY_TOKEN".to_string(), token.to_string())],
            no_output,
        )?;
    }

    println!("{}", "publish completed!!!".green().bold());

    Ok(())
}

fn publish_args(verbose: bool) -> Vec<String> {
    let mut v = vec!["publish".to_string(), "--allow-dirty".to_string()];

    if verbose {
        v.push("--verbose".to_string());
    }

    v
}
