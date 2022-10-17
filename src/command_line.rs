use clap::{command, Parser, Subcommand};

#[derive(Parser)]
#[clap(version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    Scan,
}
