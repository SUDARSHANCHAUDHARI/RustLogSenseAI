mod cli;
mod output;

use logscope::{analyzer, parser, report};

use anyhow::Result;
use clap::Parser;
use cli::{Cli, Commands};
use std::io::{self, Read};

fn main() -> Result<()> {
    let cli = Cli::parse();
    match cli.command {
        Commands::Scan { path, json } => {
            let (content, report_path) = if path == "-" {
                let mut content = String::new();
                io::stdin().read_to_string(&mut content)?;
                (content, "<stdin>")
            } else {
                (std::fs::read_to_string(&path)?, path.as_str())
            };
            let entries = parser::parse(&content);
            let analysis = analyzer::analyze(&entries);
            let report = report::build(report_path, &entries, &analysis);
            if json {
                output::json::print(&report)?;
            } else {
                output::terminal::print(&report);
            }
        }
    }
    Ok(())
}
