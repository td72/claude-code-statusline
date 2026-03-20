//! High-level widgets that map [`StatusLineInput`] fields to rendered output.
//!
//! Each widget combines one or more components from
//! [`claude_code_statusline_components`] to render a meaningful unit of
//! status line information.

use claude_code_statusline_model::StatusLineInput;

pub mod agent_info;
pub mod context_usage;
pub mod cost_summary;
pub mod model_info;
pub mod rate_limit;
pub mod token_alert;
pub mod vim_status;
pub mod workspace_info;
pub mod worktree_info;

pub use agent_info::AgentInfo;
pub use context_usage::ContextUsage;
pub use cost_summary::CostSummary;
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
/// Returns `None` if the required data is absent.
pub trait Widget {
    fn render(&self, input: &StatusLineInput) -> Option<String>;
}
