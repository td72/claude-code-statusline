//! Label component for text/name values.
//!
//! Used for: `model.display_name`, `agent.name`, `vim.mode`, etc.

use crate::color::{Color, RESET};

/// Configuration for label formatting.
#[derive(Debug, Clone)]
pub struct Label {
    /// Optional color for the label text.
    pub color: Option<Color>,
    /// Bracket style around the label. `None` for no brackets.
    pub bracket: Option<BracketStyle>,
    /// Optional prefix icon/text before the label.
    pub prefix: String,
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
            bracket: None,
            prefix: String::new(),
        }
    }
}

impl Label {
    /// Render a label from a string value.
    pub fn render(&self, text: &str) -> String {
        let mut out = String::new();

        if !self.prefix.is_empty() {
            out.push_str(&self.prefix);
        }

        let wrapped = match &self.bracket {
            Some(BracketStyle::Square) => format!("[{text}]"),
            Some(BracketStyle::Round) => format!("({text})"),
            Some(BracketStyle::Angle) => format!("<{text}>"),
            None => text.to_string(),
        };

        match self.color {
            Some(color) => {
                out.push_str(&color.fg_string());
                out.push_str(&wrapped);
                out.push_str(RESET);
            }
            None => out.push_str(&wrapped),
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
