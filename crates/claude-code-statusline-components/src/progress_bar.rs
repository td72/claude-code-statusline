//! Progress bar component for percentage values.
//!
//! Used for: `context_window.used_percentage`, `rate_limits.*.used_percentage`

use crate::color::{color_for_value, Color, Threshold, RESET};

/// Configuration for a progress bar component.
#[derive(Debug, Clone)]
pub struct ProgressBar {
    /// Width of the bar in characters.
    pub width: usize,
    /// Character for filled portion.
    pub filled_char: char,
    /// Character for empty portion.
    pub empty_char: char,
    /// Color thresholds (sorted descending by `min`).
    /// If empty, no color is applied.
    pub thresholds: Vec<Threshold>,
    /// Default color when no threshold matches.
    pub default_color: Color,
    /// Whether to show the percentage label after the bar.
    pub show_label: bool,
    /// Prefix text before the bar.
    pub prefix: String,
    /// Suffix text after the percentage label.
    pub suffix: String,
}

impl Default for ProgressBar {
    fn default() -> Self {
        Self {
            width: 10,
            filled_char: '█',
            empty_char: '░',
            thresholds: vec![
                Threshold { min: 90.0, color: Color::Red },
                Threshold { min: 70.0, color: Color::Yellow },
            ],
            default_color: Color::Green,
            show_label: true,
            prefix: String::new(),
            suffix: String::new(),
        }
    }
}

impl ProgressBar {
    /// Render the progress bar for a given percentage (0.0–100.0).
    pub fn render(&self, percentage: f64) -> String {
        let pct = percentage.clamp(0.0, 100.0);
        let filled = (pct * self.width as f64 / 100.0).round() as usize;
        let empty = self.width.saturating_sub(filled);

        let bar: String = std::iter::repeat(self.filled_char)
            .take(filled)
            .chain(std::iter::repeat(self.empty_char).take(empty))
            .collect();

        let colored_bar = if self.thresholds.is_empty() {
            bar
        } else {
            let color = color_for_value(pct, &self.thresholds, self.default_color);
            format!("{}{bar}{RESET}", color.fg_string())
        };

        let mut out = String::new();
        if !self.prefix.is_empty() {
            out.push_str(&self.prefix);
            out.push(' ');
        }
        out.push_str(&colored_bar);
        if self.show_label {
            out.push_str(&format!(" {}%", pct as u64));
        }
        if !self.suffix.is_empty() {
            out.push(' ');
            out.push_str(&self.suffix);
        }
        out
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_bar_at_50_percent() {
        let bar = ProgressBar::default();
        let result = bar.render(50.0);
        // 5 filled, 5 empty, green color
        assert!(result.contains("█████░░░░░"));
        assert!(result.contains("50%"));
    }

    #[test]
    fn bar_at_0_percent() {
        let bar = ProgressBar { show_label: false, ..Default::default() };
        let result = bar.render(0.0);
        assert!(result.contains("░░░░░░░░░░"));
    }

    #[test]
    fn bar_at_100_percent() {
        let bar = ProgressBar { show_label: false, ..Default::default() };
        let result = bar.render(100.0);
        assert!(result.contains("██████████"));
    }

    #[test]
    fn bar_clamps_out_of_range() {
        let bar = ProgressBar { show_label: true, ..Default::default() };
        let result = bar.render(150.0);
        assert!(result.contains("100%"));
    }

    #[test]
    fn bar_with_prefix_and_suffix() {
        let bar = ProgressBar {
            prefix: "ctx:".to_string(),
            suffix: "used".to_string(),
            thresholds: vec![],
            show_label: true,
            ..Default::default()
        };
        let result = bar.render(25.0);
        assert!(result.starts_with("ctx: "));
        assert!(result.ends_with("25% used"));
    }

    #[test]
    fn bar_no_color_when_thresholds_empty() {
        let bar = ProgressBar {
            thresholds: vec![],
            show_label: false,
            ..Default::default()
        };
        let result = bar.render(50.0);
        // No ANSI escape codes
        assert!(!result.contains('\x1b'));
    }

    #[test]
    fn custom_chars() {
        let bar = ProgressBar {
            filled_char: '#',
            empty_char: '-',
            width: 5,
            thresholds: vec![],
            show_label: false,
            ..Default::default()
        };
        let result = bar.render(60.0);
        assert_eq!(result, "###--");
    }
}
