//! Serde model definitions for the Claude Code status line input JSON.
//!
//! Based on the official documentation:
//! <https://code.claude.com/docs/en/statusline>

use serde::{Deserialize, Serialize};

/// Root structure for the JSON data that Claude Code sends to status line scripts via stdin.
///
/// See: <https://code.claude.com/docs/en/statusline#available-data>
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StatusLineInput {
    /// Current working directory.
    /// Same value as `workspace.current_dir`; `workspace.current_dir` is preferred.
    pub cwd: String,

    /// Unique session identifier.
    pub session_id: String,

    /// Path to conversation transcript file.
    pub transcript_path: String,

    /// Current model information.
    pub model: Model,

    /// Workspace directory information.
    pub workspace: Workspace,

    /// Claude Code version.
    pub version: String,

    /// Current output style configuration.
    pub output_style: OutputStyle,

    /// Session cost and duration tracking.
    pub cost: Cost,

    /// Context window usage information.
    pub context_window: ContextWindow,

    /// Whether the total token count from the most recent API response exceeds 200k.
    /// This is a fixed threshold regardless of actual context window size.
    pub exceeds_200k_tokens: bool,

    /// Vim mode information. Only present when vim mode is enabled.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vim: Option<Vim>,

    /// Agent information. Only present when running with `--agent` flag or agent settings configured.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent: Option<Agent>,

    /// Worktree information. Only present during `--worktree` sessions.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub worktree: Option<Worktree>,

    /// Rate limit usage for Claude.ai. Only present for Claude.ai users.
    ///
    /// Added in v2.1.80.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rate_limits: Option<RateLimits>,
}

/// Current model identifier and display name.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Model {
    /// Model identifier (e.g., `"claude-opus-4-6"`).
    pub id: String,

    /// Model display name (e.g., `"Opus"`).
    pub display_name: String,
}

/// Workspace directory information.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Workspace {
    /// Current working directory.
    pub current_dir: String,

    /// Directory where Claude Code was launched.
    /// May differ from `current_dir` if the working directory changes during a session.
    pub project_dir: String,

    /// Directories added via `/add-dir`.
    ///
    /// Added in v2.1.47.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub added_dirs: Option<Vec<String>>,
}

/// Current output style configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OutputStyle {
    /// Name of the current output style.
    pub name: String,
}

/// Session cost and duration tracking.
#[derive(Debug, Clone, Serialize, Deserialize)]
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
#[derive(Debug, Clone, Serialize, Deserialize)]
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
#[derive(Debug, Clone, Serialize, Deserialize)]
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

/// Vim mode information.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Vim {
    /// Current vim mode.
    pub mode: VimMode,
}

/// Vim mode variants.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum VimMode {
    Normal,
    Insert,
}

/// Agent information.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Agent {
    /// Agent name.
    pub name: String,
}

/// Worktree information.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Worktree {
    /// Name of the active worktree.
    pub name: String,

    /// Absolute path to the worktree directory.
    pub path: String,

    /// Git branch name for the worktree (e.g., `"worktree-my-feature"`).
    /// Absent for hook-based worktrees.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub branch: Option<String>,

    /// The directory Claude was in before entering the worktree.
    pub original_cwd: String,

    /// Git branch checked out before entering the worktree.
    /// Absent for hook-based worktrees.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub original_branch: Option<String>,
}

/// Rate limit usage for Claude.ai.
///
/// Added in v2.1.80.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RateLimits {
    /// 5-hour rolling window rate limit.
    #[serde(rename = "5h")]
    pub five_hour: RateLimitWindow,

    /// 7-day rolling window rate limit.
    #[serde(rename = "7d")]
    pub seven_day: RateLimitWindow,
}

/// A single rate limit window.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RateLimitWindow {
    /// Percentage of the rate limit used.
    pub used_percentage: f64,

    /// ISO 8601 timestamp when the rate limit resets.
    pub resets_at: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deserialize_full_example() {
        let json = r#"{
            "cwd": "/current/working/directory",
            "session_id": "abc123",
            "transcript_path": "/path/to/transcript.jsonl",
            "model": {
                "id": "claude-opus-4-6",
                "display_name": "Opus"
            },
            "workspace": {
                "current_dir": "/current/working/directory",
                "project_dir": "/original/project/directory",
                "added_dirs": ["/extra/dir1", "/extra/dir2"]
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
            "vim": {
                "mode": "NORMAL"
            },
            "agent": {
                "name": "security-reviewer"
            },
            "worktree": {
                "name": "my-feature",
                "path": "/path/to/.claude/worktrees/my-feature",
                "branch": "worktree-my-feature",
                "original_cwd": "/path/to/project",
                "original_branch": "main"
            },
            "rate_limits": {
                "5h": {
                    "used_percentage": 42.5,
                    "resets_at": "2026-03-20T15:00:00Z"
                },
                "7d": {
                    "used_percentage": 10.2,
                    "resets_at": "2026-03-27T00:00:00Z"
                }
            }
        }"#;

        let input: StatusLineInput = serde_json::from_str(json).unwrap();

        assert_eq!(input.cwd, "/current/working/directory");
        assert_eq!(input.model.id, "claude-opus-4-6");
        assert_eq!(input.model.display_name, "Opus");
        assert_eq!(input.workspace.project_dir, "/original/project/directory");
        assert_eq!(input.cost.total_cost_usd, 0.01234);
        assert_eq!(input.context_window.context_window_size, 200000);
        assert_eq!(input.context_window.used_percentage, Some(8.0));
        assert!(!input.exceeds_200k_tokens);

        let vim = input.vim.unwrap();
        assert_eq!(vim.mode, VimMode::Normal);

        let agent = input.agent.unwrap();
        assert_eq!(agent.name, "security-reviewer");

        assert_eq!(
            input.workspace.added_dirs,
            Some(vec!["/extra/dir1".to_string(), "/extra/dir2".to_string()])
        );

        let worktree = input.worktree.unwrap();
        assert_eq!(worktree.name, "my-feature");
        assert_eq!(worktree.branch, Some("worktree-my-feature".to_string()));

        let rate_limits = input.rate_limits.unwrap();
        assert_eq!(rate_limits.five_hour.used_percentage, 42.5);
        assert_eq!(rate_limits.five_hour.resets_at, "2026-03-20T15:00:00Z");
        assert_eq!(rate_limits.seven_day.used_percentage, 10.2);
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

        assert!(input.vim.is_none());
        assert!(input.agent.is_none());
        assert!(input.worktree.is_none());
        assert!(input.rate_limits.is_none());
        assert!(input.workspace.added_dirs.is_none());
        assert!(input.context_window.used_percentage.is_none());
        assert!(input.context_window.current_usage.is_none());
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
                added_dirs: None,
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
            exceeds_200k_tokens: false,
            vim: None,
            agent: None,
            worktree: None,
            rate_limits: None,
        };

        let json = serde_json::to_string(&input).unwrap();
        let deserialized: StatusLineInput = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized.model.id, "claude-opus-4-6");
        assert_eq!(deserialized.cost.total_cost_usd, 0.05);
    }
}
