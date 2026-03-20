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

/// A single output line.
#[derive(Debug, Deserialize)]
pub struct LineConfig {
    /// Widget names to render on this line.
    pub widgets: Vec<String>,
    /// Separator between widgets.
    #[serde(default = "default_separator")]
    pub separator: String,
}

fn default_separator() -> String {
    " | ".to_string()
}

/// Configuration for a single widget.
#[derive(Debug, Default, Deserialize)]
pub struct WidgetConfig {
    // Label-style options
    pub color: Option<String>,
    pub bracket: Option<String>,
    pub prefix: Option<String>,

    // Path options
    pub style: Option<String>,
    pub home_dir: Option<String>,

    // ProgressBar options
    pub bar_width: Option<usize>,
    pub filled_char: Option<String>,
    pub empty_char: Option<String>,
    pub show_label: Option<bool>,

    // ContextUsage options
    pub show_tokens: Option<bool>,
    pub token_style: Option<String>,

    // CostSummary options
    pub show_lines: Option<bool>,

    // Indicator options
    pub on_text: Option<String>,
    pub off_text: Option<String>,
    pub on_color: Option<String>,

    // RateLimit options
    pub window: Option<String>,

    // Thresholds
    pub thresholds: Option<ThresholdConfig>,
}

/// Color thresholds for progress bars.
#[derive(Debug, Default, Deserialize)]
pub struct ThresholdConfig {
    pub red: Option<f64>,
    pub yellow: Option<f64>,
}

// --- Conversion helpers ---

pub fn parse_color(s: &str) -> Color {
    match s {
        "red" => Color::Red,
        "green" => Color::Green,
        "yellow" => Color::Yellow,
        "blue" => Color::Blue,
        "cyan" => Color::Cyan,
        "magenta" => Color::Magenta,
        "white" => Color::White,
        _ => Color::White,
    }
}

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
