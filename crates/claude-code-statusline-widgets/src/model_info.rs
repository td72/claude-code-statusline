//! Model information widget.

use claude_code_statusline_components::label::Label;
use claude_code_statusline_model::StatusLineInput;

use crate::Widget;

/// Widget for displaying the current model name.
pub struct ModelInfo {
    /// Label formatter.
    pub label: Label,
}

impl Default for ModelInfo {
    fn default() -> Self {
        Self {
            label: Label::default(),
        }
    }
}

impl Widget for ModelInfo {
    fn render(&self, input: &StatusLineInput) -> Option<String> {
        Some(self.label.render(&input.model.display_name))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use claude_code_statusline_components::label::BracketStyle;
    use claude_code_statusline_model::*;

    fn make_input(model_name: &str) -> StatusLineInput {
        StatusLineInput {
            cwd: "/test".into(),
            session_id: "s".into(),
            transcript_path: "/t".into(),
            model: Model { id: "claude-opus-4-6".into(), display_name: model_name.into() },
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
            worktree: None,
            rate_limits: None,
        }
    }

    #[test]
    fn renders_model_name() {
        let w = ModelInfo::default();
        let input = make_input("Opus");
        assert_eq!(w.render(&input).unwrap(), "Opus");
    }

    #[test]
    fn renders_bracketed() {
        let w = ModelInfo {
            label: Label { bracket: Some(BracketStyle::Square), ..Default::default() },
        };
        let input = make_input("Sonnet");
        assert_eq!(w.render(&input).unwrap(), "[Sonnet]");
    }
}
