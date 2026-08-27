//! Repository widget.
//!
//! Displays the repository identity parsed from the `origin` remote
//! (`owner/name` by default), optionally as an OSC 8 hyperlink to the
//! repository web page. Returns `None` outside a git repository or when no
//! `origin` remote is configured. Uses only `StatusLineInput` data; no
//! `git` subprocess is spawned.

use claude_code_statusline_components::label::Label;
use claude_code_statusline_components::link::Link;
use claude_code_statusline_model::StatusLineInput;

use crate::Widget;

/// How much of the repository identity to display.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub enum RepoStyle {
    /// `name`
    Name,
    /// `owner/name`
    #[default]
    OwnerName,
    /// `host/owner/name`
    Full,
}

/// Widget for displaying the repository identity.
///
/// Returns `None` when `workspace.repo` is absent from the input.
#[derive(Debug, Clone)]
pub struct RepoInfo {
    /// Label formatter.
    pub label: Label,
    /// Hyperlink renderer (links to `https://{host}/{owner}/{name}`).
    pub link: Link,
    /// Display style.
    pub style: RepoStyle,
}

impl Default for RepoInfo {
    fn default() -> Self {
        Self {
            label: Label::default(),
            link: Link::default(),
            style: RepoStyle::default(),
        }
    }
}

impl Widget for RepoInfo {
    fn render(&self, input: &StatusLineInput) -> Option<String> {
        let repo = input.workspace.repo.as_ref()?;
        let text = match self.style {
            RepoStyle::Name => repo.name.clone(),
            RepoStyle::OwnerName => format!("{}/{}", repo.owner, repo.name),
            RepoStyle::Full => format!("{}/{}/{}", repo.host, repo.owner, repo.name),
        };
        let url = if repo.host.is_empty() {
            String::new()
        } else {
            format!("https://{}/{}/{}", repo.host, repo.owner, repo.name)
        };
        Some(self.label.render(&self.link.render(&text, &url)))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use claude_code_statusline_model::{Repo, Workspace};

    fn make_input(repo: Option<Repo>) -> StatusLineInput {
        StatusLineInput {
            workspace: Workspace { repo, ..Default::default() },
            ..Default::default()
        }
    }

    fn repo() -> Repo {
        Repo { host: "github.com".into(), owner: "anthropics".into(), name: "claude-code".into() }
    }

    #[test]
    fn renders_owner_name_with_link() {
        let w = RepoInfo::default();
        let result = w.render(&make_input(Some(repo()))).unwrap();
        assert_eq!(
            result,
            "\x1b]8;;https://github.com/anthropics/claude-code\x1b\\anthropics/claude-code\x1b]8;;\x1b\\"
        );
    }

    #[test]
    fn renders_styles_plain() {
        let plain = |style| RepoInfo { link: Link { enabled: false }, style, ..Default::default() };
        let input = make_input(Some(repo()));
        assert_eq!(plain(RepoStyle::Name).render(&input).unwrap(), "claude-code");
        assert_eq!(plain(RepoStyle::OwnerName).render(&input).unwrap(), "anthropics/claude-code");
        assert_eq!(plain(RepoStyle::Full).render(&input).unwrap(), "github.com/anthropics/claude-code");
    }

    #[test]
    fn returns_none_without_repo() {
        let w = RepoInfo::default();
        assert!(w.render(&make_input(None)).is_none());
    }
}
