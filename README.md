# claude-code-statusline

A configurable, multi-line status line for [Claude Code](https://code.claude.com/), written in Rust.

Reads JSON session data from stdin and renders a customizable status bar using ANSI colors. Layout and styling are fully driven by a TOML config file.

```
[Opus 4.6 (1M context)] | 📁 my-project | 🤖 security-reviewer | [NORMAL]
██░░░░░░░░ 17% (173k/1.00M)
$0.87 | 9m 2s | +423 -15
5h: █░░░░ 24% resets in 2h 30m | 7d: █░░░░ 15% resets in 2d 14h
```

## Install

```sh
cargo install --path .
```

## Setup

Add to `~/.claude/settings.json`:

```json
{
  "statusLine": {
    "type": "command",
    "command": "claude-code-statusline"
  }
}
```

## Configuration

The config file is loaded in this order:

1. `--config <path>` flag
2. `~/.config/claude-code-statusline/config.toml`
3. Built-in default

Copy the default config to get started:

```sh
mkdir -p ~/.config/claude-code-statusline
cp config.default.toml ~/.config/claude-code-statusline/config.toml
```

### Config format

Define lines and widgets in TOML:

```toml
# Each [[line]] is a row in the status bar
[[line]]
widgets = ["model", "workspace", "agent", "worktree", "vim"]
separator = " | "

[[line]]
widgets = ["context_usage", "cost_summary", "token_alert", "rate_limit_5h"]
separator = " | "

# Customize each widget
[widget.model]
color = "cyan"
bracket = "square"

[widget.context_usage]
bar_width = 10
show_tokens = true
token_style = "compact"
```

### Available widgets

| Widget | Description |
|---|---|
| `model` | Model name (e.g., `[Opus]`) |
| `workspace` | Current directory |
| `agent` | Agent name (hidden when inactive) |
| `worktree` | Worktree branch (hidden when inactive) |
| `vim` | Vim mode (hidden when disabled) |
| `context_usage` | Context window progress bar + token counts |
| `cost_summary` | Cost, duration, lines changed |
| `token_alert` | Warning indicator when >200k tokens |
| `rate_limit_5h` | 5-hour rate limit usage + countdown |
| `rate_limit_7d` | 7-day rate limit usage + countdown |

Widgets that have no data (e.g., `agent` when not using `--agent`) are automatically skipped.

## Workspace structure

```
Cargo.toml                              # Workspace + CLI binary
config.default.toml                     # Default configuration
src/                                    # CLI application
crates/
  claude-code-statusline-model/         # Serde types for statusline JSON input
  claude-code-statusline-components/    # Primitive rendering components
  claude-code-statusline-widgets/       # High-level widgets (Widget trait)
```

## License

MIT
