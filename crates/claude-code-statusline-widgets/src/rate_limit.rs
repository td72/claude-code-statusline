//! Rate limit usage widget.
//!
//! Displays a progress bar for the rate limit percentage and a countdown
//! to the reset time. Returns `None` when the input has no `rate_limits`
//! (i.e., the user is not on Claude.ai) or the selected window is absent.

use claude_code_statusline_components::countdown::Countdown;
use claude_code_statusline_components::progress_bar::ProgressBar;
use claude_code_statusline_model::StatusLineInput;

use crate::Widget;

/// Widget for displaying rate limit usage.
///
/// Returns `None` when `rate_limits` or the selected window is absent
/// from the input.
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
    ///
    /// This is the testable entry point; the [`Widget::render`] implementation
    /// calls this with the current system time.
    pub fn render_with_now(&self, input: &StatusLineInput, now_epoch_secs: i64) -> Option<String> {
        let limits = input.rate_limits.as_ref()?;
        let window = match &self.window {
            RateLimitWindowKind::FiveHour => limits.five_hour.as_ref()?,
            RateLimitWindowKind::SevenDay => limits.seven_day.as_ref()?,
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
            rate_limits: Some(RateLimits {
                five_hour: Some(RateLimitWindow {
                    used_percentage: 42.5,
                    resets_at: 1774029600, // 2026-03-18T06:00:00Z
                }),
                seven_day: Some(RateLimitWindow {
                    used_percentage: 10.2,
                    resets_at: 1774634400, // 2026-03-25T06:00:00Z
                }),
            }),
            ..Default::default()
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
    fn returns_none_when_window_absent() {
        let mut input = make_input_with_limits();
        input.rate_limits.as_mut().unwrap().seven_day = None;

        let five = RateLimit::default();
        assert!(five.render_with_now(&input, 1774029600 - 7200).is_some());

        let seven = RateLimit {
            window: RateLimitWindowKind::SevenDay,
            ..Default::default()
        };
        assert!(seven.render_with_now(&input, 0).is_none());
    }

    #[test]
    fn returns_none_without_rate_limits() {
        let w = RateLimit::default();
        let mut input = make_input_with_limits();
        input.rate_limits = None;
        assert!(w.render_with_now(&input, 0).is_none());
    }
}
