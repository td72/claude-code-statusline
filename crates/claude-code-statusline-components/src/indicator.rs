//! Indicator component for boolean flag values.
//!
//! Renders a boolean as a configurable on/off text or icon, optionally
//! colored. When the `off_text` is empty and the value is `false`, the
//! indicator produces an empty string so it can be hidden from output.
//!
//! Typical data source: `exceeds_200k_tokens`.

use crate::color::{Color, RESET};

/// Configuration for indicator formatting.
///
/// # Examples
///
/// ```
/// use claude_code_statusline_components::indicator::Indicator;
///
/// let ind = Indicator::default();
/// assert!(!ind.render(true).is_empty());
/// assert!(ind.render(false).is_empty());
/// ```
#[derive(Debug, Clone)]
pub struct Indicator {
    /// Text/icon to show when `true`.
    pub on_text: String,
    /// Text/icon to show when `false`. If empty, nothing is rendered.
    pub off_text: String,
    /// Color when `true`.
    pub on_color: Option<Color>,
    /// Color when `false`.
    pub off_color: Option<Color>,
}

impl Default for Indicator {
    fn default() -> Self {
        Self {
            on_text: "⚠".to_string(),
            off_text: String::new(),
            on_color: Some(Color::Red),
            off_color: None,
        }
    }
}

impl Indicator {
    /// Render the indicator for a boolean value.
    ///
    /// Returns an empty string if the value is `false` and `off_text` is empty.
    pub fn render(&self, value: bool) -> String {
        let (text, color) = if value {
            (&self.on_text, &self.on_color)
        } else {
            (&self.off_text, &self.off_color)
        };

        if text.is_empty() {
            return String::new();
        }

        match color {
            Some(c) => format!("{}{text}{RESET}", c.fg_string()),
            None => text.clone(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_on() {
        let i = Indicator::default();
        let result = i.render(true);
        assert!(result.contains("⚠"));
        assert!(result.contains("\x1b[31m")); // red
    }

    #[test]
    fn default_off_is_empty() {
        let i = Indicator::default();
        assert_eq!(i.render(false), "");
    }

    #[test]
    fn custom_on_off() {
        let i = Indicator {
            on_text: "YES".to_string(),
            off_text: "NO".to_string(),
            on_color: None,
            off_color: None,
        };
        assert_eq!(i.render(true), "YES");
        assert_eq!(i.render(false), "NO");
    }

    #[test]
    fn colored_off() {
        let i = Indicator {
            on_text: "●".to_string(),
            off_text: "○".to_string(),
            on_color: Some(Color::Green),
            off_color: Some(Color::White),
        };
        assert!(i.render(true).contains("\x1b[32m"));
        assert!(i.render(false).contains("\x1b[37m"));
    }
}
