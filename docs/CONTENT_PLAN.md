# Content Plan

## Positioning

RustLogSenseAI is a good solo-dev operations story: use a tiny Rust CLI to understand logs before adding heavier infrastructure.

## Blog Post Queue

| Priority | Working Title | Feature Tie-In |
| --- | --- | --- |
| 1 | What I Want From a Local Log Analyzer Before Production Debugging | Current analyzer |
| 2 | Designing Simple Error Thresholds That Developers Can Trust | Configurable thresholds |
| 3 | Turning CLI Output Into Support Notes | Markdown output |

## Auto-Blog Prompt Seed

Write a practical blog post about using a small Rust CLI to summarize application logs. Include a sample log file, the `logscope scan` command, terminal output, JSON output, and a section on when this should not replace real observability.

## Useful Examples

- `examples/app.sample`
- Error-rate anomaly example.
- Repeated error summary.
