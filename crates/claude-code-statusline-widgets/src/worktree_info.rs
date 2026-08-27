//! Worktree information widget.
//!
//! Displays the active worktree's branch name (preferred) or its name as a
//! fallback. Returns `None` when no worktree session is active.

use claude_code_statusline_components::label::Label;
use claude_code_statusline_model::StatusLineInput;

use crate::Widget;

/// Widget for displaying worktree information.
///
/// Shows the branch name if available, otherwise the worktree name.
pub struct WorktreeInfo {
    /// Label formatter.
    pub label: Label,
}

impl Default for WorktreeInfo {
    fn default() -> Self {
        Self {
            label: Label { prefix: "🌲 ".into(), ..Default::default() },
        }
    }
}

impl Widget for WorktreeInfo {
    fn render(&self, input: &StatusLineInput) -> Option<String> {
        let wt = input.worktree.as_ref()?;
        let display_name = wt.branch.as_deref().unwrap_or(&wt.name);
        Some(self.label.render(display_name))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use claude_code_statusline_model::*;

    fn make_input(worktree: Option<Worktree>) -> StatusLineInput {
        StatusLineInput {
            worktree,
            ..Default::default()
        }
    }

    #[test]
    fn renders_branch_name() {
        let w = WorktreeInfo::default();
        let input = make_input(Some(Worktree {
            name: "my-feature".into(),
            path: "/path/to/.claude/worktrees/my-feature".into(),
            branch: Some("worktree-my-feature".into()),
            original_cwd: Some("/path/to/project".into()),
            original_branch: Some("main".into()),
        }));
        let result = w.render(&input).unwrap();
        assert!(result.contains("worktree-my-feature"));
    }

    #[test]
    fn falls_back_to_name() {
        let w = WorktreeInfo::default();
        let input = make_input(Some(Worktree {
            name: "my-feature".into(),
            path: "/path/to/.claude/worktrees/my-feature".into(),
            branch: None,
            original_cwd: Some("/path/to/project".into()),
            original_branch: None,
        }));
        let result = w.render(&input).unwrap();
        assert!(result.contains("my-feature"));
    }

    #[test]
    fn returns_none_without_worktree() {
        let w = WorktreeInfo::default();
        let input = make_input(None);
        assert!(w.render(&input).is_none());
    }
}
