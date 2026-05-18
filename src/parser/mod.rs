use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct LogEntry {
    pub line: String,
    pub level: LogLevel,
    pub message: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum LogLevel {
    Error,
    Warn,
    Info,
    Debug,
    Unknown,
}

pub fn parse(content: &str) -> Vec<LogEntry> {
    content
        .lines()
        .filter(|l| !l.trim().is_empty())
        .map(parse_line)
        .collect()
}

fn parse_line(line: &str) -> LogEntry {
    let upper = line.to_uppercase();
    let level = if upper.contains("ERROR") || upper.contains("FATAL") || upper.contains("CRIT") {
        LogLevel::Error
    } else if upper.contains("WARN") {
        LogLevel::Warn
    } else if upper.contains("INFO") {
        LogLevel::Info
    } else if upper.contains("DEBUG") {
        LogLevel::Debug
    } else {
        LogLevel::Unknown
    };

    LogEntry {
        message: line.to_string(),
        line: line.to_string(),
        level,
    }
}
