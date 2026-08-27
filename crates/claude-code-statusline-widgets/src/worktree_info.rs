//! Worktree information widget.
//!
//! Displays the active worktree session's branch name (preferred) or its
//! name as a fallback. With [`WorktreeSource::Any`] (the default) it also
//! shows `workspace.git_worktree`, which Claude Code populates for any
//! linked worktree created with `git worktree add`, not only for worktree
//! sessions. Returns `None` when neither is present.

use claude_code_statusline_components::label::Label;
use claude_code_statusline_model::StatusLineInput;

use crate::Widget;

/// Which inputs the worktree widget reads.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub enum WorktreeSource {
    /// `worktree.*` (worktree sessions) first, then `workspace.git_worktree`.
    #[default]
    Any,
    /// Only `worktree.*`; ignore plain git worktrees.
    Session,
}

/// Widget for displaying worktree information.
///
/// For a worktree session, shows the branch name if available, otherwise
/// the worktree name. Otherwise (with [`WorktreeSource::Any`]) shows the
/// linked git worktree name from `workspace.git_worktree`.
pub struct WorktreeInfo {
    /// Label formatter.
    pub label: Label,
    /// Which inputs to read.
    pub source: WorktreeSource,
}

impl Default for WorktreeInfo {
    fn default() -> Self {
        Self {
            label: Label { prefix: "🌲 ".into(), ..Default::default() },
            source: WorktreeSource::default(),
        }
    }
}

impl Widget for WorktreeInfo {
    fn render(&self, input: &StatusLineInput) -> Option<String> {
        if let Some(wt) = input.worktree.as_ref() {
            let display_name = wt.branch.as_deref().unwrap_or(&wt.name);
            return Some(self.label.render(display_name));
        }
        if self.source == WorktreeSource::Session {
            return None;
        }
        let name = input.workspace.git_worktree.as_deref()?;
        Some(self.label.render(name))
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

    fn make_git_worktree_input(name: &str) -> StatusLineInput {
        StatusLineInput {
            workspace: Workspace { git_worktree: Some(name.into()), ..Default::default() },
            ..Default::default()
        }
    }

    #[test]
    fn falls_back_to_git_worktree() {
        let w = WorktreeInfo::default();
        let result = w.render(&make_git_worktree_input("feature-xyz")).unwrap();
        assert!(result.contains("feature-xyz"));
    }

    #[test]
    fn session_source_ignores_git_worktree() {
        let w = WorktreeInfo { source: WorktreeSource::Session, ..Default::default() };
        assert!(w.render(&make_git_worktree_input("feature-xyz")).is_none());
    }

    #[test]
    fn session_worktree_wins_over_git_worktree() {
        let w = WorktreeInfo::default();
        let mut input = make_git_worktree_input("plain");
        input.worktree = Some(Worktree { name: "session".into(), ..Default::default() });
        assert!(w.render(&input).unwrap().contains("session"));
    }
}
