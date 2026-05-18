use serde::{Deserialize, Serialize};
use crate::analyzer::Analysis;
use crate::parser::LogEntry;

#[derive(Debug, Serialize, Deserialize)]
pub struct ScopeReport {
    pub path: String,
    pub total_lines: usize,
    pub error_count: usize,
    pub warn_count: usize,
    pub info_count: usize,
    pub top_errors: Vec<String>,
    pub anomalies: Vec<String>,
    pub health: Health,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Health {
    Good,
    Degraded,
    Critical,
}

impl std::fmt::Display for Health {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Health::Good => write!(f, "Good"),
            Health::Degraded => write!(f, "Degraded"),
            Health::Critical => write!(f, "Critical"),
        }
    }
}

pub fn build(path: &str, entries: &[LogEntry], analysis: &Analysis) -> ScopeReport {
    let health = if analysis.error_count == 0 {
        Health::Good
    } else if analysis.error_count < 10 {
        Health::Degraded
    } else {
        Health::Critical
    };

    ScopeReport {
        path: path.to_string(),
        total_lines: entries.len(),
        error_count: analysis.error_count,
        warn_count: analysis.warn_count,
        info_count: analysis.info_count,
        top_errors: analysis.top_errors.clone(),
        anomalies: analysis.anomalies.clone(),
        health,
    }
}
