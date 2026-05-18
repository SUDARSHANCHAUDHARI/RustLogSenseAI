use assert_cmd::Command;
use logscope::analyzer;
use logscope::parser::{self, LogLevel};
use logscope::report::{self, Health};
use predicates::str::contains;
use std::io::Write;
use tempfile::NamedTempFile;

// --- CLI tests ---

#[test]
fn test_help() {
    Command::cargo_bin("logscope")
        .unwrap()
        .arg("--help")
        .assert()
        .success()
        .stdout(contains("anomalies"));
}

#[test]
fn test_scan_subcommand_help() {
    Command::cargo_bin("logscope")
        .unwrap()
        .args(["scan", "--help"])
        .assert()
        .success()
        .stdout(contains("PATH"));
}

// --- Parser: level detection ---

#[test]
fn test_parse_error_line() {
    let entries = parser::parse("ERROR connection refused");
    assert_eq!(entries.len(), 1);
    assert_eq!(entries[0].level, LogLevel::Error);
}

#[test]
fn test_parse_fatal_is_error() {
    let entries = parser::parse("FATAL out of memory");
    assert_eq!(entries[0].level, LogLevel::Error);
}

#[test]
fn test_parse_warn_line() {
    let entries = parser::parse("WARN high memory usage");
    assert_eq!(entries[0].level, LogLevel::Warn);
}

#[test]
fn test_parse_info_line() {
    let entries = parser::parse("INFO server started on port 8080");
    assert_eq!(entries[0].level, LogLevel::Info);
}

#[test]
fn test_parse_debug_line() {
    let entries = parser::parse("DEBUG fetching config");
    assert_eq!(entries[0].level, LogLevel::Debug);
}

#[test]
fn test_parse_unknown_line() {
    let entries = parser::parse("2026-05-18 some random message");
    assert_eq!(entries[0].level, LogLevel::Unknown);
}

#[test]
fn test_parse_skips_empty_lines() {
    let entries = parser::parse("INFO start\n\n   \nERROR boom");
    assert_eq!(entries.len(), 2);
}

#[test]
fn test_parse_case_insensitive() {
    let entries = parser::parse("error something failed");
    assert_eq!(entries[0].level, LogLevel::Error);
}

// --- Analyzer: counts and anomalies ---

#[test]
fn test_analyzer_counts_levels_correctly() {
    let log = "ERROR one\nERROR two\nWARN three\nINFO four\nINFO five";
    let entries = parser::parse(log);
    let analysis = analyzer::analyze(&entries);
    assert_eq!(analysis.error_count, 2);
    assert_eq!(analysis.warn_count, 1);
    assert_eq!(analysis.info_count, 2);
    assert_eq!(analysis.total_lines, 5);
}

#[test]
fn test_anomaly_detected_above_10_percent_errors() {
    // 2 errors out of 10 = 20% — above threshold
    let log = "ERROR a\nERROR b\nINFO c\nINFO d\nINFO e\nINFO f\nINFO g\nINFO h\nINFO i\nINFO j";
    let entries = parser::parse(log);
    let analysis = analyzer::analyze(&entries);
    assert!(!analysis.anomalies.is_empty());
    assert!(analysis.anomalies[0].contains("error rate"));
}

#[test]
fn test_no_anomaly_below_10_percent_errors() {
    // 1 error out of 20 = 5% — below threshold
    let log = std::iter::once("ERROR one")
        .chain(std::iter::repeat("INFO ok").take(19))
        .collect::<Vec<_>>()
        .join("\n");
    let entries = parser::parse(&log);
    let analysis = analyzer::analyze(&entries);
    assert!(analysis.anomalies.is_empty());
}

#[test]
fn test_top_errors_capped_at_5() {
    let log = (1..=7)
        .map(|i| format!("ERROR unique error number {i}"))
        .collect::<Vec<_>>()
        .join("\n");
    let entries = parser::parse(&log);
    let analysis = analyzer::analyze(&entries);
    assert!(analysis.top_errors.len() <= 5);
}

// --- Report: health thresholds ---

#[test]
fn test_zero_errors_is_good() {
    let entries = parser::parse("INFO all fine");
    let analysis = analyzer::analyze(&entries);
    let rep = report::build("app.log", &entries, &analysis);
    assert!(matches!(rep.health, Health::Good));
}

#[test]
fn test_few_errors_is_degraded() {
    let log = (0..5).map(|_| "ERROR something failed").collect::<Vec<_>>().join("\n");
    let entries = parser::parse(&log);
    let analysis = analyzer::analyze(&entries);
    let rep = report::build("app.log", &entries, &analysis);
    assert!(matches!(rep.health, Health::Degraded));
}

#[test]
fn test_many_errors_is_critical() {
    let log = (0..15).map(|_| "ERROR something failed").collect::<Vec<_>>().join("\n");
    let entries = parser::parse(&log);
    let analysis = analyzer::analyze(&entries);
    let rep = report::build("app.log", &entries, &analysis);
    assert!(matches!(rep.health, Health::Critical));
}

// --- CLI integration with real files ---

#[test]
fn test_cli_scan_json_output() {
    let mut tmp = NamedTempFile::new().unwrap();
    writeln!(tmp, "INFO server started").unwrap();
    writeln!(tmp, "ERROR connection failed").unwrap();
    writeln!(tmp, "WARN high memory usage").unwrap();

    Command::cargo_bin("logscope")
        .unwrap()
        .args(["scan", tmp.path().to_str().unwrap(), "--json"])
        .assert()
        .success()
        .stdout(contains("error_count"));
}

#[test]
fn test_cli_scan_terminal_output() {
    let mut tmp = NamedTempFile::new().unwrap();
    writeln!(tmp, "INFO server started").unwrap();
    writeln!(tmp, "ERROR connection failed").unwrap();

    Command::cargo_bin("logscope")
        .unwrap()
        .args(["scan", tmp.path().to_str().unwrap()])
        .assert()
        .success()
        .stdout(contains("LogScope Report"));
}
