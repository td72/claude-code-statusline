//! Link component for clickable text (OSC 8 hyperlinks).
//!
//! Wraps text in an [OSC 8](https://en.wikipedia.org/wiki/ANSI_escape_code#OSC)
//! escape sequence so terminals that support hyperlinks (iTerm2, Kitty,
//! WezTerm, ...) make it Cmd/Ctrl-clickable. When disabled, the text is
//! returned unchanged so the output stays clean on terminals without
//! hyperlink support.
//!
//! Typical data sources: `pr.url`, `workspace.repo`.

/// Configuration for hyperlink rendering.
///
/// # Examples
///
/// ```
/// use claude_code_statusline_components::link::Link;
///
/// let link = Link::default();
/// let out = link.render("#42", "https://example.com/pull/42");
/// assert!(out.starts_with("\x1b]8;;https://example.com/pull/42\x1b\\#42"));
///
/// let plain = Link { enabled: false };
/// assert_eq!(plain.render("#42", "https://example.com/pull/42"), "#42");
/// ```
#[derive(Debug, Clone)]
pub struct Link {
    /// Emit OSC 8 sequences. When `false`, `render` returns the text as-is.
    pub enabled: bool,
}

impl Default for Link {
    fn default() -> Self {
        Self { enabled: true }
    }
}

/// OSC 8 sequence start: `ESC ] 8 ; ; <url> ST`.
const OSC8_START: &str = "\x1b]8;;";
/// String terminator (`ESC \\`).
const ST: &str = "\x1b\\";

impl Link {
    /// Render `text` as a hyperlink to `url`.
    ///
    /// Returns `text` unchanged when links are disabled or `url` is empty.
    pub fn render(&self, text: &str, url: &str) -> String {
        if !self.enabled || url.is_empty() {
            return text.to_string();
        }
        format!("{OSC8_START}{url}{ST}{text}{OSC8_START}{ST}")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn wraps_in_osc8() {
        let link = Link::default();
        assert_eq!(
            link.render("repo", "https://github.com/o/r"),
            "\x1b]8;;https://github.com/o/r\x1b\\repo\x1b]8;;\x1b\\"
        );
    }

    #[test]
    fn disabled_returns_plain_text() {
        let link = Link { enabled: false };
        assert_eq!(link.render("repo", "https://github.com/o/r"), "repo");
    }

    #[test]
    fn empty_url_returns_plain_text() {
        let link = Link::default();
        assert_eq!(link.render("repo", ""), "repo");
    }
}
