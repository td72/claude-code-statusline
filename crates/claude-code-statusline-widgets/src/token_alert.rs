//! Token threshold alert widget.

use claude_code_statusline_components::indicator::Indicator;
use claude_code_statusline_model::StatusLineInput;

use crate::Widget;

/// Widget for displaying the 200k token threshold alert.
pub struct TokenAlert {
    /// Indicator formatter.
    pub indicator: Indicator,
}

impl Default for TokenAlert {
    fn default() -> Self {
        Self {
            indicator: Indicator::default(),
        }
    }
}

impl Widget for TokenAlert {
    fn render(&self, input: &StatusLineInput) -> Option<String> {
        let result = self.indicator.render(input.exceeds_200k_tokens);
        if result.is_empty() {
            None
        } else {
            Some(result)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use claude_code_statusline_model::*;

    fn make_input(exceeds: bool) -> StatusLineInput {
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
            exceeds_200k_tokens: exceeds,
            vim: None,
            agent: None,
            worktree: None,
            rate_limits: None,
        }
    }

    #[test]
    fn shows_alert_when_exceeded() {
        let w = TokenAlert::default();
        let input = make_input(true);
        let result = w.render(&input).unwrap();
        assert!(result.contains("⚠"));
    }

    #[test]
    fn returns_none_when_not_exceeded() {
        let w = TokenAlert::default();
        let input = make_input(false);
        assert!(w.render(&input).is_none());
    }
}
