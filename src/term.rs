use std::io::{self, IsTerminal};

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Color {
    Red,
    Green,
    White,
    Blue,
}

pub struct Style {
    pub enabled: bool,
}

impl Style {
    pub fn detect(force: bool, no_color: bool) -> Style {
        let tty = io::stdout().is_terminal();
        let noc = std::env::var_os("NO_COLOR").is_some();
        Style {
            enabled: !no_color && (force || (tty && !noc)),
        }
    }

    pub fn paint(&self, s: &str, c: Color) -> String {
        if !self.enabled {
            return s.to_string();
        }
        format!("\x1b[{}m{}\x1b[0m", code(c), s)
    }

    pub fn paint_bold(&self, s: &str, c: Color) -> String {
        if !self.enabled {
            return s.to_string();
        }
        format!("\x1b[1;{}m{}\x1b[0m", code(c), s)
    }
}

const fn code(c: Color) -> &'static str {
    match c {
        Color::Red => "31",
        Color::Green => "32",
        Color::White => "37",
        Color::Blue => "34",
    }
}

/// Fills a cell taking the raw text width into account
/// (ANSI codes must not count toward the width).
pub fn pad(s: &str, plain: &str, width: usize) -> String {
    let extra = width.saturating_sub(plain.chars().count());
    format!("{}{}", s, " ".repeat(extra))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ansi_codes() {
        assert_eq!(code(Color::Red), "31");
        assert_eq!(code(Color::Green), "32");
        assert_eq!(code(Color::White), "37");
        assert_eq!(code(Color::Blue), "34");
    }

    #[test]
    fn paint_respects_style() {
        let plain = Style { enabled: false };
        assert_eq!(plain.paint("x", Color::Red), "x");
        assert_eq!(plain.paint_bold("x", Color::Red), "x");

        let color = Style { enabled: true };
        assert_eq!(color.paint("x", Color::Red), "\x1b[31mx\x1b[0m");
        assert_eq!(color.paint_bold("x", Color::Blue), "\x1b[1;34mx\x1b[0m");
    }

    #[test]
    fn pad_uses_plain_width() {
        assert_eq!(pad("\x1b[31mx\x1b[0m", "x", 3), "\x1b[31mx\x1b[0m  ");
        assert_eq!(pad("abc", "abc", 3), "abc");
        assert_eq!(pad("abc", "abc", 1), "abc");
        assert_eq!(pad("é", "é", 2), "é ");
    }
}
