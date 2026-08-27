//! Serde model definitions for the Claude Code status line input JSON.
//!
//! This crate provides strongly-typed Rust structs for deserializing the JSON
//! payload that Claude Code sends to custom status line scripts via stdin.
//! Every field mirrors the official schema so that downstream crates can work
//! with validated, typed data instead of raw JSON.
//!
//! # Usage
//!
//! ```no_run
//! use claude_code_statusline_model::StatusLineInput;
//!
//! let json = std::io::read_to_string(std::io::stdin()).unwrap();
//! let input: StatusLineInput = serde_json::from_str(&json).unwrap();
//! println!("Model: {}", input.model.display_name);
//! ```
//!
//! Based on the official documentation:
//! <https://code.claude.com/docs/en/statusline>

use serde::{Deserialize, Serialize};

/// Root structure for the JSON data that Claude Code sends to status line scripts via stdin.
///
/// This is the top-level object. Only `cwd`, `session_id`, `model`, and
/// `workspace` are required to deserialize; every other field falls back to
/// its `Default` (or `None`) when absent so that a partial payload degrades
/// gracefully instead of failing to parse. Feature-gated objects (`vim`,
/// `agent`, `worktree`, `rate_limits`) appear only when their corresponding
/// feature is active.
///
/// See: <https://code.claude.com/docs/en/statusline#available-data>
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct StatusLineInput {
    /// Current working directory.
    /// Same value as `workspace.current_dir`; `workspace.current_dir` is preferred.
    pub cwd: String,

    /// Unique session identifier.
    pub session_id: String,

    /// Session name: the custom name set with `--name` / `/rename`, or the
    /// AI-generated title. Absent when neither exists (the default display
    /// name such as `my-app-3f` does not populate this field).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub session_name: Option<String>,

    /// UUID of the user prompt currently being processed.
    /// Absent until the first user input.
    ///
    /// Requires Claude Code v2.1.196 or later.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub prompt_id: Option<String>,

    /// Path to conversation transcript file.
    #[serde(default)]
    pub transcript_path: String,

    /// Current model information.
    pub model: Model,

    /// Workspace directory information.
    pub workspace: Workspace,

    /// Claude Code version.
    #[serde(default)]
    pub version: String,

    /// Current output style configuration.
    #[serde(default)]
    pub output_style: OutputStyle,

    /// Session cost and duration tracking.
    #[serde(default)]
    pub cost: Cost,

    /// Context window usage information.
    #[serde(default)]
    pub context_window: ContextWindow,

    /// Whether the total token count from the most recent API response exceeds 200k.
    /// This is a fixed threshold regardless of actual context window size.
    #[serde(default)]
    pub exceeds_200k_tokens: bool,

    /// Reasoning effort level. Only present when the current model supports
    /// the effort parameter. Reflects live `/effort` changes.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub effort: Option<Effort>,

    /// Vim mode information. Only present when vim mode is enabled.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub vim: Option<Vim>,

    /// Agent information. Only present when running with `--agent` flag or agent settings configured.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub agent: Option<Agent>,

    /// Open pull request (or GitLab merge request) for the current branch.
    /// Present only while one is found; removed once it merges or closes.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pr: Option<Pr>,

    /// Worktree information. Only present during `--worktree` sessions.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub worktree: Option<Worktree>,

    /// Rate limit usage for Claude.ai. Only present for Claude.ai Pro/Max
    /// subscribers, after the first API response in the session.
    ///
    /// Added in v2.1.80.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rate_limits: Option<RateLimits>,
}

/// Current model identifier and display name.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Model {
    /// Model identifier (e.g., `"claude-opus-4-6"`).
    pub id: String,

    /// Model display name (e.g., `"Opus"`).
    pub display_name: String,
}

/// Workspace directory information.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Workspace {
    /// Current working directory.
    pub current_dir: String,

    /// Directory where Claude Code was launched.
    /// May differ from `current_dir` if the working directory changes during a session.
    #[serde(default)]
    pub project_dir: String,

    /// Directories added via `/add-dir`.
    ///
    /// Added in v2.1.47.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub added_dirs: Option<Vec<String>>,

    /// Git worktree name when the current directory is inside a linked
    /// worktree created with `git worktree add`. Absent in the main working
    /// tree. Unlike [`StatusLineInput::worktree`], this is populated for any
    /// git worktree, not only worktree sessions.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub git_worktree: Option<String>,

    /// Repository identity parsed from the `origin` remote.
    /// Absent outside a git repository or when no `origin` remote is configured.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub repo: Option<Repo>,
}

/// Repository identity parsed from the `origin` remote.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Repo {
    /// Host name (e.g., `"github.com"`).
    #[serde(default)]
    pub host: String,

    /// Repository owner (e.g., `"anthropics"`).
    #[serde(default)]
    pub owner: String,

    /// Repository name (e.g., `"claude-code"`).
    #[serde(default)]
    pub name: String,
}

/// Current output style configuration.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct OutputStyle {
    /// Name of the current output style.
    #[serde(default)]
    pub name: String,
}

/// Session cost and duration tracking.
///
/// Every field defaults to `0` when absent.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct Cost {
    /// Total session cost in USD.
    pub total_cost_usd: f64,

    /// Total wall-clock time since the session started, in milliseconds.
    pub total_duration_ms: u64,

    /// Total time spent waiting for API responses in milliseconds.
    pub total_api_duration_ms: u64,

    /// Lines of code added during the session.
    pub total_lines_added: u64,

    /// Lines of code removed during the session.
    pub total_lines_removed: u64,
}

/// Context window usage information.
///
/// Every field defaults to `0` / `None` when absent.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct ContextWindow {
    /// Cumulative input token count across the entire session.
    pub total_input_tokens: u64,

    /// Cumulative output token count across the entire session.
    pub total_output_tokens: u64,

    /// Maximum context window size in tokens.
    /// 200000 by default, or 1000000 for models with extended context.
    pub context_window_size: u64,

    /// Pre-calculated percentage of context window used.
    /// Calculated from input tokens only.
    /// May be `null` early in the session.
    pub used_percentage: Option<f64>,

    /// Pre-calculated percentage of context window remaining.
    /// May be `null` early in the session.
    pub remaining_percentage: Option<f64>,

    /// Token counts from the most recent API call.
    /// `null` before the first API call in a session.
    pub current_usage: Option<CurrentUsage>,
}

/// Token counts from the most recent API call.
///
/// Every field defaults to `0` when absent.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct CurrentUsage {
    /// Input tokens in current context.
    pub input_tokens: u64,

    /// Output tokens generated.
    pub output_tokens: u64,

    /// Tokens written to cache.
    pub cache_creation_input_tokens: u64,

    /// Tokens read from cache.
    pub cache_read_input_tokens: u64,
}

/// Reasoning effort configuration.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Effort {
    /// Effort level: `"low"`, `"medium"`, `"high"`, `"xhigh"`, or `"max"`.
    /// Ultracode is not a distinct level and reports as `"xhigh"`.
    pub level: String,
}

/// Vim mode information.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Vim {
    /// Current vim mode.
    pub mode: VimMode,
}

/// Vim mode variants.
///
/// Serialized as uppercase strings (`"NORMAL"`, `"INSERT"`, `"VISUAL"`,
/// `"VISUAL LINE"`) to match the JSON schema. Any other string deserializes
/// to [`VimMode::Unknown`] so that a new mode added by Claude Code never
/// breaks parsing of the whole payload.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum VimMode {
    /// Normal (command) mode.
    #[default]
    Normal,
    /// Insert (editing) mode.
    Insert,
    /// Visual (character-wise selection) mode.
    Visual,
    /// Visual line (line-wise selection) mode.
    #[serde(rename = "VISUAL LINE")]
    VisualLine,
    /// Any mode string not listed above.
    #[serde(other)]
    Unknown,
}

/// Agent information.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Agent {
    /// Agent name.
    pub name: String,
}

/// Open pull request / merge request for the current branch.
///
/// Mirrors the PR badge in the Claude Code footer.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Pr {
    /// Pull request number (merge request number for GitLab).
    pub number: u64,

    /// URL of the pull request.
    #[serde(default)]
    pub url: String,

    /// Review status: `"approved"`, `"pending"`, `"changes_requested"`, or
    /// `"draft"`. May be absent even when `pr` is present.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub review_state: Option<String>,

    /// `"mr"` when this describes a GitLab merge request. Absent for GitHub
    /// pull requests.
    ///
    /// Requires Claude Code v2.1.234 or later.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
}

/// Worktree information.
///
/// Only `name` is required; hook-based worktrees may omit the branch fields
/// and the rest is tolerated as optional for robustness.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Worktree {
    /// Name of the active worktree.
    pub name: String,

    /// Absolute path to the worktree directory.
    #[serde(default)]
    pub path: String,

    /// Git branch name for the worktree (e.g., `"worktree-my-feature"`).
    /// Absent for hook-based worktrees.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub branch: Option<String>,

    /// The directory Claude was in before entering the worktree.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub original_cwd: Option<String>,

    /// Git branch checked out before entering the worktree.
    /// Absent for hook-based worktrees.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub original_branch: Option<String>,
}

/// Rate limit usage for Claude.ai.
///
/// Each window may be independently absent.
///
/// Added in v2.1.80.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct RateLimits {
    /// 5-hour rolling window rate limit.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub five_hour: Option<RateLimitWindow>,

    /// 7-day rolling window rate limit.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub seven_day: Option<RateLimitWindow>,
}

/// A single rate limit window.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct RateLimitWindow {
    /// Percentage of the rate limit used.
    pub used_percentage: f64,

    /// Unix timestamp (seconds) when the rate limit resets.
    pub resets_at: i64,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deserialize_full_example() {
        let json = r#"{
            "cwd": "/current/working/directory",
            "session_id": "abc123",
            "session_name": "my-session",
            "prompt_id": "550e8400-e29b-41d4-a716-446655440000",
            "transcript_path": "/path/to/transcript.jsonl",
            "model": {
                "id": "claude-opus-4-6",
                "display_name": "Opus"
            },
            "workspace": {
                "current_dir": "/current/working/directory",
                "project_dir": "/original/project/directory",
                "added_dirs": ["/extra/dir1", "/extra/dir2"],
                "git_worktree": "feature-xyz",
                "repo": {
                    "host": "github.com",
                    "owner": "anthropics",
                    "name": "claude-code"
                }
            },
            "version": "1.0.80",
            "output_style": {
                "name": "default"
            },
            "cost": {
                "total_cost_usd": 0.01234,
                "total_duration_ms": 45000,
                "total_api_duration_ms": 2300,
                "total_lines_added": 156,
                "total_lines_removed": 23
            },
            "context_window": {
                "total_input_tokens": 15234,
                "total_output_tokens": 4521,
                "context_window_size": 200000,
                "used_percentage": 8,
                "remaining_percentage": 92,
                "current_usage": {
                    "input_tokens": 8500,
                    "output_tokens": 1200,
                    "cache_creation_input_tokens": 5000,
                    "cache_read_input_tokens": 2000
                }
            },
            "exceeds_200k_tokens": false,
            "effort": {
                "level": "high"
            },
            "vim": {
                "mode": "NORMAL"
            },
            "agent": {
                "name": "security-reviewer"
            },
            "pr": {
                "number": 1234,
                "url": "https://github.com/anthropics/claude-code/pull/1234",
                "review_state": "pending"
            },
            "worktree": {
                "name": "my-feature",
                "path": "/path/to/.claude/worktrees/my-feature",
                "branch": "worktree-my-feature",
                "original_cwd": "/path/to/project",
                "original_branch": "main"
            },
            "rate_limits": {
                "five_hour": {
                    "used_percentage": 42.5,
                    "resets_at": 1774029600
                },
                "seven_day": {
                    "used_percentage": 10.2,
                    "resets_at": 1774634400
                }
            }
        }"#;

        let input: StatusLineInput = serde_json::from_str(json).unwrap();

        assert_eq!(input.cwd, "/current/working/directory");
        assert_eq!(input.session_name.as_deref(), Some("my-session"));
        assert_eq!(input.prompt_id.as_deref(), Some("550e8400-e29b-41d4-a716-446655440000"));
        assert_eq!(input.model.id, "claude-opus-4-6");
        assert_eq!(input.model.display_name, "Opus");
        assert_eq!(input.workspace.project_dir, "/original/project/directory");
        assert_eq!(input.cost.total_cost_usd, 0.01234);
        assert_eq!(input.context_window.context_window_size, 200000);
        assert_eq!(input.context_window.used_percentage, Some(8.0));
        assert!(!input.exceeds_200k_tokens);

        assert_eq!(input.effort.unwrap().level, "high");

        let vim = input.vim.unwrap();
        assert_eq!(vim.mode, VimMode::Normal);

        let agent = input.agent.unwrap();
        assert_eq!(agent.name, "security-reviewer");

        let pr = input.pr.unwrap();
        assert_eq!(pr.number, 1234);
        assert_eq!(pr.review_state.as_deref(), Some("pending"));
        assert!(pr.kind.is_none());

        assert_eq!(
            input.workspace.added_dirs,
            Some(vec!["/extra/dir1".to_string(), "/extra/dir2".to_string()])
        );
        assert_eq!(input.workspace.git_worktree.as_deref(), Some("feature-xyz"));
        let repo = input.workspace.repo.unwrap();
        assert_eq!((repo.host.as_str(), repo.owner.as_str(), repo.name.as_str()),
                   ("github.com", "anthropics", "claude-code"));

        let worktree = input.worktree.unwrap();
        assert_eq!(worktree.name, "my-feature");
        assert_eq!(worktree.branch, Some("worktree-my-feature".to_string()));

        let rate_limits = input.rate_limits.unwrap();
        let five_hour = rate_limits.five_hour.unwrap();
        assert_eq!(five_hour.used_percentage, 42.5);
        assert_eq!(five_hour.resets_at, 1774029600);
        assert_eq!(rate_limits.seven_day.unwrap().used_percentage, 10.2);
    }

    #[test]
    fn deserialize_required_only() {
        let json = r#"{
            "cwd": "/p",
            "session_id": "s",
            "model": { "id": "m", "display_name": "M" },
            "workspace": { "current_dir": "/p" }
        }"#;

        let input: StatusLineInput = serde_json::from_str(json).unwrap();
        assert_eq!(input.workspace.project_dir, "");
        assert_eq!(input.version, "");
        assert_eq!(input.cost.total_cost_usd, 0.0);
        assert_eq!(input.context_window.context_window_size, 0);
        assert!(!input.exceeds_200k_tokens);
        assert!(input.rate_limits.is_none());
    }

    #[test]
    fn deserialize_partial_rate_limits() {
        let json = r#"{
            "five_hour": { "used_percentage": 23.5, "resets_at": 1738425600 }
        }"#;

        let limits: RateLimits = serde_json::from_str(json).unwrap();
        assert_eq!(limits.five_hour.unwrap().used_percentage, 23.5);
        assert!(limits.seven_day.is_none());

        let limits: RateLimits = serde_json::from_str("{}").unwrap();
        assert!(limits.five_hour.is_none());
        assert!(limits.seven_day.is_none());
    }

    #[test]
    fn deserialize_minimal_worktree() {
        let json = r#"{ "name": "my-feature" }"#;

        let wt: Worktree = serde_json::from_str(json).unwrap();
        assert_eq!(wt.name, "my-feature");
        assert_eq!(wt.path, "");
        assert!(wt.branch.is_none());
        assert!(wt.original_cwd.is_none());
        assert!(wt.original_branch.is_none());
    }

    #[test]
    fn deserialize_minimal() {
        let json = r#"{
            "cwd": "/home/user/project",
            "session_id": "sess-001",
            "transcript_path": "/tmp/transcript.jsonl",
            "model": { "id": "claude-sonnet-4-6", "display_name": "Sonnet" },
            "workspace": { "current_dir": "/home/user/project", "project_dir": "/home/user/project" },
            "version": "1.0.80",
            "output_style": { "name": "default" },
            "cost": {
                "total_cost_usd": 0.0,
                "total_duration_ms": 0,
                "total_api_duration_ms": 0,
                "total_lines_added": 0,
                "total_lines_removed": 0
            },
            "context_window": {
                "total_input_tokens": 0,
                "total_output_tokens": 0,
                "context_window_size": 200000,
                "used_percentage": null,
                "remaining_percentage": null,
                "current_usage": null
            },
            "exceeds_200k_tokens": false
        }"#;

        let input: StatusLineInput = serde_json::from_str(json).unwrap();

        assert!(input.session_name.is_none());
        assert!(input.prompt_id.is_none());
        assert!(input.effort.is_none());
        assert!(input.vim.is_none());
        assert!(input.agent.is_none());
        assert!(input.pr.is_none());
        assert!(input.worktree.is_none());
        assert!(input.rate_limits.is_none());
        assert!(input.workspace.added_dirs.is_none());
        assert!(input.workspace.repo.is_none());
        assert!(input.workspace.git_worktree.is_none());
        assert!(input.context_window.used_percentage.is_none());
        assert!(input.context_window.current_usage.is_none());
    }

    #[test]
    fn deserialize_vim_modes() {
        for (json, expected) in [
            (r#"{"mode":"NORMAL"}"#, VimMode::Normal),
            (r#"{"mode":"INSERT"}"#, VimMode::Insert),
            (r#"{"mode":"VISUAL"}"#, VimMode::Visual),
            (r#"{"mode":"VISUAL LINE"}"#, VimMode::VisualLine),
            (r#"{"mode":"REPLACE"}"#, VimMode::Unknown),
        ] {
            let vim: Vim = serde_json::from_str(json).unwrap();
            assert_eq!(vim.mode, expected, "input: {json}");
        }
    }

    #[test]
    fn serialize_roundtrip() {
        let input = StatusLineInput {
            cwd: "/test".to_string(),
            session_id: "id".to_string(),
            transcript_path: "/t.jsonl".to_string(),
            model: Model {
                id: "claude-opus-4-6".to_string(),
                display_name: "Opus".to_string(),
            },
            workspace: Workspace {
                current_dir: "/test".to_string(),
                project_dir: "/test".to_string(),
                ..Default::default()
            },
            version: "1.0.0".to_string(),
            output_style: OutputStyle {
                name: "default".to_string(),
            },
            cost: Cost {
                total_cost_usd: 0.05,
                total_duration_ms: 10000,
                total_api_duration_ms: 5000,
                total_lines_added: 10,
                total_lines_removed: 3,
            },
            context_window: ContextWindow {
                total_input_tokens: 1000,
                total_output_tokens: 500,
                context_window_size: 200000,
                used_percentage: Some(5.0),
                remaining_percentage: Some(95.0),
                current_usage: None,
            },
            ..Default::default()
        };

        let json = serde_json::to_string(&input).unwrap();
        let deserialized: StatusLineInput = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized.model.id, "claude-opus-4-6");
        assert_eq!(deserialized.cost.total_cost_usd, 0.05);
    }
}
