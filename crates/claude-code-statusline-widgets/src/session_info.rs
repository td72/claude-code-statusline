//! Session name widget.
//!
//! Displays the session name (custom `--name` / `/rename` value, or the
//! AI-generated title), truncated to `max_len` characters. Useful for
//! telling parallel sessions apart. Returns `None` when the session has
//! no name.

use claude_code_statusline_components::label::Label;
use claude_code_statusline_model::StatusLineInput;

use crate::Widget;

/// Widget for displaying the session name.
///
/// Returns `None` when `session_name` is absent or empty.
#[derive(Debug, Clone)]
pub struct SessionInfo {
    /// Label formatter.
    pub label: Label,
    /// Maximum number of characters to display; longer names are cut and
    /// suffixed with `…`. `None` disables truncation.
    pub max_len: Option<usize>,
}

impl Default for SessionInfo {
    fn default() -> Self {
        Self {
            label: Label { prefix: "💬 ".into(), ..Default::default() },
            max_len: Some(24),
        }
    }
}

impl SessionInfo {
    fn truncate<'a>(&self, name: &'a str) -> std::borrow::Cow<'a, str> {
        match self.max_len {
            Some(max) if max > 0 && name.chars().count() > max => {
                let cut: String = name.chars().take(max.saturating_sub(1)).collect();
                format!("{}…", cut.trim_end()).into()
            }
            _ => name.into(),
        }
    }
}

impl Widget for SessionInfo {
    fn render(&self, input: &StatusLineInput) -> Option<String> {
        let name = input.session_name.as_deref().filter(|n| !n.is_empty())?;
        Some(self.label.render(&self.truncate(name)))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_input(name: Option<&str>) -> StatusLineInput {
        StatusLineInput { session_name: name.map(Into::into), ..Default::default() }
    }

    #[test]
    fn renders_name() {
        let w = SessionInfo::default();
        assert_eq!(w.render(&make_input(Some("fix login"))).unwrap(), "💬 fix login");
    }

    #[test]
    fn truncates_long_names() {
        let w = SessionInfo { max_len: Some(10), ..Default::default() };
        let result = w.render(&make_input(Some("a very long session title"))).unwrap();
        assert_eq!(result, "💬 a very lo…");
        assert_eq!(result.chars().count(), 2 + 10); // "💬 " + 10
    }

    #[test]
    fn no_truncation_when_disabled() {
        let w = SessionInfo { max_len: None, label: Label::default() };
        let long = "x".repeat(50);
        assert_eq!(w.render(&make_input(Some(&long))).unwrap(), long);
    }

    #[test]
    fn returns_none_without_name() {
        let w = SessionInfo::default();
        assert!(w.render(&make_input(None)).is_none());
        assert!(w.render(&make_input(Some(""))).is_none());
    }
}
