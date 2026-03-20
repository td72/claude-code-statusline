//! Path component for file/directory path values.
//!
//! Formats a filesystem path string in one of three styles: full, basename
//! only, or home-directory shortened (replacing the home prefix with `~`).
//!
//! Typical data sources: `workspace.current_dir`, `workspace.project_dir`,
//! `worktree.path`.

/// Path display style controlling how much of the path is shown.
#[derive(Debug, Clone, Default)]
pub enum PathStyle {
    /// Full path: `/home/user/projects/myapp`
    Full,
    /// Last component only: `myapp`
    #[default]
    BaseName,
    /// Shortened with `~` for home dir: `~/projects/myapp`
    HomeShortened,
}

/// Configuration for path formatting.
///
/// # Examples
///
/// ```
/// use claude_code_statusline_components::path::{Path, PathStyle};
///
/// let p = Path::default(); // BaseName style
/// assert_eq!(p.render("/home/user/myapp"), "myapp");
/// ```
#[derive(Debug, Clone)]
pub struct Path {
    /// Display style.
    pub style: PathStyle,
    /// Optional prefix icon/text.
    pub prefix: String,
    /// Home directory path for `HomeShortened` style.
    /// If empty, `HOME` env var is used.
    pub home_dir: String,
}

impl Default for Path {
    fn default() -> Self {
        Self {
            style: PathStyle::default(),
            prefix: String::new(),
            home_dir: String::new(),
        }
    }
}

impl Path {
    /// Render a path string according to the configured [`PathStyle`].
    ///
    /// If a `prefix` is set, it is prepended to the formatted path.
    pub fn render(&self, path: &str) -> String {
        let formatted = match &self.style {
            PathStyle::Full => path.to_string(),
            PathStyle::BaseName => {
                path.rsplit('/')
                    .next()
                    .unwrap_or(path)
                    .to_string()
            }
            PathStyle::HomeShortened => {
                let home = if self.home_dir.is_empty() {
                    std::env::var("HOME").unwrap_or_default()
                } else {
                    self.home_dir.clone()
                };
                if !home.is_empty() && path.starts_with(&home) {
                    format!("~{}", &path[home.len()..])
                } else {
                    path.to_string()
                }
            }
        };

        if self.prefix.is_empty() {
            formatted
        } else {
            format!("{}{formatted}", self.prefix)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basename() {
        let p = Path::default();
        assert_eq!(p.render("/home/user/projects/myapp"), "myapp");
    }

    #[test]
    fn full_path() {
        let p = Path { style: PathStyle::Full, ..Default::default() };
        assert_eq!(p.render("/home/user/projects"), "/home/user/projects");
    }

    #[test]
    fn home_shortened() {
        let p = Path {
            style: PathStyle::HomeShortened,
            home_dir: "/home/user".to_string(),
            ..Default::default()
        };
        assert_eq!(p.render("/home/user/projects/myapp"), "~/projects/myapp");
    }

    #[test]
    fn home_shortened_no_match() {
        let p = Path {
            style: PathStyle::HomeShortened,
            home_dir: "/home/other".to_string(),
            ..Default::default()
        };
        assert_eq!(p.render("/home/user/projects"), "/home/user/projects");
    }

    #[test]
    fn with_prefix() {
        let p = Path { prefix: "📁 ".to_string(), ..Default::default() };
        assert_eq!(p.render("/home/user/myapp"), "📁 myapp");
    }

    #[test]
    fn basename_no_slash() {
        let p = Path::default();
        assert_eq!(p.render("myapp"), "myapp");
    }
}
