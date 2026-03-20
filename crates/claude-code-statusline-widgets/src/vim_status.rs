//! Vim mode status widget.

use claude_code_statusline_components::label::Label;
use claude_code_statusline_model::StatusLineInput;

use crate::Widget;

/// Widget for displaying the current vim mode.
pub struct VimStatus {
    /// Label formatter.
    pub label: Label,
}

impl Default for VimStatus {
    fn default() -> Self {
        Self {
            label: Label::default(),
        }
    }
}

impl Widget for VimStatus {
    fn render(&self, input: &StatusLineInput) -> Option<String> {
        let vim = input.vim.as_ref()?;
        let mode_str = match vim.mode {
            claude_code_statusline_model::VimMode::Normal => "NORMAL",
            claude_code_statusline_model::VimMode::Insert => "INSERT",
        };
        Some(self.label.render(mode_str))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use claude_code_statusline_model::*;

    fn make_input(vim: Option<Vim>) -> StatusLineInput {
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
            vim,
            agent: None,
            worktree: None,
            rate_limits: None,
        }
    }

    #[test]
    fn renders_normal() {
        let w = VimStatus::default();
        let input = make_input(Some(Vim { mode: VimMode::Normal }));
        assert_eq!(w.render(&input).unwrap(), "NORMAL");
    }

    #[test]
    fn renders_insert() {
        let w = VimStatus::default();
        let input = make_input(Some(Vim { mode: VimMode::Insert }));
        assert_eq!(w.render(&input).unwrap(), "INSERT");
    }

    #[test]
    fn returns_none_without_vim() {
        let w = VimStatus::default();
        let input = make_input(None);
        assert!(w.render(&input).is_none());
    }
}
