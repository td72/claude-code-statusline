//! Reasoning effort widget.
//!
//! Displays the session's reasoning effort level (`low`, `medium`, `high`,
//! `xhigh`, `max`), optionally colored per level. Returns `None` when the
//! current model does not support the effort parameter.

use claude_code_statusline_components::color::Color;
use claude_code_statusline_components::label::Label;
use claude_code_statusline_model::StatusLineInput;

use crate::Widget;

/// Widget for displaying the reasoning effort level.
///
/// Returns `None` when `effort` is absent from the input.
#[derive(Debug, Clone)]
pub struct EffortLevel {
    /// Label formatter.
    pub label: Label,
    /// Per-level foreground color overrides, keyed by level name
    /// (e.g. `("xhigh", Color::Yellow)`). Falls back to `label.color`.
    pub colors: Vec<(String, Color)>,
}

impl Default for EffortLevel {
    fn default() -> Self {
        Self {
            label: Label { prefix: "🧠 ".into(), ..Default::default() },
            colors: Vec::new(),
        }
    }
}

impl Widget for EffortLevel {
    fn render(&self, input: &StatusLineInput) -> Option<String> {
        let level = input.effort.as_ref()?.level.as_str();
        let color = self
            .colors
            .iter()
            .find(|(name, _)| name == level)
            .map(|(_, c)| *c);

        match color {
            Some(c) => Label { color: Some(c), ..self.label.clone() }.render(level),
            None => self.label.render(level),
        }
        .into()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use claude_code_statusline_model::Effort;

    fn make_input(level: Option<&str>) -> StatusLineInput {
        StatusLineInput {
            effort: level.map(|l| Effort { level: l.into() }),
            ..Default::default()
        }
    }

    #[test]
    fn renders_level() {
        let w = EffortLevel::default();
        let input = make_input(Some("high"));
        assert_eq!(w.render(&input).unwrap(), "🧠 high");
    }

    #[test]
    fn applies_level_color() {
        let w = EffortLevel {
            colors: vec![("max".into(), Color::Red)],
            ..Default::default()
        };
        let result = w.render(&make_input(Some("max"))).unwrap();
        assert!(result.contains("\x1b[31m"));
        assert!(result.contains("max"));

        // unmatched level stays plain
        assert_eq!(w.render(&make_input(Some("low"))).unwrap(), "🧠 low");
    }

    #[test]
    fn returns_none_without_effort() {
        let w = EffortLevel::default();
        assert!(w.render(&make_input(None)).is_none());
    }
}
