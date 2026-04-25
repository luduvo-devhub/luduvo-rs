use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "runner")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,

    #[arg(short, long)]
    pub verbose: bool,

    #[arg(short = 'q', long)]
    pub no_output: bool,
}

#[derive(Subcommand)]
pub enum Commands {
    Qa,
    Publish,

    Push {
        #[arg(short, long)]
        title: String,

        #[arg(short, long)]
        description: Option<String>,
    },

    Test {
        crates: Vec<String>,
    },
}
