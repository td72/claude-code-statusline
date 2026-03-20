//! Context window usage widget.
//!
//! Renders context window usage as a progress bar with optional token counts.

use claude_code_statusline_components::count::Count;
use claude_code_statusline_components::progress_bar::ProgressBar;
use claude_code_statusline_model::StatusLineInput;

use crate::Widget;

/// Widget for displaying context window usage.
pub struct ContextUsage {
    /// Progress bar for the usage percentage.
    pub bar: ProgressBar,
    /// Token count formatter. If `Some`, appends token usage after the bar.
    pub token_count: Option<Count>,
}

impl Default for ContextUsage {
    fn default() -> Self {
        Self {
            bar: ProgressBar::default(),
            token_count: None,
        }
    }
}

impl Widget for ContextUsage {
    fn render(&self, input: &StatusLineInput) -> Option<String> {
        let pct = input.context_window.used_percentage.unwrap_or(0.0);
        let mut out = self.bar.render(pct);

        if let Some(count) = &self.token_count {
            if let Some(usage) = &input.context_window.current_usage {
                let used = usage.input_tokens
                    + usage.cache_creation_input_tokens
                    + usage.cache_read_input_tokens;
                let total = input.context_window.context_window_size;
                out.push_str(&format!(
                    " ({}/{})",
                    count.render(used),
                    count.render(total),
                ));
            }
        }

        Some(out)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use claude_code_statusline_components::count::CountStyle;
    use claude_code_statusline_model::*;

    fn make_input(pct: Option<f64>, usage: Option<CurrentUsage>) -> StatusLineInput {
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
                used_percentage: pct,
                remaining_percentage: pct.map(|p| 100.0 - p),
                current_usage: usage,
            },
            exceeds_200k_tokens: false,
            vim: None,
            agent: None,
            worktree: None,
            rate_limits: None,
        }
    }

    #[test]
    fn renders_bar() {
        let w = ContextUsage::default();
        let input = make_input(Some(50.0), None);
        let result = w.render(&input).unwrap();
        assert!(result.contains("50%"));
    }

    #[test]
    fn renders_with_token_count() {
        let w = ContextUsage {
            bar: ProgressBar { show_label: true, ..Default::default() },
            token_count: Some(Count { style: CountStyle::Compact, ..Default::default() }),
        };
        let input = make_input(
            Some(10.0),
            Some(CurrentUsage {
                input_tokens: 15_000,
                output_tokens: 5_000,
                cache_creation_input_tokens: 3_000,
                cache_read_input_tokens: 2_000,
            }),
        );
        let result = w.render(&input).unwrap();
        assert!(result.contains("10%"));
        // 15000 + 3000 + 2000 = 20000 → "20.0k"
        assert!(result.contains("20.0k"));
        assert!(result.contains("200k"));
    }

    #[test]
    fn null_percentage_defaults_to_zero() {
        let w = ContextUsage::default();
        let input = make_input(None, None);
        let result = w.render(&input).unwrap();
        assert!(result.contains("0%"));
    }
}
