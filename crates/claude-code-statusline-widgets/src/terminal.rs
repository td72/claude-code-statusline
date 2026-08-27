//! Terminal dimensions as reported by Claude Code.
//!
//! Claude Code captures the status line command's output, so `tput cols`
//! and ioctl-based detection do not work. Instead it sets the `COLUMNS`
//! and `LINES` environment variables before running the command
//! (Claude Code v2.1.153 and later).

/// Terminal width in columns from `COLUMNS`, if set and numeric.
pub fn columns() -> Option<usize> {
    std::env::var("COLUMNS").ok()?.trim().parse().ok().filter(|&c| c > 0)
}
