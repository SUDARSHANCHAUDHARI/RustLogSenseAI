# RustLogSenseAI

![Rust](https://img.shields.io/badge/Rust-1.75%2B-orange?logo=rust)
![License](https://img.shields.io/badge/License-MIT-blue)

RustLogSenseAI is a small, fast Rust CLI for scanning plain-text log files and turning them into a useful health summary. It counts log levels, highlights repeated errors, detects high error-rate anomalies, and emits either readable terminal output or JSON for automation.

## Why This Exists

When a service is noisy, the first useful question is often simple: how many errors, how many warnings, which messages repeat, and is this log healthy enough to ignore? RustLogSenseAI answers that quickly without requiring a full observability stack.

## Features

- Scans any text log file line by line.
- Detects `ERROR`, `FATAL`, `CRIT`, `WARN`, `INFO`, and `DEBUG` levels.
- Treats log level matching as case-insensitive.
- Counts total lines and per-level totals.
- Groups repeated error messages.
- Caps top repeated errors to the most useful entries.
- Flags high error-rate anomalies when errors exceed 10% of total lines.
- Produces `Good`, `Degraded`, or `Critical` health verdicts.
- Supports terminal and JSON output.

## Installation

```bash
git clone https://github.com/SUDARSHANCHAUDHARI/RustLogSenseAI.git
cd RustLogSenseAI
cargo build --release
```

The binary is created at:

```bash
target/release/logscope
```

Optional local install:

```bash
cargo install --path .
```

## Usage

```bash
# Scan a log file
logscope scan app.log

# Emit JSON
logscope scan app.log --json
```

## Log Level Detection

| Keyword | Level |
|---|---|
| `ERROR`, `FATAL`, `CRIT` | Error |
| `WARN` | Warning |
| `INFO` | Info |
| `DEBUG` | Debug |
| Anything else | Unknown |

## Health Verdicts

| Verdict | Condition |
|---|---|
| `Good` | 0 errors |
| `Degraded` | 1 to 9 errors |
| `Critical` | 10 or more errors |

## Example Output

```text
LogScope Report
Path:          app.log
Total Lines:   120
Errors:        3
Warnings:      12
Info:          98
Health:        Degraded

Top Errors:
  [2x] ERROR connection refused to db host
  [1x] ERROR timeout after 30s

Anomalies:
  (none)
```

## JSON Use Cases

JSON output is useful for CI, scheduled jobs, local troubleshooting scripts, and dashboards that need a lightweight summary without parsing terminal text.

```bash
logscope scan app.log --json > report.json
```

## Development

```bash
cargo fmt --check
cargo clippy -- -D warnings
cargo test
cargo build --release
```

The integration test suite covers parser behavior, health thresholds, anomaly detection, top error grouping, and CLI output.

## Project Structure

```text
src/
  cli.rs          Command-line interface
  parser/         Log level parsing
  analyzer/       Counts, anomalies, and health logic
  report.rs       Terminal and JSON report model
tests/
  integration_test.rs
```

## Release Status

Current production release: `v1.0.0`

The `v1.0.0` release was verified with formatting, clippy, tests, optimized release build, and `cargo package`.

## License

MIT. See [LICENSE](LICENSE).

## Developer

Built by [Sudarshan Chaudhari](https://github.com/SUDARSHANCHAUDHARI) under SudarshanTechLabs.
