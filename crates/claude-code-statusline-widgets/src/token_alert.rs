//! Token threshold alert widget.
//!
//! Displays a warning indicator when the total token count from the most
//! recent API response exceeds 200k tokens. Returns `None` when the
//! threshold is not exceeded.

use claude_code_statusline_components::indicator::Indicator;
use claude_code_statusline_model::StatusLineInput;

use crate::Widget;

/// Widget for displaying the 200k token threshold alert.
///
/// Returns `None` when `exceeds_200k_tokens` is `false` and the
/// indicator's `off_text` is empty (the default).
pub struct TokenAlert {
    /// Indicator formatter.
    pub indicator: Indicator,
}

impl Default for TokenAlert {
    fn default() -> Self {
        Self {
            indicator: Indicator::default(),
        }
    }
}

impl Widget for TokenAlert {
    fn render(&self, input: &StatusLineInput) -> Option<String> {
        let result = self.indicator.render(input.exceeds_200k_tokens);
        if result.is_empty() {
            None
        } else {
            Some(result)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_input(exceeds: bool) -> StatusLineInput {
        StatusLineInput {
            exceeds_200k_tokens: exceeds,
            ..Default::default()
        }
    }

    #[test]
    fn shows_alert_when_exceeded() {
        let w = TokenAlert::default();
        let input = make_input(true);
        let result = w.render(&input).unwrap();
        assert!(result.contains("⚠"));
    }

    #[test]
    fn returns_none_when_not_exceeded() {
        let w = TokenAlert::default();
        let input = make_input(false);
        assert!(w.render(&input).is_none());
    }
}
