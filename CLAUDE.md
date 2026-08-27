# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## What this is

A configurable, multi-line status line for Claude Code. Reads JSON session data from stdin, renders widgets based on TOML config, outputs ANSI-colored text to stdout.

## Commands

```bash
cargo build                          # Build all crates
cargo test --workspace               # Run all tests (120 tests across 4 crates)
cargo install --path .               # Install binary as `claude-code-statusline`

# Test with mock input
echo '{"cwd":"/tmp","session_id":"s","transcript_path":"/t","model":{"id":"m","display_name":"M"},"workspace":{"current_dir":"/tmp","project_dir":"/tmp"},"version":"1.0","output_style":{"name":"default"},"cost":{"total_cost_usd":0,"total_duration_ms":0,"total_api_duration_ms":0,"total_lines_added":0,"total_lines_removed":0},"context_window":{"total_input_tokens":0,"total_output_tokens":0,"context_window_size":200000,"used_percentage":null,"remaining_percentage":null,"current_usage":null},"exceeds_200k_tokens":false}' | cargo run
```

## Architecture

```
stdin JSON → model (parse) → widgets (render) → stdout
                                ↑
                          components (primitives)
                                ↑
                          config.toml (layout + styling)
```

**3 crates + 1 binary:**

- **model** — Serde types for Claude Code's statusline JSON. Based on https://code.claude.com/docs/en/statusline. Only `cwd`, `session_id`, `model`, `workspace` are required; everything else is `#[serde(default)]` / `Option` so a partial payload degrades instead of blanking the status line. All structs derive `Default` — build test inputs with `StatusLineInput { field, ..Default::default() }`.
- **components** — Rendering primitives: `ProgressBar`, `Duration`, `Currency`, `Count`, `Countdown`, `Label`, `Path`, `Indicator`, `Link` (OSC 8), plus `color` module for ANSI/256-color support.
- **widgets** — `Widget` trait (`render(&self, input: &StatusLineInput) -> Option<String>`) with 17 implementations. Returns `None` when data is absent (e.g., agent/vim/worktree/pr/effort).
- **src/** — CLI binary. `config.rs` loads TOML (--config flag → ~/.config/ → built-in default via `include_str!`). `builder.rs` maps widget names to constructed `Box<dyn Widget>`.

## Key patterns

- **`builder.rs` is the glue** — maps config widget names ("model", "context_usage", etc.) to widget structs. Add new widgets here.
- **`git_branch` runs `git branch --show-current`** — the only widget that executes external commands rather than reading from `StatusLineInput` (skipped when `worktree.branch` already carries the name).
- **`RateLimit.render_with_now()`** — accepts explicit timestamp for deterministic testing. `Widget::render()` wraps it with real system time.
- **Label bg + prefix interaction** — when `bg` is set, prefix is rendered inside the colored region (badge style). `WorkspaceInfo` builder moves prefix from Path to Label when bg is configured.
- **Color strings** — config accepts names ("red", "cyan") or ANSI 256 codes ("117", "236") parsed in `config::parse_color`.
- **Parse error handling** — on JSON parse failure, prints `⚠ statusline parse error` to stdout and error details to stderr (visible with `claude --debug`), exits 0 so the statusline doesn't go blank.

## Commit convention

Gitmoji prefix + English subject (≤50 chars). Examples: `✨ Add widget`, `🐛 Fix parse error`, `💄 Update styling`.
