use anyhow::Result;
use logscope::report::ScopeReport;

pub fn print(report: &ScopeReport) -> Result<()> {
    println!("{}", serde_json::to_string_pretty(report)?);
    Ok(())
}
