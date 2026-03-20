//! Rate limit usage widget.

use claude_code_statusline_components::countdown::Countdown;
use claude_code_statusline_components::progress_bar::ProgressBar;
use claude_code_statusline_model::StatusLineInput;

use crate::Widget;

/// Widget for displaying rate limit usage.
pub struct RateLimit {
    /// Progress bar for the usage percentage.
    pub bar: ProgressBar,
    /// Countdown for the reset time.
    pub countdown: Countdown,
    /// Which window to display.
    pub window: RateLimitWindowKind,
    /// Separator between bar and countdown.
    pub separator: String,
}

/// Which rate limit window to display.
#[derive(Debug, Clone, Default)]
pub enum RateLimitWindowKind {
    /// 5-hour window.
    #[default]
    FiveHour,
    /// 7-day window.
    SevenDay,
}

impl Default for RateLimit {
    fn default() -> Self {
        Self {
            bar: ProgressBar::default(),
            countdown: Countdown::default(),
            window: RateLimitWindowKind::default(),
            separator: " resets in ".to_string(),
        }
    }
}

impl RateLimit {
    /// Render with an explicit `now` timestamp (Unix seconds).
    pub fn render_with_now(&self, input: &StatusLineInput, now_epoch_secs: i64) -> Option<String> {
        let limits = input.rate_limits.as_ref()?;
        let window = match &self.window {
            RateLimitWindowKind::FiveHour => &limits.five_hour,
            RateLimitWindowKind::SevenDay => &limits.seven_day,
        };

        let bar = self.bar.render(window.used_percentage);
        let remaining = self.countdown.render(now_epoch_secs, window.resets_at);

        Some(format!("{bar}{}{remaining}", self.separator))
    }
}

impl Widget for RateLimit {
    fn render(&self, input: &StatusLineInput) -> Option<String> {
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .ok()?
            .as_secs() as i64;
        self.render_with_now(input, now)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use claude_code_statusline_model::*;

    fn make_input_with_limits() -> StatusLineInput {
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
            worktree: None,
            rate_limits: Some(RateLimits {
                five_hour: RateLimitWindow {
                    used_percentage: 42.5,
                    resets_at: 1774029600, // 2026-03-18T06:00:00Z
                },
                seven_day: RateLimitWindow {
                    used_percentage: 10.2,
                    resets_at: 1774634400, // 2026-03-25T06:00:00Z
                },
            }),
        }
    }

    #[test]
    fn renders_five_hour() {
        let w = RateLimit::default();
        // 2h before reset
        let now = 1774029600 - 7200;
        let input = make_input_with_limits();
        let result = w.render_with_now(&input, now).unwrap();
        assert!(result.contains("42%"));
        assert!(result.contains("2h 0m"));
    }

    #[test]
    fn renders_seven_day() {
        let w = RateLimit {
            window: RateLimitWindowKind::SevenDay,
            ..Default::default()
        };
        // 7 days before reset
        let now = 1774634400 - 7 * 86400;
        let input = make_input_with_limits();
        let result = w.render_with_now(&input, now).unwrap();
        assert!(result.contains("10%"));
        assert!(result.contains("7d 0h"));
    }

    #[test]
    fn returns_none_without_rate_limits() {
        let w = RateLimit::default();
        let mut input = make_input_with_limits();
        input.rate_limits = None;
        assert!(w.render_with_now(&input, 0).is_none());
    }
}
