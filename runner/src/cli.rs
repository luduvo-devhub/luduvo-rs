use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "runner", about = "a fancy command runner that replaces github actions")]
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
    #[command(name = "qa", about = "runs tests and makes sure code quality is good")]
    Qa,

    #[command(about = "publish a crate to crates.io")]
    Publish {
        #[arg(help = "the crates to publish")]
        crates: Option<Vec<String>>,
    },

    #[command(about = "push your changes to the luduvo-rs repo")]
    Push {
            #[arg(short, long)]
            title: String,

            #[arg(short, long)]
            description: Option<String>,

            #[arg(short = 'p', long)]
            publish: bool,
        },

    #[command(about = "run tests for a crate")]
    Test {
        #[arg(help = "the crates to run tests for")]
        crates: Option<Vec<String>>,
    },
}
