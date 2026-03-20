//! Statusline rendering components for Claude Code.
//!
//! Each component takes a value and renders it as a formatted string,
//! optionally with ANSI color codes.

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
