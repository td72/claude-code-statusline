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
| `Label` | Text with optional fg/bg color, brackets, prefix, and badge padding | `[Opus]` |
| `Path` | File path with display style (basename, full, home-shortened) | `~/projects/myapp` |
| `Indicator` | Boolean flag to icon/text | (warning symbol when true) |

## Supporting utilities

- `color::Color` -- ANSI color enum (Black, Red, Green, Yellow, Blue, Cyan, Magenta, White, Ansi256)
- `color::Color::fg()` / `fg_string()` -- foreground ANSI escape sequences
- `color::Color::bg()` / `bg_string()` -- background ANSI escape sequences
- `color::Threshold` -- maps a value range to a color
- `color::colored()` / `color::color_for_value()` -- helpers for colorized output

ANSI 256-color codes (0--255) are supported via `Color::Ansi256(n)`.

## Usage

```rust
use claude_code_statusline_components::ProgressBar;
use claude_code_statusline_components::Currency;
use claude_code_statusline_components::Duration;
use claude_code_statusline_components::Label;
use claude_code_statusline_components::color::Color;

let bar = ProgressBar::default();
println!("{}", bar.render(73.0)); // colored bar at 73%

let cost = Currency::default();
println!("{}", cost.render(1.234)); // "$1.23"

let dur = Duration::default();
println!("{}", dur.render(125_000)); // "2m 5s"

// Badge-style label with background color
let badge = Label {
    color: Some(Color::White),
    bg: Some(Color::Ansi256(240)),
    prefix: "📁 ".into(),
    pad: true,
    ..Default::default()
};
println!("{}", badge.render("my-project")); // " 📁 my-project " with bg
```

All components implement `Default` with sensible defaults and can be customized through their public fields.
