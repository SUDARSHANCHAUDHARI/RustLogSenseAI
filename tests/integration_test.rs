use assert_cmd::Command;
use predicates::str::contains;
use tempfile::NamedTempFile;
use std::io::Write;

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
fn test_scan_file() {
    let mut tmp = NamedTempFile::new().unwrap();
    writeln!(tmp, "INFO server started").unwrap();
    writeln!(tmp, "ERROR connection failed").unwrap();
    writeln!(tmp, "WARN high memory usage").unwrap();

    Command::cargo_bin("logscope")
        .unwrap()
        .args(["scan", tmp.path().to_str().unwrap()])
        .assert()
        .success()
        .stdout(contains("LogScope Report"));
}
