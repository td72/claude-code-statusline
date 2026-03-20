//! ANSI color utilities for terminal output.
//!
//! Provides a [`Color`] enum for the standard 8 terminal colors plus ANSI-256,
//! along with helper functions for wrapping text in foreground/background
//! escape sequences and selecting colors based on numeric thresholds.

/// ANSI color codes.
///
/// Covers the 8 standard terminal colors and the extended 256-color palette.
///
/// # Examples
///
/// ```
/// use claude_code_statusline_components::color::{Color, colored};
///
/// let text = colored("OK", Color::Green);
/// assert!(text.starts_with("\x1b[32m"));
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Color {
    /// ANSI black (code 30/40).
    Black,
    /// ANSI red (code 31/41).
    Red,
    /// ANSI green (code 32/42).
    Green,
    /// ANSI yellow (code 33/43).
    Yellow,
    /// ANSI blue (code 34/44).
    Blue,
    /// ANSI cyan (code 36/46).
    Cyan,
    /// ANSI magenta (code 35/45).
    Magenta,
    /// ANSI white (code 37/47).
    White,
    /// A custom ANSI 256-color code (0-255).
    Ansi256(u8),
}

impl Color {
    /// Returns the ANSI escape sequence to set this foreground color.
    ///
    /// For [`Color::Ansi256`] this returns an empty string; use
    /// [`fg_string`](Self::fg_string) instead.
    pub fn fg(self) -> &'static str {
        match self {
            Color::Black => "\x1b[30m",
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

    /// Returns the ANSI escape sequence to set this background color.
    ///
    /// For [`Color::Ansi256`] this returns an empty string; use
    /// [`bg_string`](Self::bg_string) instead.
    pub fn bg(self) -> &'static str {
        match self {
            Color::Black => "\x1b[40m",
            Color::Red => "\x1b[41m",
            Color::Green => "\x1b[42m",
            Color::Yellow => "\x1b[43m",
            Color::Blue => "\x1b[44m",
            Color::Magenta => "\x1b[45m",
            Color::Cyan => "\x1b[46m",
            Color::White => "\x1b[47m",
            Color::Ansi256(_) => "", // handled by bg_string
        }
    }

    /// Returns the background ANSI escape sequence as an owned string.
    pub fn bg_string(self) -> String {
        match self {
            Color::Ansi256(n) => format!("\x1b[48;5;{n}m"),
            other => other.bg().to_string(),
        }
    }
}

/// ANSI reset sequence.
pub const RESET: &str = "\x1b[0m";

/// Wrap text with a foreground color and append an ANSI reset.
///
/// This is a convenience function that handles both standard colors and
/// [`Color::Ansi256`] variants automatically.
///
/// # Examples
///
/// ```
/// use claude_code_statusline_components::color::{Color, colored};
///
/// assert_eq!(colored("hi", Color::Red), "\x1b[31mhi\x1b[0m");
/// ```
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
///
/// Thresholds **must** be sorted in descending order of `min`.
/// Returns the color of the first threshold where `value >= min`,
/// or `default` if none match.
///
/// # Examples
///
/// ```
/// use claude_code_statusline_components::color::{Color, Threshold, color_for_value};
///
/// let thresholds = vec![
///     Threshold { min: 90.0, color: Color::Red },
///     Threshold { min: 70.0, color: Color::Yellow },
/// ];
/// assert_eq!(color_for_value(95.0, &thresholds, Color::Green), Color::Red);
/// assert_eq!(color_for_value(50.0, &thresholds, Color::Green), Color::Green);
/// ```
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
