//! Fast mode indicator widget.
//!
//! Shows an indicator while fast mode is enabled for the session.
//! Returns `None` when fast mode is off and `off_text` is empty.

use claude_code_statusline_components::color::Color;
use claude_code_statusline_components::indicator::Indicator;
use claude_code_statusline_model::StatusLineInput;

use crate::Widget;

/// Widget for the fast mode flag.
pub struct FastMode {
    /// Indicator formatter.
    pub indicator: Indicator,
}

impl Default for FastMode {
    fn default() -> Self {
        Self {
            indicator: Indicator {
                on_text: "⚡ fast".into(),
                on_color: Some(Color::Yellow),
                ..Default::default()
            },
        }
    }
}

impl Widget for FastMode {
    fn render(&self, input: &StatusLineInput) -> Option<String> {
        Some(self.indicator.render(input.fast_mode)).filter(|s| !s.is_empty())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn shows_when_enabled() {
        let w = FastMode::default();
        let input = StatusLineInput { fast_mode: true, ..Default::default() };
        assert!(w.render(&input).unwrap().contains("fast"));
    }

    #[test]
    fn hidden_when_disabled() {
        let w = FastMode::default();
        assert!(w.render(&StatusLineInput::default()).is_none());
    }

    #[test]
    fn off_text_when_configured() {
        let w = FastMode {
            indicator: Indicator { off_text: "slow".into(), ..Default::default() },
        };
        assert_eq!(w.render(&StatusLineInput::default()).unwrap(), "slow");
    }
}
