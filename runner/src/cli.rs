use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(
    name = "runner",
    about = "a fancy command runner, designed specifically for luduvo-rs"
)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,

    #[arg(short, long, help = "whether or not to use debug prints for commands")]
    pub verbose: bool,

    #[arg(short = 'q', long, help = "whether or not to stop command output")]
    pub no_output: bool,
}

#[derive(Subcommand)]
pub enum Commands {
    #[command(
        name = "docgen",
        about = "automatically generate zensical markdown docs from doc comments"
    )]
    Docgen {
        #[arg(short, long, help = "whether to not serve via `uvx zensical serve`")]
        no_serve: bool,
    },

    #[command(about = "add a new crate to the crates directory")]
    New {
        #[arg(help = "the name of the crate")]
        crate_name: String,

        #[arg(help = "the dependencies of the crate to add")]
        dependencies: Option<Vec<String>>,
    },

    #[command(about = "publish a crate to crates.io")]
    Publish {
        #[arg(help = "the crates to publish")]
        crates: Option<Vec<String>>,
    },

    #[command(about = "push your changes to the luduvo-rs repo")]
    Push {
        branch: String,

        #[arg(short, long)]
        title: String,

        #[arg(short, long)]
        description: Option<String>,

        #[arg(short = 'p', long)]
        publish: bool,
    },

    #[command(name = "qa", about = "runs tests and ensures code quality")]
    Qa,

    #[command(about = "run tests for a crate")]
    Test {
        #[arg(help = "the crates to run tests for")]
        crates: Option<Vec<String>>,
    },
}
