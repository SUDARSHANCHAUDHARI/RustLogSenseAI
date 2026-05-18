# RustLogSenseAI — Claude Code Context

## Purpose
Rust CLI log parser and analyzer. Scans log files, counts error/warn/info levels,
detects anomalies, surfaces top errors. Feeds LogSenseAI and SignageLogAI dashboards.

## Type
Rust CLI (logscope)

## Stack
- Language: Rust (stable)
- CLI: clap
- Regex: regex
- Serialization: serde + serde_json
- Errors: anyhow + thiserror
- Terminal: colored

## Commands
cargo run -- scan /var/log/syslog
cargo run -- scan app.log --json
cargo test
cargo clippy
cargo fmt
cargo build --release

## GitHub Repo
https://github.com/SUDARSHANCHAUDHARI/RustLogSenseAI
