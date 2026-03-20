//! ANSI color utilities for terminal output.

/// ANSI color codes.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Color {
    Red,
    Green,
    Yellow,
    Blue,
    Cyan,
    Magenta,
    White,
    /// A custom ANSI 256-color code.
    Ansi256(u8),
}

impl Color {
    /// Returns the ANSI escape sequence to set this foreground color.
    pub fn fg(self) -> &'static str {
        match self {
            Color::Red => "\x1b[31m",
            Color::Green => "\x1b[32m",
            Color::Yellow => "\x1b[33m",
            Color::Blue => "\x1b[34m",
            Color::Magenta => "\x1b[35m",
            Color::Cyan => "\x1b[36m",
            Color::White => "\x1b[37m",
            Color::Ansi256(_) => "", // handled by fg_string
        }
    }

    /// Returns the ANSI escape sequence as an owned string.
    /// Required for `Ansi256` which needs dynamic formatting.
    pub fn fg_string(self) -> String {
        match self {
            Color::Ansi256(n) => format!("\x1b[38;5;{n}m"),
            other => other.fg().to_string(),
        }
    }
}

/// ANSI reset sequence.
pub const RESET: &str = "\x1b[0m";

/// Wrap text with a foreground color and reset.
pub fn colored(text: &str, color: Color) -> String {
    match color {
        Color::Ansi256(_) => format!("{}{text}{RESET}", color.fg_string()),
        _ => format!("{}{text}{RESET}", color.fg()),
    }
}

/// A threshold that maps a value range to a color.
#[derive(Debug, Clone)]
pub struct Threshold {
    /// Minimum value (inclusive) for this threshold.
    pub min: f64,
    /// Color to use when value >= min.
    pub color: Color,
}

/// Select a color based on thresholds.
/// Thresholds should be sorted in descending order of `min`.
/// Returns the color of the first threshold where `value >= min`,
/// or `default` if none match.
pub fn color_for_value(value: f64, thresholds: &[Threshold], default: Color) -> Color {
    for t in thresholds {
        if value >= t.min {
            return t.color;
        }
    }
    default
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn colored_wraps_text() {
        let result = colored("hello", Color::Green);
        assert_eq!(result, "\x1b[32mhello\x1b[0m");
    }

    #[test]
    fn color_for_value_selects_threshold() {
        let thresholds = vec![
            Threshold { min: 90.0, color: Color::Red },
            Threshold { min: 70.0, color: Color::Yellow },
        ];
        assert_eq!(color_for_value(95.0, &thresholds, Color::Green), Color::Red);
        assert_eq!(color_for_value(80.0, &thresholds, Color::Green), Color::Yellow);
        assert_eq!(color_for_value(50.0, &thresholds, Color::Green), Color::Green);
    }

    #[test]
    fn ansi256_fg_string() {
        let c = Color::Ansi256(208);
        assert_eq!(c.fg_string(), "\x1b[38;5;208m");
    }
}
