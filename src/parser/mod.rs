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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_error_level() {
        let entries = parse("2024-01-01 ERROR something broke\n2024-01-01 FATAL critical issue");
        assert_eq!(entries.len(), 2);
        assert!(entries.iter().all(|e| e.level == LogLevel::Error));
    }

    #[test]
    fn parses_warn_level() {
        let entry = &parse("2024-01-01 WARN disk almost full")[0];
        assert_eq!(entry.level, LogLevel::Warn);
    }

    #[test]
    fn parses_info_level() {
        let entry = &parse("2024-01-01 INFO server started")[0];
        assert_eq!(entry.level, LogLevel::Info);
    }

    #[test]
    fn parses_debug_level() {
        let entry = &parse("2024-01-01 DEBUG cache miss")[0];
        assert_eq!(entry.level, LogLevel::Debug);
    }

    #[test]
    fn unknown_level_for_unclassified_lines() {
        let entry = &parse("some random log line without level")[0];
        assert_eq!(entry.level, LogLevel::Unknown);
    }

    #[test]
    fn skips_blank_lines() {
        let entries = parse("ERROR line\n\n  \nINFO line");
        assert_eq!(entries.len(), 2);
    }

    #[test]
    fn empty_input_returns_empty() {
        assert!(parse("").is_empty());
    }
}
