//! Workspace information widget.
//!
//! Displays the current working directory using the [`Path`] component
//! for formatting, wrapped in an optional [`Label`] for color/badge styling.

use claude_code_statusline_components::label::Label;
use claude_code_statusline_components::path::Path;
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
}

impl Default for WorkspaceInfo {
    fn default() -> Self {
        Self {
            path: Path { prefix: "📁 ".into(), ..Default::default() },
            label: Label::default(),
        }
    }
}

impl Widget for WorkspaceInfo {
    fn render(&self, input: &StatusLineInput) -> Option<String> {
        let path_str = self.path.render(&input.workspace.current_dir);
        if self.label.color.is_some() || self.label.bg.is_some() {
            Some(self.label.render(&path_str))
        } else {
            Some(path_str)
        }
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
}
