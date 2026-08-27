//! Workspace information widget.
//!
//! Displays the current working directory using the [`Path`] component
//! for formatting, wrapped in an optional [`Label`] for color/badge styling.
//! Optionally switches to a more compact path style on narrow terminals
//! (width from `COLUMNS`, see [`crate::terminal::columns`]).

use claude_code_statusline_components::label::Label;
use claude_code_statusline_components::path::{Path, PathStyle};
use claude_code_statusline_model::StatusLineInput;

use crate::Widget;

/// Widget for displaying workspace directory information.
///
/// Always returns `Some` because workspace data is always present.
pub struct WorkspaceInfo {
    /// Path formatter for the current directory.
    pub path: Path,
    /// Label formatter for styling (color, bg, etc.).
    pub label: Label,
    /// Alternative path style used when the terminal is narrower than
    /// `narrow_below` columns.
    pub narrow_style: Option<PathStyle>,
    /// Column threshold for `narrow_style`.
    pub narrow_below: usize,
}

impl Default for WorkspaceInfo {
    fn default() -> Self {
        Self {
            path: Path { prefix: "📁 ".into(), ..Default::default() },
            label: Label::default(),
            narrow_style: None,
            narrow_below: 0,
        }
    }
}

impl WorkspaceInfo {
    /// Render for an explicit terminal width; the [`Widget`] impl reads
    /// `COLUMNS` and delegates here.
    pub fn render_with_columns(&self, input: &StatusLineInput, columns: Option<usize>) -> Option<String> {
        let narrow = match (&self.narrow_style, columns) {
            (Some(style), Some(cols)) if cols < self.narrow_below => Some(style.clone()),
            _ => None,
        };
        let path_str = match narrow {
            Some(style) => Path { style, ..self.path.clone() }.render(&input.workspace.current_dir),
            None => self.path.render(&input.workspace.current_dir),
        };
        if self.label.color.is_some() || self.label.bg.is_some() {
            Some(self.label.render(&path_str))
        } else {
            Some(path_str)
        }
    }
}

impl Widget for WorkspaceInfo {
    fn render(&self, input: &StatusLineInput) -> Option<String> {
        self.render_with_columns(input, crate::terminal::columns())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use claude_code_statusline_model::*;

    fn make_input(dir: &str) -> StatusLineInput {
        StatusLineInput {
            cwd: dir.into(),
            workspace: Workspace { current_dir: dir.into(), project_dir: dir.into(), ..Default::default() },
            ..Default::default()
        }
    }

    #[test]
    fn renders_basename() {
        let w = WorkspaceInfo::default();
        let input = make_input("/home/user/projects/myapp");
        assert_eq!(w.render(&input).unwrap(), "📁 myapp");
    }

    #[test]
    fn switches_to_narrow_style_below_threshold() {
        let w = WorkspaceInfo {
            path: Path { style: PathStyle::Full, prefix: "📁 ".into(), ..Default::default() },
            narrow_style: Some(PathStyle::BaseName),
            narrow_below: 100,
            ..Default::default()
        };
        let input = make_input("/home/user/projects/myapp");
        assert_eq!(w.render_with_columns(&input, Some(120)).unwrap(), "📁 /home/user/projects/myapp");
        assert_eq!(w.render_with_columns(&input, Some(80)).unwrap(), "📁 myapp");
        // unknown width keeps the primary style
        assert_eq!(w.render_with_columns(&input, None).unwrap(), "📁 /home/user/projects/myapp");
    }
}
