//! Git branch widget.
//!
//! Uses `worktree.branch` when the input already carries it (worktree
//! sessions); otherwise shells out to `git branch --show-current` in the
//! workspace's current directory. Returns `None` if the command fails
//! (e.g., not a git repo) or the branch name is empty (detached HEAD).

use claude_code_statusline_components::label::Label;
use claude_code_statusline_model::StatusLineInput;

use crate::Widget;

/// Widget for displaying the current git branch.
///
/// Returns `None` when the current directory is not inside a git
/// repository or when HEAD is detached.
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

impl GitBranch {
    /// Branch name already known from the input, if any.
    ///
    /// In a worktree session `worktree.branch` is the checked-out branch, so
    /// the `git` subprocess can be skipped.
    fn known_branch(input: &StatusLineInput) -> Option<&str> {
        input
            .worktree
            .as_ref()
            .and_then(|wt| wt.branch.as_deref())
            .filter(|b| !b.is_empty())
    }
}

impl Widget for GitBranch {
    fn render(&self, input: &StatusLineInput) -> Option<String> {
        if let Some(name) = Self::known_branch(input) {
            return Some(self.label.render(name));
        }

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

#[cfg(test)]
mod tests {
    use super::*;
    use claude_code_statusline_model::{Workspace, Worktree};

    #[test]
    fn uses_worktree_branch_without_git() {
        let w = GitBranch::default();
        let input = StatusLineInput {
            // A directory that is certainly not a git repository.
            workspace: Workspace { current_dir: "/".into(), ..Default::default() },
            worktree: Some(Worktree {
                name: "my-feature".into(),
                branch: Some("worktree-my-feature".into()),
                ..Default::default()
            }),
            ..Default::default()
        };
        assert_eq!(w.render(&input).unwrap(), "🌿 worktree-my-feature");
    }

    #[test]
    fn returns_none_outside_repo() {
        let w = GitBranch::default();
        let input = StatusLineInput {
            workspace: Workspace { current_dir: "/".into(), ..Default::default() },
            ..Default::default()
        };
        assert!(w.render(&input).is_none());
    }
}
