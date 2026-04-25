use anyhow::Result;
use colored::*;
use dotenvy::dotenv;
use std::process::{Command, Stdio};

const CRATES: &[&str] = &["crates/api", "crates/dom"];

fn main() -> Result<()> {
    dotenv().ok();

    let mut args: Vec<String> = std::env::args().collect();

    if args.len() < 2 {
        eprintln!("usage: runner <qa|publish|push|test> [args...] [--verbose|-v] [--no-output|-q]");
        std::process::exit(1);
    }

    let verbose = args.iter().any(|a| a == "--verbose" || a == "-v");
    let no_output = args.iter().any(|a| a == "--no-output" || a == "-q");

    args.retain(|a| a != "--verbose" && a != "-v" && a != "--no-output" && a != "-q");

    match args[1].as_str() {
        "qa" => run_qa(verbose, no_output),
        "publish" => run_publish(verbose, no_output),
        "push" => run_push(&args, no_output),
        "test" => run_test(&args, verbose, no_output),

        cmd => {
            eprintln!("unknown command: {}", cmd);

            std::process::exit(1);
        }
    }
}

/* ---------------- NEW: TEST COMMAND ---------------- */

fn run_test(args: &[String], verbose: bool, no_output: bool) -> Result<()> {
    if args.len() < 3 {
        println!("usage: runner test <crate>");

        std::process::exit(1);
    }

    let crate_name = &args[2];
    let crate_path = format!("crates/{}", crate_name);

    if !CRATES.contains(&crate_path.as_str()) {
        println!(
            "unknown crate: {} (expected one of {:?})",
            crate_name, CRATES
        );

        std::process::exit(1);
    }

    println!(
        "{} {}",
        "running tests for crate:".blue().bold(),
        crate_name
    );

    run_command(&crate_path, "cargo", &test_args(verbose), no_output)?;

    println!("{}", "tests completed ✔️".green().bold());

    Ok(())
}

/* ---------------- EXISTING COMMANDS ---------------- */

fn run_push(args: &[String], no_output: bool) -> Result<()> {
    if args.len() < 4 {
        println!("usage: runner push <title> <description>");

        std::process::exit(1);
    }

    let title = &args[2];
    let description = &args[3];

    println!("{}", "pushing to git...".blue().bold());

    run_command(".", "git", &["add", "-A"], no_output)?;

    run_command(
        ".",
        "git",
        &["commit", "-m", title, "-m", description],
        no_output,
    )?;

    run_command(".", "git", &["push", "origin", "main"], no_output)?;

    println!("{}", "push completed ✔️".green().bold());

    Ok(())
}

fn run_qa(verbose: bool, no_output: bool) -> Result<()> {
    println!("{}", "running qa pipeline.........".blue().bold());

    for crate_path in CRATES {
        println!("\n{} {}", "processing crate:".green(), crate_path);

        run_command(crate_path, "cargo", &build_args(verbose), no_output)?;
        run_command(crate_path, "cargo", &test_args(verbose), no_output)?;
        run_command(crate_path, "cargo", &clippy_args(verbose), no_output)?;
    }

    println!("\n{}", "qa completed successfully!!!!!".green().bold());
    Ok(())
}

fn run_publish(verbose: bool, no_output: bool) -> Result<()> {
    println!("{}", "publishing crates....".blue().bold());

    let token = std::env::var("CARGO_REGISTRY_TOKEN")
        .map_err(|_| anyhow::anyhow!("missing CARGO_REGISTRY_TOKEN env variable!!!!!"))?;

    for crate_path in CRATES {
        println!("\n{} {}", "publishing crate:".yellow(), crate_path);

        run_command_with_env(
            crate_path,
            "cargo",
            &publish_args(verbose),
            &[("CARGO_REGISTRY_TOKEN", &token)],
            no_output,
        )?;
    }

    println!("\n{}", "publish completed!!!!".green().bold());
    Ok(())
}

/* ---------------- ARG BUILDERS ---------------- */

fn build_args(verbose: bool) -> Vec<&'static str> {
    let mut args = vec!["build"];
    if verbose {
        args.push("--verbose");
    }
    args
}

fn test_args(verbose: bool) -> Vec<&'static str> {
    let mut args = vec!["test", "--tests"];
    if verbose {
        args.push("--verbose");
    }
    args
}

fn clippy_args(verbose: bool) -> Vec<&'static str> {
    let mut args = vec!["clippy", "--", "-D", "warnings"];
    if verbose {
        args.insert(1, "--verbose");
    }
    args
}

fn publish_args(verbose: bool) -> Vec<&'static str> {
    let mut args = vec!["publish", "--allow-dirty"];
    if verbose {
        args.push("--verbose");
    }
    args
}

/* ---------------- COMMAND RUNNERS ---------------- */

fn run_command(dir: &str, cmd: &str, args: &[&str], no_output: bool) -> Result<()> {
    println!("> {} {}", cmd, args.join(" "));

    let mut command = Command::new(cmd);
    command.args(args).current_dir(dir);

    configure_stdio(&mut command, no_output);

    let status = command.status()?;

    if !status.success() {
        println!("command failed: {} {}", cmd, args.join(" "));

        std::process::exit(1);
    }

    Ok(())
}

fn run_command_with_env(
    dir: &str,
    cmd: &str,
    args: &[&str],
    envs: &[(&str, &str)],
    no_output: bool,
) -> Result<()> {
    println!("> {} {}", cmd, args.join(" "));

    let mut command = Command::new(cmd);
    command.args(args).current_dir(dir);

    for (key, value) in envs {
        command.env(key, value);
    }

    configure_stdio(&mut command, no_output);

    let status = command.status()?;

    if !status.success() {
        println!("command failed: {} {}", cmd, args.join(" "));

        std::process::exit(1);
    }

    Ok(())
}

fn configure_stdio(command: &mut Command, no_output: bool) {
    if no_output {
        command.stdout(Stdio::null()).stderr(Stdio::null());
    } else {
        command.stdout(Stdio::inherit()).stderr(Stdio::inherit());
    }
}
