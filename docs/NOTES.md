# Notes

## Why This Exists

Sometimes a developer just needs a quick read on a log file: how many errors, how many warnings, and whether one failure repeats enough to matter. RustLogSenseAI is built for that first pass.

## Known Limits

- It is not a replacement for centralized observability.
- It does not understand every application log format.
- Current analysis is line-oriented and intentionally simple.

## Maintenance Notes

- Add sample logs that are synthetic and safe.
- Keep parser rules understandable.
- Prefer small fixtures over large generated logs.
