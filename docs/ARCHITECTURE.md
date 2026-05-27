# Architecture

RustLogSenseAI is a small CLI for reading plain-text logs, counting severity levels, detecting repeated errors, and producing a health summary.

## Goals

- Provide a fast local log summary without a server.
- Keep parser, analyzer, and output concerns separate.
- Produce output that works for both humans and scripts.
- Stay useful for small production support workflows.

## Module Layout

| Module | Responsibility |
| --- | --- |
| `src/cli.rs` | CLI command and option parsing |
| `src/parser/` | Log line classification |
| `src/analyzer/` | Error-rate and repeated-error analysis |
| `src/report.rs` | Shared report data model |
| `src/output/` | Terminal and JSON rendering |

## Data Flow

1. The CLI receives a log file path and output mode.
2. The parser converts raw lines into structured log entries.
3. The analyzer computes counts, repeated errors, and health status.
4. The report model carries the final result.
5. The renderer prints terminal or JSON output.

## Design Notes

- Unknown lines are kept from crashing the run.
- Analyzer thresholds should remain easy to reason about.
- Output should remain deterministic for test fixtures.
- Parsing improvements should be fixture-driven.

## Release Assumptions

- `cargo fmt --check`, `cargo clippy -- -D warnings`, `cargo test`, and `cargo package` pass before release.
- GitHub Actions are intentionally not used in this repo.
