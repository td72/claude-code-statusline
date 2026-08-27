//! Model information widget.
//!
//! Displays the current model's display name, optionally stripping
//! parenthesized suffixes (e.g., `"(1M context)"`) for a shorter label.

use claude_code_statusline_components::label::Label;
use claude_code_statusline_model::StatusLineInput;

use crate::Widget;

/// Widget for displaying the current model name.
///
/// Always returns `Some` because the model field is always present.
pub struct ModelInfo {
    /// Label formatter.
    pub label: Label,
    /// Remove parenthesized suffixes like "(1M context)" from display name.
    pub short: bool,
}

impl Default for ModelInfo {
    fn default() -> Self {
        Self {
            label: Label::default(),
            short: false,
        }
    }
}

impl Widget for ModelInfo {
    fn render(&self, input: &StatusLineInput) -> Option<String> {
        let name = if self.short {
            input.model.display_name
                .split('(')
                .next()
                .unwrap_or(&input.model.display_name)
                .trim()
        } else {
            &input.model.display_name
        };
        Some(self.label.render(name))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use claude_code_statusline_components::label::BracketStyle;
    use claude_code_statusline_model::*;

    fn make_input(model_name: &str) -> StatusLineInput {
        StatusLineInput {
            model: Model { id: "claude-opus-5".into(), display_name: model_name.into() },
            ..Default::default()
        }
    }

    #[test]
    fn renders_model_name() {
        let w = ModelInfo::default();
        let input = make_input("Opus");
        assert_eq!(w.render(&input).unwrap(), "Opus");
    }

    #[test]
    fn renders_bracketed() {
        let w = ModelInfo {
            label: Label { bracket: Some(BracketStyle::Square), ..Default::default() },
            short: false,
        };
        let input = make_input("Sonnet");
        assert_eq!(w.render(&input).unwrap(), "[Sonnet]");
    }

    #[test]
    fn renders_short() {
        let w = ModelInfo { short: true, ..Default::default() };
        let input = make_input("Opus 5 (1M context)");
        assert_eq!(w.render(&input).unwrap(), "Opus 5");
    }
}
