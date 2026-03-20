//! Vim mode status widget.
//!
//! Displays the current vim mode (`NORMAL` or `INSERT`) with optional
//! per-mode foreground and background colors. Returns `None` when vim
//! mode is not enabled.

use claude_code_statusline_components::color::{Color, RESET};
use claude_code_statusline_model::StatusLineInput;

use crate::Widget;

/// Widget for displaying the current vim mode.
///
/// Returns `None` when vim mode is not enabled.
/// When no colors are configured, the mode string is rendered as plain text.
pub struct VimStatus {
    /// Background color for NORMAL mode.
    pub normal_bg: Option<Color>,
    /// Background color for INSERT mode.
    pub insert_bg: Option<Color>,
    /// Foreground color for NORMAL mode.
    pub normal_fg: Option<Color>,
    /// Foreground color for INSERT mode.
    pub insert_fg: Option<Color>,
}

impl Default for VimStatus {
    fn default() -> Self {
        Self {
            normal_bg: None,
            insert_bg: None,
            normal_fg: None,
            insert_fg: None,
        }
    }
}

impl Widget for VimStatus {
    fn render(&self, input: &StatusLineInput) -> Option<String> {
        let vim = input.vim.as_ref()?;
        let (mode_str, bg, fg) = match vim.mode {
            claude_code_statusline_model::VimMode::Normal => ("NORMAL", &self.normal_bg, &self.normal_fg),
            claude_code_statusline_model::VimMode::Insert => ("INSERT", &self.insert_bg, &self.insert_fg),
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
            cwd: "/test".into(),
            session_id: "s".into(),
            transcript_path: "/t".into(),
            model: Model { id: "m".into(), display_name: "M".into() },
            workspace: Workspace {
                current_dir: "/test".into(),
                project_dir: "/test".into(),
                added_dirs: None,
            },
            version: "1.0".into(),
            output_style: OutputStyle { name: "default".into() },
            cost: Cost {
                total_cost_usd: 0.0,
                total_duration_ms: 0,
                total_api_duration_ms: 0,
                total_lines_added: 0,
                total_lines_removed: 0,
            },
            context_window: ContextWindow {
                total_input_tokens: 0,
                total_output_tokens: 0,
                context_window_size: 200_000,
                used_percentage: None,
                remaining_percentage: None,
                current_usage: None,
            },
            exceeds_200k_tokens: false,
            vim,
            agent: None,
            worktree: None,
            rate_limits: None,
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
    fn renders_with_bg_colors() {
        let w = VimStatus {
            normal_bg: Some(Color::Blue),
            insert_bg: Some(Color::Green),
            normal_fg: Some(Color::White),
            insert_fg: Some(Color::White),
        };

        let input_normal = make_input(Some(Vim { mode: VimMode::Normal }));
        let result = w.render(&input_normal).unwrap();
        assert!(result.contains("\x1b[44m")); // blue bg
        assert!(result.contains(" NORMAL "));

        let input_insert = make_input(Some(Vim { mode: VimMode::Insert }));
        let result = w.render(&input_insert).unwrap();
        assert!(result.contains("\x1b[42m")); // green bg
        assert!(result.contains(" INSERT "));
    }

    #[test]
    fn returns_none_without_vim() {
        let w = VimStatus::default();
        let input = make_input(None);
        assert!(w.render(&input).is_none());
    }
}
