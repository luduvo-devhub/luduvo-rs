use anyhow::Result;
use std::process::{Command, Stdio};

pub fn run_command(dir: String, cmd: String, args: &[String], no_output: bool) -> Result<()> {
    println!("> {} {}", cmd, args.join(" "));

    let mut command = Command::new(&cmd);
    command.args(args).current_dir(dir);

    configure_stdio(&mut command, no_output);

    let status = command.status()?;

    if !status.success() {
        println!("command failed: {} {}", cmd, args.join(" "));
        std::process::exit(1);
    }

    Ok(())
}

pub fn run_command_with_env(
    dir: String,
    cmd: String,
    args: &[String],
    envs: &[(String, String)],
    no_output: bool,
) -> Result<()> {
    println!("> {} {}", cmd, args.join(" "));

    let mut command = Command::new(&cmd);

    command.args(args).current_dir(dir);

    for (k, v) in envs {
        command.env(k, v);
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
