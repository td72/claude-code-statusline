//! Workspace information widget.

use claude_code_statusline_components::path::Path;
use claude_code_statusline_model::StatusLineInput;

use crate::Widget;

/// Widget for displaying workspace directory information.
pub struct WorkspaceInfo {
    /// Path formatter for the current directory.
    pub path: Path,
}

impl Default for WorkspaceInfo {
    fn default() -> Self {
        Self {
            path: Path { prefix: "📁 ".into(), ..Default::default() },
        }
    }
}

impl Widget for WorkspaceInfo {
    fn render(&self, input: &StatusLineInput) -> Option<String> {
        Some(self.path.render(&input.workspace.current_dir))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use claude_code_statusline_model::*;

    fn make_input(dir: &str) -> StatusLineInput {
        StatusLineInput {
            cwd: dir.into(),
            session_id: "s".into(),
            transcript_path: "/t".into(),
            model: Model { id: "m".into(), display_name: "M".into() },
            workspace: Workspace {
                current_dir: dir.into(),
                project_dir: dir.into(),
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
            vim: None,
            agent: None,
            worktree: None,
            rate_limits: None,
        }
    }

    #[test]
    fn renders_basename() {
        let w = WorkspaceInfo::default();
        let input = make_input("/home/user/projects/myapp");
        assert_eq!(w.render(&input).unwrap(), "📁 myapp");
    }
}
