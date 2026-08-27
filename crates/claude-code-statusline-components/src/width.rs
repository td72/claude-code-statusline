//! Terminal-width helpers that understand ANSI escape sequences.
//!
//! Status line output mixes visible text with CSI color codes and OSC 8
//! hyperlinks, so `str::len` and `chars().count()` both overstate the space
//! a string takes on screen. [`visible_width`] measures only what the
//! terminal draws, and [`truncate_to_width`] cuts a string to fit without
//! ever splitting an escape sequence.

use unicode_width::UnicodeWidthChar;

use crate::color::RESET;

const ESC: char = '\x1b';
const OSC8_CLOSE: &str = "\x1b]8;;\x1b\\";

/// One lexed piece of a string: either an escape sequence (zero width) or a
/// single visible character.
enum Piece<'a> {
    Escape { text: &'a str, is_osc8_open: bool, is_osc8_close: bool },
    Char(char),
}

/// Split `s` into escape sequences and visible characters.
fn pieces(s: &str) -> impl Iterator<Item = Piece<'_>> {
    let bytes = s.as_bytes();
    let mut i = 0;
    std::iter::from_fn(move || {
        if i >= s.len() {
            return None;
        }
        let rest = &s[i..];
        let c = rest.chars().next()?;
        if c != ESC {
            i += c.len_utf8();
            return Some(Piece::Char(c));
        }

        // Escape sequence: find its end.
        let end = match bytes.get(i + 1) {
            // CSI: ESC [ params... final byte in 0x40..=0x7E
            Some(b'[') => bytes[i + 2..]
                .iter()
                .position(|b| (0x40..=0x7E).contains(b))
                .map(|p| i + 2 + p + 1),
            // OSC: ESC ] ... terminated by BEL or ESC \\
            Some(b']') => {
                let body = &bytes[i + 2..];
                let bel = body.iter().position(|&b| b == 0x07).map(|p| i + 2 + p + 1);
                let st = body
                    .windows(2)
                    .position(|w| w == [0x1b, b'\\'])
                    .map(|p| i + 2 + p + 2);
                match (bel, st) {
                    (Some(a), Some(b)) => Some(a.min(b)),
                    (a, b) => a.or(b),
                }
            }
            // Two-byte escape (ESC x)
            Some(_) => Some(i + 2),
            None => Some(i + 1),
        }
        .unwrap_or(s.len());

        let text = &s[i..end];
        i = end;
        let is_osc8 = text.starts_with("\x1b]8;");
        let is_osc8_close = is_osc8 && (text.starts_with("\x1b]8;;\x07") || text.starts_with(OSC8_CLOSE));
        Some(Piece::Escape { text, is_osc8_open: is_osc8 && !is_osc8_close, is_osc8_close })
    })
}

/// Number of terminal columns `s` occupies, ignoring ANSI CSI and OSC
/// sequences and counting wide characters (CJK, emoji) as two columns.
///
/// # Examples
///
/// ```
/// use claude_code_statusline_components::width::visible_width;
///
/// assert_eq!(visible_width("abc"), 3);
/// assert_eq!(visible_width("\x1b[32mabc\x1b[0m"), 3);
/// assert_eq!(visible_width("📁 x"), 4);
/// ```
pub fn visible_width(s: &str) -> usize {
    pieces(s)
        .map(|p| match p {
            Piece::Char(c) => c.width().unwrap_or(0),
            Piece::Escape { .. } => 0,
        })
        .sum()
}

/// Truncate `s` so that its visible width is at most `max`, appending
/// `ellipsis` when anything was cut. Escape sequences are copied through
/// untouched, an unterminated OSC 8 link is closed, and colors are reset
/// so the cut never leaks styling into the rest of the row.
///
/// Returns `s` unchanged when it already fits.
///
/// # Examples
///
/// ```
/// use claude_code_statusline_components::width::truncate_to_width;
///
/// assert_eq!(truncate_to_width("hello world", 8, "…"), "hello w…");
/// assert_eq!(truncate_to_width("short", 8, "…"), "short");
/// ```
pub fn truncate_to_width(s: &str, max: usize, ellipsis: &str) -> String {
    if visible_width(s) <= max {
        return s.to_string();
    }

    let ellipsis_width = visible_width(ellipsis);
    let budget = max.saturating_sub(ellipsis_width);
    let mut out = String::with_capacity(s.len());
    let mut width = 0;
    let mut saw_escape = false;
    let mut link_open = false;

    for piece in pieces(s) {
        match piece {
            Piece::Escape { text, is_osc8_open, is_osc8_close } => {
                saw_escape = true;
                if is_osc8_open {
                    link_open = true;
                } else if is_osc8_close {
                    link_open = false;
                }
                out.push_str(text);
            }
            Piece::Char(c) => {
                let w = c.width().unwrap_or(0);
                if width + w > budget {
                    break;
                }
                width += w;
                out.push(c);
            }
        }
    }

    if link_open {
        out.push_str(OSC8_CLOSE);
    }
    if saw_escape {
        out.push_str(RESET);
    }
    out.push_str(ellipsis);
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn width_ignores_csi_and_osc() {
        assert_eq!(visible_width("\x1b[48;5;240m\x1b[38;5;255m tmp \x1b[0m"), 5);
        assert_eq!(visible_width("\x1b]8;;https://x\x1b\\#12\x1b]8;;\x1b\\"), 3);
        assert_eq!(visible_width("\x1b]8;;https://x\x07#12\x1b]8;;\x07"), 3);
    }

    #[test]
    fn width_counts_wide_chars_twice() {
        assert_eq!(visible_width("日本"), 4);
        assert_eq!(visible_width("🤖 M"), 4);
    }

    #[test]
    fn truncate_keeps_escapes_and_resets() {
        let s = "\x1b[32mhello world\x1b[0m";
        assert_eq!(truncate_to_width(s, 8, "…"), "\x1b[32mhello w\x1b[0m…");
    }

    #[test]
    fn truncate_closes_open_link() {
        let s = "\x1b]8;;https://x\x1b\\pull-request-42\x1b]8;;\x1b\\ tail";
        let out = truncate_to_width(s, 6, "…");
        assert!(out.starts_with("\x1b]8;;https://x\x1b\\pull-"));
        assert!(out.contains(OSC8_CLOSE));
        assert!(out.ends_with("…"));
        assert_eq!(visible_width(&out), 6);
    }

    #[test]
    fn truncate_does_not_split_wide_char() {
        assert_eq!(truncate_to_width("日本語です", 5, "…"), "日本…");
        assert_eq!(visible_width(&truncate_to_width("日本語です", 5, "…")), 5);
    }

    #[test]
    fn truncate_returns_input_when_it_fits() {
        assert_eq!(truncate_to_width("abc", 3, "…"), "abc");
        assert_eq!(truncate_to_width("", 0, "…"), "");
    }

    #[test]
    fn truncate_with_zero_budget_yields_ellipsis_only() {
        assert_eq!(truncate_to_width("abc", 1, "…"), "…");
    }
}
