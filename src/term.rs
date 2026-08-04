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

/// Remplit une cellule en tenant compte de la largeur du texte brut
/// (les codes ANSI ne doivent pas compter dans la largeur).
pub fn pad(s: &str, plain: &str, width: usize) -> String {
    let extra = width.saturating_sub(plain.chars().count());
    format!("{}{}", s, " ".repeat(extra))
}
