//! Duration component for millisecond values.
//!
//! Used for: `cost.total_duration_ms`, `cost.total_api_duration_ms`

/// Configuration for duration formatting.
#[derive(Debug, Clone)]
pub struct Duration {
    /// Whether to show seconds when duration >= 1 hour.
    pub show_seconds_in_hours: bool,
}

impl Default for Duration {
    fn default() -> Self {
        Self {
            show_seconds_in_hours: false,
        }
    }
}

impl Duration {
    /// Render a duration from milliseconds to a human-readable format.
    ///
    /// - Under 1 second: `"0s"`
    /// - Under 1 minute: `"Xs"`
    /// - Under 1 hour: `"Xm Ys"`
    /// - 1 hour or more: `"Xh Ym"` (or `"Xh Ym Zs"` if `show_seconds_in_hours`)
    pub fn render(&self, ms: u64) -> String {
        let total_secs = ms / 1000;
        let hours = total_secs / 3600;
        let mins = (total_secs % 3600) / 60;
        let secs = total_secs % 60;

        if hours > 0 {
            if self.show_seconds_in_hours {
                format!("{hours}h {mins}m {secs}s")
            } else {
                format!("{hours}h {mins}m")
            }
        } else if mins > 0 {
            format!("{mins}m {secs}s")
        } else {
            format!("{secs}s")
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn zero() {
        assert_eq!(Duration::default().render(0), "0s");
    }

    #[test]
    fn seconds_only() {
        assert_eq!(Duration::default().render(45_000), "45s");
    }

    #[test]
    fn minutes_and_seconds() {
        assert_eq!(Duration::default().render(125_000), "2m 5s");
    }

    #[test]
    fn hours_default() {
        assert_eq!(Duration::default().render(3_661_000), "1h 1m");
    }

    #[test]
    fn hours_with_seconds() {
        let d = Duration { show_seconds_in_hours: true };
        assert_eq!(d.render(3_661_000), "1h 1m 1s");
    }

    #[test]
    fn sub_second() {
        assert_eq!(Duration::default().render(500), "0s");
    }
}
