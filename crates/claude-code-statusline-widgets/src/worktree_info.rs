//! Worktree information widget.

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
            cwd: "/test".into(),
            session_id: "s".into(),
            transcript_path: "/t".into(),
            model: Model { id: "m".into(), display_name: "M".into() },
            workspace: Workspace {
                current_dir: "/test".into(),
                project_dir: "/test".into(),
                added_dirs: None,
            },
            version: "1.0".into(),
            output_style: OutputStyle { name: "default".into() },
            cost: Cost {
                total_cost_usd: 0.0,
                total_duration_ms: 0,
                total_api_duration_ms: 0,
                total_lines_added: 0,
                total_lines_removed: 0,
            },
            context_window: ContextWindow {
                total_input_tokens: 0,
                total_output_tokens: 0,
                context_window_size: 200_000,
                used_percentage: None,
                remaining_percentage: None,
                current_usage: None,
            },
            exceeds_200k_tokens: false,
            vim: None,
            agent: None,
            worktree,
            rate_limits: None,
        }
    }

    #[test]
    fn renders_branch_name() {
        let w = WorktreeInfo::default();
        let input = make_input(Some(Worktree {
            name: "my-feature".into(),
            path: "/path/to/.claude/worktrees/my-feature".into(),
            branch: Some("worktree-my-feature".into()),
            original_cwd: "/path/to/project".into(),
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
            original_cwd: "/path/to/project".into(),
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
