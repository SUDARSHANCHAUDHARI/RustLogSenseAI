# RustLogSenseAI

Rust CLI log parser and analyzer. Scans log files, counts error/warn/info levels, detects anomalies, and surfaces top errors with a health verdict.

## Install

```bash
cargo build --release
# binary at target/release/logscope
```

## Usage

```bash
# Scan a log file
logscope scan app.log

# JSON output
logscope scan app.log --json
```

## Log level detection

| Keyword | Level |
|---|---|
| `ERROR`, `FATAL`, `CRIT` | Error |
| `WARN` | Warn |
| `INFO` | Info |
| `DEBUG` | Debug |
| anything else | Unknown |

Case-insensitive. Auto-detected per line.

## Anomaly detection

- **High error rate** — flags when errors exceed 10% of total lines

## Health verdicts

| Verdict | Condition |
|---|---|
| `Good` | 0 errors |
| `Degraded` | 1–9 errors |
| `Critical` | 10+ errors |

## Output

```
LogScope Report
Path:         app.log
Total Lines:  120
Errors:       3
Warnings:     12
Info:         98
Health:       Degraded

Top Errors:
  [2x] ERROR connection refused to db host
  [1x] ERROR timeout after 30s

Anomalies:
  (none)
```

## Test

```bash
cargo test
```

19 integration tests — parser (all levels), analyzer, anomaly detection, health thresholds, CLI.

## Stack

Rust · clap · regex · serde · colored · chrono · anyhow
