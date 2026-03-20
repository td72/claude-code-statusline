//! Cost and duration summary widget.

use claude_code_statusline_components::count::Count;
use claude_code_statusline_components::currency::Currency;
use claude_code_statusline_components::duration::Duration;
use claude_code_statusline_model::StatusLineInput;

use crate::Widget;

/// Widget for displaying session cost, duration, and lines changed.
pub struct CostSummary {
    /// Currency formatter for cost.
    pub currency: Currency,
    /// Duration formatter for elapsed time.
    pub duration: Duration,
    /// Line count formatter. If `Some`, appends lines added/removed.
    pub line_count: Option<Count>,
    /// Separator between parts.
    pub separator: String,
    /// Prefix for cost (e.g., "💰 ").
    pub cost_prefix: String,
    /// Prefix for duration (e.g., "⏱ ").
    pub duration_prefix: String,
    /// Prefix for lines changed (e.g., "📝 ").
    pub lines_prefix: String,
}

impl Default for CostSummary {
    fn default() -> Self {
        Self {
            currency: Currency::default(),
            duration: Duration::default(),
            line_count: None,
            separator: " | ".to_string(),
            cost_prefix: String::new(),
            duration_prefix: String::new(),
            lines_prefix: String::new(),
        }
    }
}

impl Widget for CostSummary {
    fn render(&self, input: &StatusLineInput) -> Option<String> {
        let cost = &input.cost;
        let mut parts = vec![
            format!("{}{}", self.cost_prefix, self.currency.render(cost.total_cost_usd)),
            format!("{}{}", self.duration_prefix, self.duration.render(cost.total_duration_ms)),
        ];

        if let Some(count) = &self.line_count {
            let added = Count { prefix: "+".into(), ..count.clone() }.render(cost.total_lines_added);
            let removed = Count { prefix: "-".into(), ..count.clone() }.render(cost.total_lines_removed);
            parts.push(format!("{}{added} {removed}", self.lines_prefix));
        }

        Some(parts.join(&self.separator))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use claude_code_statusline_model::*;

    fn make_input(cost_usd: f64, duration_ms: u64, added: u64, removed: u64) -> StatusLineInput {
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
                total_cost_usd: cost_usd,
                total_duration_ms: duration_ms,
                total_api_duration_ms: 0,
                total_lines_added: added,
                total_lines_removed: removed,
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
    fn renders_cost_and_duration() {
        let w = CostSummary::default();
        let input = make_input(0.05, 125_000, 0, 0);
        let result = w.render(&input).unwrap();
        assert!(result.contains("$0.05"));
        assert!(result.contains("2m 5s"));
    }

    #[test]
    fn renders_with_lines() {
        let w = CostSummary {
            line_count: Some(Count::default()),
            ..Default::default()
        };
        let input = make_input(1.23, 60_000, 156, 23);
        let result = w.render(&input).unwrap();
        assert!(result.contains("$1.23"));
        assert!(result.contains("+156"));
        assert!(result.contains("-23"));
    }
}
