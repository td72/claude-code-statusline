//! High-level widgets that map [`StatusLineInput`] fields to rendered output.
//!
//! Each widget combines one or more low-level components from
//! [`claude_code_statusline_components`] to render a meaningful unit of
//! status line information.  Widgets implement the [`Widget`] trait, which
//! takes a reference to [`StatusLineInput`] and returns `Option<String>` --
//! `None` when the required data is absent (e.g., no active worktree).
//!
//! # Available widgets
//!
//! | Widget            | Renders                                      |
//! |-------------------|----------------------------------------------|
//! | [`ModelInfo`]      | Model display name                           |
//! | [`WorkspaceInfo`]  | Current working directory                    |
//! | [`AgentInfo`]      | Active agent name                            |
//! | [`WorktreeInfo`]   | Worktree branch/name                         |
//! | [`GitBranch`]      | Current git branch (via `git` CLI)           |
//! | [`VimStatus`]      | Vim mode indicator                           |
//! | [`ContextUsage`]   | Context window progress bar                  |
//! | [`CostSummary`]    | Session cost, duration, and lines changed    |
//! | [`TokenAlert`]     | 200k-token threshold warning                 |
//! | [`RateLimit`]      | Rate limit usage bar and reset countdown     |

use claude_code_statusline_model::StatusLineInput;

pub mod agent_info;
pub mod context_usage;
pub mod cost_summary;
pub mod git_branch;
pub mod model_info;
pub mod rate_limit;
pub mod token_alert;
pub mod vim_status;
pub mod workspace_info;
pub mod worktree_info;

pub use agent_info::AgentInfo;
pub use context_usage::ContextUsage;
pub use cost_summary::CostSummary;
pub use git_branch::GitBranch;
pub use model_info::ModelInfo;
pub use rate_limit::RateLimit;
pub use token_alert::TokenAlert;
pub use vim_status::VimStatus;
pub use workspace_info::WorkspaceInfo;
pub use worktree_info::WorktreeInfo;

/// Trait for widgets that render a portion of the status line.
///
/// A widget reads relevant fields from [`StatusLineInput`] and produces
/// a rendered string using one or more components.
/// Returns `None` if the required data is absent (e.g., when vim mode
/// is not enabled, the `VimStatus` widget returns `None`).
pub trait Widget {
    /// Render this widget for the given input.
    ///
    /// Returns `Some(rendered_string)` on success, or `None` if the
    /// widget's required data is not present in `input`.
    fn render(&self, input: &StatusLineInput) -> Option<String>;
}
