# claude-code-statusline-widgets

High-level widgets that map `StatusLineInput` fields to rendered status line output. Each widget combines one or more components from `claude-code-statusline-components` to produce a meaningful display unit.

## The `Widget` trait

All widgets implement a common trait:

```rust
pub trait Widget {
    fn render(&self, input: &StatusLineInput) -> Option<String>;
}
```

Returns `None` when the required data is absent (e.g., `AgentInfo` returns `None` if no agent is active).

## Widgets

| Widget | Renders | Returns `None` when |
|---|---|---|
| `ModelInfo` | Model display name; `short` option strips parenthesized suffixes | never |
| `CostSummary` | Cost, duration, and optionally lines changed; configurable icon prefixes (`cost_prefix`, `duration_prefix`, `lines_prefix`) | never |
| `ContextUsage` | Context window progress bar with optional token counts | never |
| `TokenAlert` | Warning indicator when tokens exceed 200k | tokens are under threshold |
| `VimStatus` | Current vim mode (NORMAL / INSERT) with optional per-mode bg/fg colors | vim mode is disabled |
| `AgentInfo` | Active agent name | no agent is active |
| `WorktreeInfo` | Worktree branch or name | not in a worktree session |
| `WorkspaceInfo` | Current working directory | never |
| `GitBranch` | Current git branch (runs `git branch --show-current`) | not a git repo or detached HEAD |
| `RateLimit` | Rate limit progress bar with reset countdown; configurable `reset_separator` | no rate limit data (non-Claude.ai) |

## Usage

```rust
use claude_code_statusline_model::StatusLineInput;
use claude_code_statusline_widgets::{Widget, ModelInfo, CostSummary, ContextUsage};

let input: StatusLineInput = serde_json::from_str(&json)?;

let parts: Vec<String> = [
    ModelInfo::default().render(&input),
    CostSummary::default().render(&input),
    ContextUsage::default().render(&input),
]
.into_iter()
.flatten()
.collect();

println!("{}", parts.join(" | "));
```

Each widget is customizable through its public fields, which hold the underlying component configurations.
