//! Pull request widget.
//!
//! Displays the open pull request (or GitLab merge request) for the current
//! branch as `#1234` (`!1234` for merge requests), optionally as an OSC 8
//! hyperlink to the PR and colored by review state. Returns `None` when no
//! open PR is found.

use claude_code_statusline_components::color::Color;
use claude_code_statusline_components::label::Label;
use claude_code_statusline_components::link::Link;
use claude_code_statusline_model::StatusLineInput;

use crate::Widget;

/// Widget for displaying the open pull request.
///
/// Returns `None` when `pr` is absent from the input.
#[derive(Debug, Clone)]
pub struct PrInfo {
    /// Label formatter.
    pub label: Label,
    /// Hyperlink renderer (wraps the number in OSC 8 when enabled).
    pub link: Link,
    /// Foreground color per `review_state` (`approved`, `pending`,
    /// `changes_requested`, `draft`). Falls back to `label.color`.
    pub colors: Vec<(String, Color)>,
}

impl Default for PrInfo {
    fn default() -> Self {
        Self {
            label: Label::default(),
            link: Link::default(),
            colors: Vec::new(),
        }
    }
}

impl Widget for PrInfo {
    fn render(&self, input: &StatusLineInput) -> Option<String> {
        let pr = input.pr.as_ref()?;
        let sigil = if pr.kind.as_deref() == Some("mr") { '!' } else { '#' };
        let text = self.link.render(&format!("{sigil}{}", pr.number), &pr.url);

        let color = pr.review_state.as_deref().and_then(|state| {
            self.colors
                .iter()
                .find(|(name, _)| name == state)
                .map(|(_, c)| *c)
        });

        match color {
            Some(c) => Label { color: Some(c), ..self.label.clone() }.render(&text),
            None => self.label.render(&text),
        }
        .into()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use claude_code_statusline_model::Pr;

    fn make_input(pr: Option<Pr>) -> StatusLineInput {
        StatusLineInput { pr, ..Default::default() }
    }

    fn pr(number: u64, state: Option<&str>, kind: Option<&str>) -> Pr {
        Pr {
            number,
            url: format!("https://github.com/o/r/pull/{number}"),
            review_state: state.map(Into::into),
            kind: kind.map(Into::into),
        }
    }

    #[test]
    fn renders_number_as_link() {
        let w = PrInfo::default();
        let result = w.render(&make_input(Some(pr(42, None, None)))).unwrap();
        assert!(result.contains("\x1b]8;;https://github.com/o/r/pull/42\x1b\\#42\x1b]8;;\x1b\\"));
    }

    #[test]
    fn renders_plain_when_link_disabled() {
        let w = PrInfo { link: Link { enabled: false }, ..Default::default() };
        assert_eq!(w.render(&make_input(Some(pr(42, None, None)))).unwrap(), "#42");
    }

    #[test]
    fn uses_bang_for_merge_requests() {
        let w = PrInfo { link: Link { enabled: false }, ..Default::default() };
        assert_eq!(w.render(&make_input(Some(pr(7, None, Some("mr"))))).unwrap(), "!7");
    }

    #[test]
    fn colors_by_review_state() {
        let w = PrInfo {
            link: Link { enabled: false },
            colors: vec![("approved".into(), Color::Green), ("draft".into(), Color::White)],
            ..Default::default()
        };
        let result = w.render(&make_input(Some(pr(1, Some("approved"), None)))).unwrap();
        assert!(result.contains("\x1b[32m"));
        assert!(result.contains("#1"));

        // unmapped state stays plain
        assert_eq!(w.render(&make_input(Some(pr(1, Some("pending"), None)))).unwrap(), "#1");
    }

    #[test]
    fn returns_none_without_pr() {
        let w = PrInfo::default();
        assert!(w.render(&make_input(None)).is_none());
    }
}
