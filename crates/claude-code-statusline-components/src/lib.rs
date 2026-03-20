//! Low-level rendering components for the Claude Code status line.
//!
//! Each component is a small, self-contained formatter that takes a single
//! value (a percentage, a duration in milliseconds, a currency amount, etc.)
//! and produces a terminal-ready string, optionally decorated with ANSI color
//! escape sequences.
//!
//! Components are intentionally stateless and know nothing about
//! [`claude_code_statusline_model::StatusLineInput`]; higher-level
//! *widgets* (in the `claude-code-statusline-widgets` crate) bridge the gap
//! between the model and these components.
//!
//! # Available components
//!
//! | Component      | Purpose                                   |
//! |---------------|-------------------------------------------|
//! | [`ProgressBar`] | Percentage bar with color thresholds      |
//! | [`Duration`]    | Milliseconds to human-readable duration   |
//! | [`Currency`]    | Floating-point value to currency string   |
//! | [`Count`]       | Integer with plain / comma / compact style|
//! | [`Countdown`]   | Unix timestamp delta to remaining time    |
//! | [`Label`]       | Text with optional color and brackets     |
//! | [`Path`]        | File path with basename / home-shortened  |
//! | [`Indicator`]   | Boolean flag to icon or text              |

pub mod color;
pub mod count;
pub mod countdown;
pub mod currency;
pub mod duration;
pub mod indicator;
pub mod label;
pub mod path;
pub mod progress_bar;

pub use count::Count;
pub use countdown::Countdown;
pub use currency::Currency;
pub use duration::Duration;
pub use indicator::Indicator;
pub use label::Label;
pub use path::Path;
pub use progress_bar::ProgressBar;
