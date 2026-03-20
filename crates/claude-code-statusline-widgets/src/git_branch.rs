//! Git branch widget.
//!
//! Runs `git branch --show-current` to get the current branch name.

use claude_code_statusline_components::label::Label;
use claude_code_statusline_model::StatusLineInput;

use crate::Widget;

/// Widget for displaying the current git branch.
pub struct GitBranch {
    /// Label formatter.
    pub label: Label,
}

impl Default for GitBranch {
    fn default() -> Self {
        Self {
            label: Label { prefix: "🌿 ".into(), ..Default::default() },
        }
    }
}

impl Widget for GitBranch {
    fn render(&self, input: &StatusLineInput) -> Option<String> {
        let branch = std::process::Command::new("git")
            .args(["branch", "--show-current"])
            .current_dir(&input.workspace.current_dir)
            .stdout(std::process::Stdio::piped())
            .stderr(std::process::Stdio::null())
            .output()
            .ok()?;

        if !branch.status.success() {
            return None;
        }

        let name = String::from_utf8_lossy(&branch.stdout).trim().to_string();
        if name.is_empty() {
            return None;
        }

        Some(self.label.render(&name))
    }
}
