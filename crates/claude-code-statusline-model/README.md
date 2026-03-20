# claude-code-statusline-model

Serde model definitions for the JSON data that Claude Code sends to status line scripts via stdin.

Based on the [official documentation](https://code.claude.com/docs/en/statusline).

## Key types

- `StatusLineInput` -- root structure containing all status line fields
- `Model` -- model identifier and display name
- `Workspace` -- current and project directory paths, plus any added dirs
- `Cost` -- session cost in USD, durations, and lines changed
- `ContextWindow` / `CurrentUsage` -- context window size and token counts
- `Vim` / `VimMode` -- vim mode state (NORMAL / INSERT)
- `Agent` -- active agent name
- `Worktree` -- worktree name, path, and branch info
- `RateLimits` / `RateLimitWindow` -- Claude.ai rate limit usage (`five_hour` and `seven_day` windows with Unix-timestamp `resets_at`)

Optional fields (`vim`, `agent`, `worktree`, `rate_limits`) are `Option` and only present when the corresponding feature is active.

## Usage

```rust
use claude_code_statusline_model::StatusLineInput;

let json = std::io::read_to_string(std::io::stdin())?;
let input: StatusLineInput = serde_json::from_str(&json)?;

println!("Model: {}", input.model.display_name);
println!("Cost:  ${:.2}", input.cost.total_cost_usd);

if let Some(vim) = &input.vim {
    println!("Vim mode: {:?}", vim.mode);
}
```
