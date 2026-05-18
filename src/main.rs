mod analyzer;
mod cli;
mod output;
mod parser;
mod report;

use anyhow::Result;
use clap::Parser;
use cli::{Cli, Commands};

fn main() -> Result<()> {
    let cli = Cli::parse();
    match cli.command {
        Commands::Scan { path, json } => {
            let content = std::fs::read_to_string(&path)?;
            let entries = parser::parse(&content);
            let analysis = analyzer::analyze(&entries);
            let report = report::build(&path, &entries, &analysis);
            if json {
                output::json::print(&report)?;
            } else {
                output::terminal::print(&report);
            }
        }
    }
    Ok(())
}
