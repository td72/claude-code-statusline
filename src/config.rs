//! TOML configuration types and loading logic.
//!
//! The configuration file defines which widgets appear on each output line,
//! along with per-widget styling overrides (colors, brackets, bar width,
//! thresholds, etc.).
//!
//! Configuration is loaded in the following priority order:
//!
//! 1. Explicit `--config <path>` CLI argument.
//! 2. `~/.config/claude-code-statusline/config.toml`.
//! 3. Built-in default (`config.default.toml`).

use std::collections::HashMap;

use serde::Deserialize;

use claude_code_statusline_components::color::Color;
use claude_code_statusline_components::label::BracketStyle;

/// Top-level configuration.
#[derive(Debug, Deserialize)]
pub struct Config {
    /// Lines to render. Each line contains a list of widget names.
    pub line: Vec<LineConfig>,
    /// Per-widget configuration, keyed by widget name.
    #[serde(default)]
    pub widget: HashMap<String, WidgetConfig>,
}

/// A single output line definition.
///
/// Each line contains one or more widgets joined by a separator. A line can
/// be capped to a visible width: `max_width` is an absolute column count,
/// `reserve_right` subtracts from the terminal width (`COLUMNS`) so the
/// line leaves room for Claude Code's right-aligned notifications. When
/// both are set the smaller limit wins; with neither set the line is never
/// truncated.
#[derive(Debug, Deserialize)]
pub struct LineConfig {
    /// Widget names to render on this line.
    pub widgets: Vec<String>,
    /// Separator between widgets.
    #[serde(default = "default_separator")]
    pub separator: String,
    /// Absolute maximum visible width in columns.
    pub max_width: Option<usize>,
    /// Columns to keep free at the right edge of the terminal.
    pub reserve_right: Option<usize>,
    /// Text appended when the line is truncated.
    #[serde(default = "default_ellipsis")]
    pub ellipsis: String,
}

impl LineConfig {
    /// Effective width limit for this line given the terminal width.
    pub fn width_limit(&self, columns: Option<usize>) -> Option<usize> {
        let from_terminal = match (columns, self.reserve_right) {
            (Some(cols), Some(reserve)) => Some(cols.saturating_sub(reserve)),
            _ => None,
        };
        match (self.max_width, from_terminal) {
            (Some(a), Some(b)) => Some(a.min(b)),
            (a, b) => a.or(b),
        }
    }
}

fn default_separator() -> String {
    " | ".to_string()
}

fn default_ellipsis() -> String {
    "…".to_string()
}

/// Configuration for a single widget.
///
/// This is a flat bag of optional fields; each widget reads only the
/// fields it cares about and ignores the rest. Unknown fields are
/// silently ignored by serde.
#[derive(Debug, Default, Deserialize)]
pub struct WidgetConfig {
    // -- Label-style options --

    /// Foreground color name (e.g., `"cyan"`, `"red"`, or a `0`-`255` ANSI code).
    pub color: Option<String>,
    /// Background color name.
    pub bg: Option<String>,
    /// Bracket style: `"square"`, `"round"`, `"angle"`, or `"none"`.
    pub bracket: Option<String>,
    /// Prefix text/icon prepended to the widget output.
    pub prefix: Option<String>,

    // -- ModelInfo options --

    /// When `true`, strip parenthesized suffixes from the model display name.
    pub short: Option<bool>,

    // -- Path / Repo options --

    /// Display style. For `workspace`: `"full"`, `"home_shortened"`, or
    /// `"basename"` (default). For `repo`: `"name"`, `"owner_name"`
    /// (default), or `"full"`.
    pub style: Option<String>,
    /// Explicit home directory for `home_shortened` style.
    pub home_dir: Option<String>,
    /// Path style to switch to when the terminal is narrower than
    /// `narrow_below` columns (`workspace` only).
    pub narrow_style: Option<String>,
    /// Column threshold for `narrow_style`.
    pub narrow_below: Option<usize>,

    // -- ProgressBar options --

    /// Width of the progress bar in characters.
    pub bar_width: Option<usize>,
    /// Single character used for the filled portion of the bar.
    pub filled_char: Option<String>,
    /// Single character used for the empty portion of the bar.
    pub empty_char: Option<String>,
    /// Whether to display the percentage label after the bar.
    pub show_label: Option<bool>,

    // -- ContextUsage options --

    /// When `true`, append token counts after the progress bar.
    pub show_tokens: Option<bool>,
    /// Token count style: `"plain"`, `"comma"`, or `"compact"`.
    pub token_style: Option<String>,

    // -- CostSummary options --

    /// When `true`, include lines added/removed in the cost summary.
    pub show_lines: Option<bool>,
    /// Prefix for the cost portion (e.g., an emoji).
    pub cost_prefix: Option<String>,
    /// Prefix for the duration portion.
    pub duration_prefix: Option<String>,
    /// Prefix for the lines-changed portion.
    pub lines_prefix: Option<String>,

    // -- Indicator options --

    /// Text/icon shown when the indicator is `true`.
    pub on_text: Option<String>,
    /// Text/icon shown when the indicator is `false`.
    pub off_text: Option<String>,
    /// Color name for the `true` state.
    pub on_color: Option<String>,

    // -- VimStatus options --

    /// Background color for NORMAL mode.
    pub normal_bg: Option<String>,
    /// Background color for INSERT mode.
    pub insert_bg: Option<String>,
    /// Background color for VISUAL / VISUAL LINE modes.
    pub visual_bg: Option<String>,
    /// Foreground color for NORMAL mode.
    pub normal_fg: Option<String>,
    /// Foreground color for INSERT mode.
    pub insert_fg: Option<String>,
    /// Foreground color for VISUAL / VISUAL LINE modes.
    pub visual_fg: Option<String>,

    // -- RateLimit options --

    /// Rate limit window: `"5h"` or `"7d"`.
    pub window: Option<String>,
    /// Separator text between the bar and the countdown (e.g., `" resets in "`).
    pub reset_separator: Option<String>,

    // -- Session options --

    /// Maximum characters for the session name (`0` disables truncation).
    pub max_len: Option<usize>,

    // -- Worktree options --

    /// Worktree source: `"any"` (default; worktree sessions and plain
    /// `git worktree add` worktrees) or `"session"` (worktree sessions only).
    pub source: Option<String>,

    // -- Link options --

    /// Emit OSC 8 hyperlinks (default `true`). Set to `false` on terminals
    /// without hyperlink support.
    pub link: Option<bool>,

    // -- Value-keyed colors --

    /// Foreground color per value, e.g. `{ xhigh = "yellow", max = "red" }`
    /// for the `effort` widget. Overrides `color` when the value matches.
    pub colors: Option<HashMap<String, String>>,

    // -- Thresholds --

    /// Color thresholds for progress bar coloring.
    pub thresholds: Option<ThresholdConfig>,
}

/// Color thresholds for progress bars.
///
/// Defines the minimum percentage at which the bar turns red or yellow.
/// Values should satisfy `red > yellow` for correct behavior.
#[derive(Debug, Default, Deserialize)]
pub struct ThresholdConfig {
    /// Percentage at which the bar turns red.
    pub red: Option<f64>,
    /// Percentage at which the bar turns yellow.
    pub yellow: Option<f64>,
}

// --- Conversion helpers ---

/// Parse a color name string into a [`Color`].
///
/// Accepts `"black"`, `"red"`, `"green"`, `"yellow"`, `"blue"`, `"cyan"`,
/// `"magenta"`, `"white"`, or a numeric string (`"0"`--`"255"`) for
/// ANSI-256 colors. Unrecognized strings fall back to [`Color::White`].
pub fn parse_color(s: &str) -> Color {
    match s {
        "black" => Color::Black,
        "red" => Color::Red,
        "green" => Color::Green,
        "yellow" => Color::Yellow,
        "blue" => Color::Blue,
        "cyan" => Color::Cyan,
        "magenta" => Color::Magenta,
        "white" => Color::White,
        n => n.parse::<u8>().map(Color::Ansi256).unwrap_or(Color::White),
    }
}

/// Parse a bracket style name into a [`BracketStyle`].
///
/// Accepts `"square"`, `"round"`, `"angle"`, or `"none"`.
/// Unrecognized strings return `None`.
pub fn parse_bracket(s: &str) -> Option<BracketStyle> {
    match s {
        "square" => Some(BracketStyle::Square),
        "round" => Some(BracketStyle::Round),
        "angle" => Some(BracketStyle::Angle),
        "none" => None,
        _ => None,
    }
}

impl Config {
    /// Load a configuration from the given path, the default user config
    /// location, or the built-in default, in that priority order.
    ///
    /// Silently falls back to the next source on any I/O or parse error.
    pub fn load_or_default(path: Option<&str>) -> Config {
        if let Some(p) = path {
            if let Ok(content) = std::fs::read_to_string(p) {
                if let Ok(config) = toml::from_str(&content) {
                    return config;
                }
            }
        }

        // Try ~/.config/claude-code-statusline/config.toml
        if let Some(home) = std::env::var("HOME").ok() {
            let default_path = format!("{home}/.config/claude-code-statusline/config.toml");
            if let Ok(content) = std::fs::read_to_string(&default_path) {
                if let Ok(config) = toml::from_str(&content) {
                    return config;
                }
            }
        }

        // Built-in default
        toml::from_str(include_str!("../config.default.toml")).expect("default config is valid")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn line(max_width: Option<usize>, reserve_right: Option<usize>) -> LineConfig {
        LineConfig {
            widgets: vec![],
            separator: String::new(),
            max_width,
            reserve_right,
            ellipsis: default_ellipsis(),
        }
    }

    #[test]
    fn width_limit_combines_sources() {
        assert_eq!(line(None, None).width_limit(Some(100)), None);
        assert_eq!(line(Some(40), None).width_limit(None), Some(40));
        assert_eq!(line(None, Some(20)).width_limit(Some(100)), Some(80));
        assert_eq!(line(None, Some(20)).width_limit(None), None);
        assert_eq!(line(Some(40), Some(20)).width_limit(Some(100)), Some(40));
        assert_eq!(line(Some(90), Some(20)).width_limit(Some(100)), Some(80));
    }

    #[test]
    fn line_config_defaults() {
        let cfg: LineConfig = toml::from_str(r#"widgets = ["model"]"#).unwrap();
        assert_eq!(cfg.separator, " | ");
        assert_eq!(cfg.ellipsis, "…");
        assert!(cfg.width_limit(Some(80)).is_none());
    }

    #[test]
    fn default_config_parses() {
        let cfg = Config::load_or_default(Some("/nonexistent/path.toml"));
        assert!(!cfg.line.is_empty());
    }
}
