use colored::Colorize;
use logscope::report::{Health, ScopeReport};

pub fn print(report: &ScopeReport) {
    println!("\n{}", "LogScope Report".bold().underline());
    println!("{} {}", "Path:".bold(), report.path);
    println!("{} {}", "Total Lines:".bold(), report.total_lines);
    println!(
        "{} {}",
        "Errors:".bold(),
        report.error_count.to_string().red()
    );
    println!(
        "{} {}",
        "Warnings:".bold(),
        report.warn_count.to_string().yellow()
    );
    println!(
        "{} {}",
        "Info:".bold(),
        report.info_count.to_string().cyan()
    );

    let health_colored = match report.health {
        Health::Good => "Good".green().bold(),
        Health::Degraded => "Degraded".yellow().bold(),
        Health::Critical => "Critical".red().bold(),
    };
    println!("{} {}", "Health:".bold(), health_colored);

    if !report.top_errors.is_empty() {
        println!("\n{}", "Top Errors:".bold());
        for e in &report.top_errors {
            println!("  {}", e.dimmed());
        }
    }

    if !report.anomalies.is_empty() {
        println!("\n{}", "Anomalies:".bold());
        for a in &report.anomalies {
            println!("  {}", a.yellow());
        }
    }
    println!();
}
