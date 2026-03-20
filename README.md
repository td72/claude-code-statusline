# claude-code-statusline

A configurable, multi-line status line for [Claude Code](https://code.claude.com/), written in Rust.

Reads JSON session data from stdin and renders a customizable status bar using ANSI colors. Layout and styling are fully driven by a TOML config file. Supports lualine-style badge layouts with background colors and ANSI 256-color codes.

```
 NORMAL 📁 my-project  main
🤖 Opus 4.6 ██░░░░░░░░ 17% (173k/1.00M)
💰 $0.87 | ⏱ 9m 2s | 📝 +423 -15
🕐 █░░░░ 24% ↻2h 30m   📅 █░░░░ 15% ↻2d 14h
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
widgets = ["vim", "workspace", "git_branch", "worktree"]
separator = ""

[[line]]
widgets = ["model", "context_usage"]
separator = " "

[[line]]
widgets = ["cost_summary"]
separator = " | "

[[line]]
widgets = ["rate_limit_5h", "rate_limit_7d"]
separator = "   "

# Customize each widget
[widget.model]
prefix = "🤖 "
short = true
color = "255"

[widget.context_usage]
bar_width = 10
show_tokens = true
token_style = "compact"
```

### Lualine-style badge layout

Use `separator = ""` with per-widget background colors (`bg`) for a lualine-style appearance. ANSI 256-color codes (`"0"` -- `"255"`) are supported alongside named colors.

```toml
[[line]]
widgets = ["vim", "workspace", "git_branch"]
separator = ""

# lualine theme: normal=#80d8ff, insert=#c3e88d, fg=#263238
[widget.vim]
normal_bg = "117"   # #87d7ff ≈ #80d8ff
normal_fg = "236"   # #303030 ≈ #263238
insert_bg = "150"   # #afd787 ≈ #c3e88d
insert_fg = "236"

# lualine section b: fg=#eeffff bg=#515559
[widget.workspace]
prefix = "📁 "
style = "basename"
color = "255"
bg = "240"

# lualine section c: fg=#eeffff bg=#2E3C43
[widget.git_branch]
prefix = " "
color = "255"
bg = "237"
```

### Available widgets

| Widget | Description |
|---|---|
| `model` | Model name (e.g., `Opus 4.6`); `short = true` strips parenthesized suffixes |
| `workspace` | Current directory |
| `git_branch` | Current git branch (via `git branch --show-current`; hidden outside a repo) |
| `agent` | Agent name (hidden when inactive) |
| `worktree` | Worktree branch (hidden when inactive) |
| `vim` | Vim mode with optional per-mode bg/fg colors (hidden when disabled) |
| `context_usage` | Context window progress bar + token counts |
| `cost_summary` | Cost, duration, lines changed; configurable icon prefixes |
| `token_alert` | Warning indicator when >200k tokens |
| `rate_limit_5h` | 5-hour rate limit usage + countdown |
| `rate_limit_7d` | 7-day rate limit usage + countdown |

Widgets that have no data (e.g., `agent` when not using `--agent`) are automatically skipped. If the JSON input cannot be parsed, a fallback message is displayed instead of a blank statusline.

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
