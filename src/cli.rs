use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(
    name = "logsenseai",
    about = "Log parser and analyzer — detect anomalies and summarize logs",
    version
)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Scan a log file or directory
    Scan {
        /// Path to log file or directory
        path: String,
        /// Output as JSON
        #[arg(long)]
        json: bool,
    },
}
