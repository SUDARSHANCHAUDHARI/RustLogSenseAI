# RustLogSenseAI

[![crates.io](https://img.shields.io/crates/v/logsenseai?logo=rust)](https://crates.io/crates/logsenseai)
[![Downloads](https://img.shields.io/crates/d/logsenseai?logo=rust)](https://crates.io/crates/logsenseai)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)
![Rust](https://img.shields.io/badge/Rust-1.75%2B-orange?logo=rust)

> A small, fast Rust CLI that turns plain-text log files into a useful health summary.

**RustLogSenseAI** (installed as the `logsenseai` command) scans plain-text log files,
counts log levels, highlights repeated errors, detects high error-rate anomalies, and
emits either readable terminal output or JSON for automation.

## Table of Contents

- [Overview](#overview)
- [Features](#features)
- [Installation](#installation)
- [Usage](#usage)
- [Included Example](#included-example)
- [Log Level Detection](#log-level-detection)
- [Health Verdicts](#health-verdicts)
- [Example Output](#example-output)
- [JSON Use Cases](#json-use-cases)
- [Development](#development)
- [Project Structure](#project-structure)
- [Documentation](#documentation)
- [Release Status](#release-status)
- [License](#license)
- [About](#about)

## Overview

When a service is noisy, the first useful question is often simple: how many errors, how
many warnings, which messages repeat, and is this log healthy enough to ignore?
RustLogSenseAI answers that quickly without requiring a full observability stack.

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

### From crates.io (recommended)

```bash
cargo install logsenseai
```

### From source

```bash
git clone https://github.com/SUDARSHANCHAUDHARI/RustLogSenseAI.git
cd RustLogSenseAI
cargo build --release
```

The binary is created at:

```bash
target/release/logsenseai
```

Optional local install from a source checkout:

```bash
cargo install --path .
```

## Usage

```bash
# Scan a log file
logsenseai scan app.log

# Emit JSON
logsenseai scan app.log --json

# Read from stdin
cat app.log | logsenseai scan -
```

## Included Example

The repository includes a sample log file:

```bash
logsenseai scan examples/app.sample

cat examples/app.sample | logsenseai scan -
```

Real output:

```text
LogScope Report
Path: examples/app.sample
Total Lines: 9
Errors: 3
Warnings: 1
Info: 4
Health: Degraded

Top Errors:
  [2x] ERROR connection refused to db host
  [1x] ERROR timeout after 30s

Anomalies:
  High error rate: 33.3% of log lines are errors
```

When the path is `-`, LogScope reads all input from stdin and reports the source as `<stdin>`.

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

JSON output is useful for CI, scheduled jobs, local troubleshooting scripts, and dashboards
that need a lightweight summary without parsing terminal text.

```bash
logsenseai scan examples/app.sample --json > report.json
```

## Development

```bash
cargo fmt --check
cargo clippy -- -D warnings
cargo test
cargo build --release
```

The integration test suite covers parser behavior, health thresholds, anomaly detection,
top error grouping, file input, stdin input, and CLI output.

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

## Documentation

- [Architecture](docs/ARCHITECTURE.md)
- [Roadmap](docs/ROADMAP.md)
- [Maintainer notes](docs/NOTES.md)
- [Content plan](docs/CONTENT_PLAN.md)

## Release Status

Current release: **`v1.1.1`**, published on [crates.io](https://crates.io/crates/logsenseai).

Each release is verified with formatting, Clippy, tests, an optimized release build, and
`cargo package` before publishing.

## License

MIT — see [LICENSE](LICENSE).

---

## About

I'm Sudarshan Chaudhari, a Senior Quality Engineer, Test Automation specialist, and AI systems builder based in Bangkok, Thailand.

I have 13+ years of experience in software quality engineering, working across SaaS, fintech, gaming, web, mobile, cloud, and digital signage platforms. My background combines hands-on test automation with QA leadership, test strategy, CI/CD, release quality, production investigation, and cross-platform validation.

Alongside my professional QA career, I run [SudarshanTechLabs](https://sudarshantechlabs.com/), my independent engineering and product lab where I design, build, test, and ship software across Android, web, AI, cybersecurity, developer tooling, and cross-platform applications.

### What I work on

- ⚙️ **Quality Engineering & Test Automation** — Playwright, Selenium, Cypress, Appium, API testing, automation frameworks, end-to-end testing, CI/CD, release gates, GitHub Actions, risk-based testing, and production validation
- 🤖 **AI Systems & Automation** — AI agents, multi-agent orchestration, MCP servers, AI-assisted QA, prompt tooling, developer workflows, automation systems, and Claude Code plugins
- 📱 **Mobile & Cross-Platform Applications** — Android applications built with Kotlin and Jetpack Compose, Google Play releases, automated build and publishing pipelines, and cross-platform development spanning iOS, web, Windows, and macOS
- 🌐 **Web Applications & Platforms** — Full-stack applications using Next.js, TypeScript, Firebase, Cloudflare, REST APIs, and modern web infrastructure
- 🛠️ **Developer Tooling & CLI Engineering** — Rust, Python, TypeScript, CLI utilities, multi-repository tooling, build automation, release tooling, and engineering productivity systems
- 🛡️ **Cybersecurity & Observability** — Threat detection, log analysis, security auditing, vulnerability assessment, monitoring, and security-focused developer tools
- 📺 **Digital Signage & Device Platforms** — Content validation, playback testing, device compatibility, production investigation, monitoring, and QA across diverse hardware and operating-system environments

My work sits at the intersection of quality engineering, automation, AI, and software development. I approach products with a QA mindset from the beginning: understanding failure modes, designing for testability, automating repetitive work, and building release confidence into the engineering process.

Through SudarshanTechLabs, I also build products and tools from idea to production, covering architecture, development, testing, CI/CD, release automation, monitoring, and ongoing maintenance.

🌐 [sudarshantechlabs.com](https://sudarshantechlabs.com/) · 💼 [LinkedIn](https://linkedin.com/in/sudarshan-chaudhari) · 🐙 [GitHub](https://github.com/SUDARSHANCHAUDHARI) · ✉️ [sunny.sudarshan@gmail.com](mailto:sunny.sudarshan@gmail.com)
