//! Label component for text/name values.
//!
//! Used for: `model.display_name`, `agent.name`, `vim.mode`, etc.

use crate::color::{Color, RESET};

/// Configuration for label formatting.
#[derive(Debug, Clone)]
pub struct Label {
    /// Optional foreground color for the label text.
    pub color: Option<Color>,
    /// Optional background color for the label.
    pub bg: Option<Color>,
    /// Bracket style around the label. `None` for no brackets.
    pub bracket: Option<BracketStyle>,
    /// Optional prefix icon/text before the label.
    pub prefix: String,
    /// Add spaces around the text when bg is set (badge style).
    pub pad: bool,
}

/// Bracket style for labels.
#[derive(Debug, Clone)]
pub enum BracketStyle {
    /// `[text]`
    Square,
    /// `(text)`
    Round,
    /// `<text>`
    Angle,
}

impl Default for Label {
    fn default() -> Self {
        Self {
            color: None,
            bg: None,
            bracket: None,
            prefix: String::new(),
            pad: false,
        }
    }
}

impl Label {
    /// Render a label from a string value.
    pub fn render(&self, text: &str) -> String {
        let mut out = String::new();

        let wrapped = match &self.bracket {
            Some(BracketStyle::Square) => format!("[{text}]"),
            Some(BracketStyle::Round) => format!("({text})"),
            Some(BracketStyle::Angle) => format!("<{text}>"),
            None => text.to_string(),
        };

        let has_color = self.color.is_some() || self.bg.is_some();
        if has_color {
            if let Some(bg) = self.bg {
                out.push_str(&bg.bg_string());
            }
            if let Some(fg) = self.color {
                out.push_str(&fg.fg_string());
            }
            if self.pad {
                out.push(' ');
            }
            if !self.prefix.is_empty() {
                out.push_str(&self.prefix);
            }
            out.push_str(&wrapped);
            if self.pad {
                out.push(' ');
            }
            out.push_str(RESET);
        } else {
            if !self.prefix.is_empty() {
                out.push_str(&self.prefix);
            }
            out.push_str(&wrapped);
        }

        out
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn plain_label() {
        assert_eq!(Label::default().render("Opus"), "Opus");
    }

    #[test]
    fn bracketed_label() {
        let l = Label { bracket: Some(BracketStyle::Square), ..Default::default() };
        assert_eq!(l.render("Opus"), "[Opus]");
    }

    #[test]
    fn colored_label() {
        let l = Label { color: Some(Color::Cyan), ..Default::default() };
        let result = l.render("Opus");
        assert_eq!(result, "\x1b[36mOpus\x1b[0m");
    }

    #[test]
    fn label_with_prefix() {
        let l = Label {
            prefix: "🤖 ".to_string(),
            bracket: Some(BracketStyle::Square),
            ..Default::default()
        };
        assert_eq!(l.render("Opus"), "🤖 [Opus]");
    }

    #[test]
    fn round_brackets() {
        let l = Label { bracket: Some(BracketStyle::Round), ..Default::default() };
        assert_eq!(l.render("NORMAL"), "(NORMAL)");
    }
}
