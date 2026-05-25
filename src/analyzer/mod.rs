use crate::parser::{LogEntry, LogLevel};
use serde::{Deserialize, Serialize};
use std::cmp::Reverse;
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
pub struct Analysis {
    pub error_count: usize,
    pub warn_count: usize,
    pub info_count: usize,
    pub total_lines: usize,
    pub top_errors: Vec<String>,
    pub anomalies: Vec<String>,
}

pub fn analyze(entries: &[LogEntry]) -> Analysis {
    let error_count = entries
        .iter()
        .filter(|e| e.level == LogLevel::Error)
        .count();
    let warn_count = entries.iter().filter(|e| e.level == LogLevel::Warn).count();
    let info_count = entries.iter().filter(|e| e.level == LogLevel::Info).count();

    let mut pattern_counts: HashMap<String, usize> = HashMap::new();
    for entry in entries.iter().filter(|e| e.level == LogLevel::Error) {
        let key = entry.message.chars().take(60).collect::<String>();
        *pattern_counts.entry(key).or_insert(0) += 1;
    }

    let mut top_errors: Vec<(String, usize)> = pattern_counts.into_iter().collect();
    top_errors.sort_by_key(|entry| Reverse(entry.1));
    let top_errors: Vec<String> = top_errors
        .into_iter()
        .take(5)
        .map(|(k, v)| format!("[{}x] {}", v, k))
        .collect();

    let anomalies = detect_anomalies(entries);

    Analysis {
        error_count,
        warn_count,
        info_count,
        total_lines: entries.len(),
        top_errors,
        anomalies,
    }
}

fn detect_anomalies(entries: &[LogEntry]) -> Vec<String> {
    let mut anomalies = Vec::new();
    let error_rate = entries
        .iter()
        .filter(|e| e.level == LogLevel::Error)
        .count() as f64
        / entries.len().max(1) as f64;
    if error_rate > 0.1 {
        anomalies.push(format!(
            "High error rate: {:.1}% of log lines are errors",
            error_rate * 100.0
        ));
    }
    anomalies
}
