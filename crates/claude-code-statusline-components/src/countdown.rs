//! Countdown component for Unix timestamp values.
//!
//! Computes the difference between a target timestamp and "now", and renders
//! the remaining time in a human-readable form such as `"2h 30m"` or `"3d 5h"`.
//!
//! Typical data source: `rate_limits.*.resets_at`.

/// Configuration for countdown formatting.
///
/// # Examples
///
/// ```
/// use claude_code_statusline_components::countdown::Countdown;
///
/// let c = Countdown::default();
/// // 2 hours remaining
/// assert_eq!(c.render(1000, 1000 + 7200), "2h 0m");
/// ```
#[derive(Debug, Clone)]
pub struct Countdown {
    /// Text to show when the timestamp is in the past.
    pub expired_text: String,
}

impl Default for Countdown {
    fn default() -> Self {
        Self {
            expired_text: "now".to_string(),
        }
    }
}

impl Countdown {
    /// Render the remaining time until a target timestamp.
    ///
    /// `now_epoch_secs` and `target_epoch_secs` are both Unix timestamps in seconds.
    ///
    /// Returns a human-readable remaining time like `"2h 30m"` or `"3d 5h"`.
    pub fn render(&self, now_epoch_secs: i64, target_epoch_secs: i64) -> String {
        let remaining = target_epoch_secs - now_epoch_secs;
        if remaining <= 0 {
            return self.expired_text.clone();
        }

        let remaining = remaining as u64;
        let days = remaining / 86400;
        let hours = (remaining % 86400) / 3600;
        let mins = (remaining % 3600) / 60;

        if days > 0 {
            format!("{days}d {hours}h")
        } else if hours > 0 {
            format!("{hours}h {mins}m")
        } else {
            format!("{mins}m")
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn expired() {
        assert_eq!(Countdown::default().render(1000, 500), "now");
    }

    #[test]
    fn minutes_only() {
        // 30 minutes remaining
        assert_eq!(Countdown::default().render(0, 1800), "30m");
    }

    #[test]
    fn hours_and_minutes() {
        // 2h 30m remaining
        assert_eq!(Countdown::default().render(0, 9000), "2h 30m");
    }

    #[test]
    fn days_and_hours() {
        // 1d 2h remaining
        let secs = 86400 + 7200;
        assert_eq!(Countdown::default().render(0, secs), "1d 2h");
    }

    #[test]
    fn custom_expired_text() {
        let c = Countdown { expired_text: "reset!".to_string() };
        assert_eq!(c.render(1000, 1000), "reset!");
    }

    #[test]
    fn just_expired() {
        assert_eq!(Countdown::default().render(100, 100), "now");
    }
}
