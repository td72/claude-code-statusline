//! Extended thinking indicator widget.
//!
//! Shows an indicator while extended thinking is enabled for the session.
//! Returns `None` when `thinking` is absent, or when thinking is disabled
//! and `off_text` is empty.

use claude_code_statusline_components::indicator::Indicator;
use claude_code_statusline_model::StatusLineInput;

use crate::Widget;

/// Widget for the extended thinking flag.
pub struct ThinkingStatus {
    /// Indicator formatter.
    pub indicator: Indicator,
}

impl Default for ThinkingStatus {
    fn default() -> Self {
        Self {
            indicator: Indicator {
                on_text: "💭".into(),
                on_color: None,
                ..Default::default()
            },
        }
    }
}

impl Widget for ThinkingStatus {
    fn render(&self, input: &StatusLineInput) -> Option<String> {
        let enabled = input.thinking.as_ref()?.enabled;
        Some(self.indicator.render(enabled)).filter(|s| !s.is_empty())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use claude_code_statusline_model::Thinking;

    fn make_input(enabled: Option<bool>) -> StatusLineInput {
        StatusLineInput {
            thinking: enabled.map(|enabled| Thinking { enabled }),
            ..Default::default()
        }
    }

    #[test]
    fn shows_when_enabled() {
        let w = ThinkingStatus::default();
        assert_eq!(w.render(&make_input(Some(true))).unwrap(), "💭");
    }

    #[test]
    fn hidden_when_disabled_or_absent() {
        let w = ThinkingStatus::default();
        assert!(w.render(&make_input(Some(false))).is_none());
        assert!(w.render(&make_input(None)).is_none());
    }
}
