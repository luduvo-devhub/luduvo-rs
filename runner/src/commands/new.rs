use std::ops::Deref;

use anyhow::Result;
use colored::*;

use crate::utils::command::run_command;

pub fn run(
    crate_name: String,
    dependencies: Option<Vec<String>>,
    verbose: bool,
    no_output: bool,
) -> Result<()> {
    let dependencies: Vec<String> = match dependencies {
        Some(c) => c,
        None => vec![],
    };

    println!("{}", "creating new package...".blue().bold());

    run_command(
        "crates".to_string(),
        "cargo".to_string(),
        &new_args(&crate_name, verbose),
        no_output,
    )?;

    if dependencies.len() > 0 {
        if dependencies.len() == 1 {
            println!("{}", "adding 1 dependency...".blue().bold());
        } else {
            println!(
                "{}",
                format!("adding {} dependencies...", dependencies.len())
                    .blue()
                    .bold()
            );
        }

        for dep_name in dependencies {
            run_command(
                format!("crates/{}", &crate_name),
                "cargo".to_string(),
                &dep_args(&dep_name, verbose),
                no_output,
            )?;
        }
    };

    println!("{}", "package created!!!".green().bold());

    Ok(())
}

fn new_args(crate_name: &String, verbose: bool) -> Vec<String> {
    let mut v = vec![
        "new".to_string(),
        crate_name.deref().to_string(),
        "--name".to_string(),
        format!("luduvo-{}", crate_name.deref().to_string()),
        "--lib".to_string(),
    ];

    if verbose {
        v.push("--verbose".to_string());
    }

    v
}

fn dep_args(dep_name: &String, verbose: bool) -> Vec<String> {
    let mut v = vec!["add".to_string(), dep_name.deref().to_string()];

    if verbose {
        v.push("--verbose".to_string());
    }

    v
}
