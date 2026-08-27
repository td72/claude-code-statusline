//! Vim mode status widget.
//!
//! Displays the current vim mode (`NORMAL`, `INSERT`, `VISUAL`, or
//! `V-LINE`) with optional per-mode foreground and background colors.
//! Returns `None` when vim mode is not enabled or the mode is unknown.

use claude_code_statusline_components::color::{Color, RESET};
use claude_code_statusline_model::{StatusLineInput, VimMode};

use crate::Widget;

/// Widget for displaying the current vim mode.
///
/// Returns `None` when vim mode is not enabled or the mode is not one
/// this widget knows how to display ([`VimMode::Unknown`]).
/// When no colors are configured, the mode string is rendered as plain text.
/// Both visual modes (`VISUAL`, `VISUAL LINE`) share the `visual_*` colors.
#[derive(Debug, Clone, Default)]
pub struct VimStatus {
    /// Background color for NORMAL mode.
    pub normal_bg: Option<Color>,
    /// Background color for INSERT mode.
    pub insert_bg: Option<Color>,
    /// Background color for VISUAL / VISUAL LINE modes.
    pub visual_bg: Option<Color>,
    /// Foreground color for NORMAL mode.
    pub normal_fg: Option<Color>,
    /// Foreground color for INSERT mode.
    pub insert_fg: Option<Color>,
    /// Foreground color for VISUAL / VISUAL LINE modes.
    pub visual_fg: Option<Color>,
}

impl Widget for VimStatus {
    fn render(&self, input: &StatusLineInput) -> Option<String> {
        let vim = input.vim.as_ref()?;
        let (mode_str, bg, fg) = match vim.mode {
            VimMode::Normal => ("NORMAL", &self.normal_bg, &self.normal_fg),
            VimMode::Insert => ("INSERT", &self.insert_bg, &self.insert_fg),
            VimMode::Visual => ("VISUAL", &self.visual_bg, &self.visual_fg),
            VimMode::VisualLine => ("V-LINE", &self.visual_bg, &self.visual_fg),
            VimMode::Unknown => return None,
        };

        let has_color = bg.is_some() || fg.is_some();
        if !has_color {
            return Some(mode_str.to_string());
        }

        let mut out = String::new();
        if let Some(bg) = bg {
            out.push_str(&bg.bg_string());
        }
        if let Some(fg) = fg {
            out.push_str(&fg.fg_string());
        }
        out.push(' ');
        out.push_str(mode_str);
        out.push(' ');
        out.push_str(RESET);
        Some(out)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use claude_code_statusline_model::*;

    fn make_input(vim: Option<Vim>) -> StatusLineInput {
        StatusLineInput {
            vim,
            ..Default::default()
        }
    }

    #[test]
    fn renders_normal_plain() {
        let w = VimStatus::default();
        let input = make_input(Some(Vim { mode: VimMode::Normal }));
        assert_eq!(w.render(&input).unwrap(), "NORMAL");
    }

    #[test]
    fn renders_insert_plain() {
        let w = VimStatus::default();
        let input = make_input(Some(Vim { mode: VimMode::Insert }));
        assert_eq!(w.render(&input).unwrap(), "INSERT");
    }

    #[test]
    fn renders_visual_modes_plain() {
        let w = VimStatus::default();
        let input = make_input(Some(Vim { mode: VimMode::Visual }));
        assert_eq!(w.render(&input).unwrap(), "VISUAL");
        let input = make_input(Some(Vim { mode: VimMode::VisualLine }));
        assert_eq!(w.render(&input).unwrap(), "V-LINE");
    }

    #[test]
    fn returns_none_for_unknown_mode() {
        let w = VimStatus::default();
        let input = make_input(Some(Vim { mode: VimMode::Unknown }));
        assert!(w.render(&input).is_none());
    }

    #[test]
    fn renders_with_bg_colors() {
        let w = VimStatus {
            normal_bg: Some(Color::Blue),
            insert_bg: Some(Color::Green),
            visual_bg: Some(Color::Magenta),
            normal_fg: Some(Color::White),
            insert_fg: Some(Color::White),
            visual_fg: Some(Color::White),
        };

        let input_normal = make_input(Some(Vim { mode: VimMode::Normal }));
        let result = w.render(&input_normal).unwrap();
        assert!(result.contains("\x1b[44m")); // blue bg
        assert!(result.contains(" NORMAL "));

        let input_insert = make_input(Some(Vim { mode: VimMode::Insert }));
        let result = w.render(&input_insert).unwrap();
        assert!(result.contains("\x1b[42m")); // green bg
        assert!(result.contains(" INSERT "));

        let input_visual = make_input(Some(Vim { mode: VimMode::VisualLine }));
        let result = w.render(&input_visual).unwrap();
        assert!(result.contains("\x1b[45m")); // magenta bg
        assert!(result.contains(" V-LINE "));
    }

    #[test]
    fn returns_none_without_vim() {
        let w = VimStatus::default();
        let input = make_input(None);
        assert!(w.render(&input).is_none());
    }
}
