# claude-code-statusline-components

Low-level rendering components for building Claude Code status lines. Each component formats a single value type into a terminal-ready string, optionally with ANSI color codes.

## Components

| Component | Purpose | Example output |
|---|---|---|
| `ProgressBar` | Percentage as a colored bar | `██████░░░░ 60%` |
| `Duration` | Milliseconds to human-readable time | `2m 5s` |
| `Currency` | Float to formatted currency | `$0.05` |
| `Count` | Integer with optional style (plain, comma, compact) | `15.0k` |
| `Countdown` | Unix timestamp delta to remaining time | `2h 30m` |
| `Label` | Text with optional color and brackets | `[Opus]` |
| `Path` | File path with display style (basename, full, home-shortened) | `~/projects/myapp` |
| `Indicator` | Boolean flag to icon/text | (warning symbol when true) |

## Supporting utilities

- `color::Color` -- ANSI color enum (Red, Green, Yellow, ... , Ansi256)
- `color::Threshold` -- maps a value range to a color
- `color::colored()` / `color::color_for_value()` -- helpers for colorized output

## Usage

```rust
use claude_code_statusline_components::ProgressBar;
use claude_code_statusline_components::Currency;
use claude_code_statusline_components::Duration;

let bar = ProgressBar::default();
println!("{}", bar.render(73.0)); // colored bar at 73%

let cost = Currency::default();
println!("{}", cost.render(1.234)); // "$1.23"

let dur = Duration::default();
println!("{}", dur.render(125_000)); // "2m 5s"
```

All components implement `Default` with sensible defaults and can be customized through their public fields.
