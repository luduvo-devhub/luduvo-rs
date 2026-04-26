use anyhow::Result;
use colored::*;

use crate::utils::command::run_command;
use crate::utils::docgen::generate_docs;

pub fn run(no_serve: bool, _verbose: bool, no_output: bool) -> Result<()> {
    println!("{}", "generating docs...".blue().bold());

    generate_docs("crates", "docs")?;

    println!("{}", "docs generated in ./docs".green().bold());

    if !no_serve {
        println!("{}", "serving docs...".blue().bold());

        run_command(
            ".".to_string(),
            "uvx".to_string(),
            &["zensical".to_string(), "serve".to_string()],
            no_output,
        )?;
    }

    Ok(())
}
