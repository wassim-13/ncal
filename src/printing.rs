use unicode_width::UnicodeWidthStr;

pub fn left_right(left: &str, right: &str, total_width: usize) -> String {
    let left_w = visible_width(left);
    let right_w = visible_width(right);
    let spaces = total_width.saturating_sub(left_w + right_w);

    format!("{left}{}{right}", " ".repeat(spaces))
}

fn visible_width(s: &str) -> usize {
    let stripped = strip_ansi(s);
    UnicodeWidthStr::width(stripped.as_str())
}

fn strip_ansi(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    let mut chars = s.chars().peekable();

    while let Some(c) = chars.next() {
        if c == '\x1b' {
            if chars.peek() == Some(&'[') {
                chars.next(); // skip '['

                for ch in chars.by_ref() {
                    if ('@'..='~').contains(&ch) {
                        break;
                    }
                }
            }
        } else {
            out.push(c);
        }
    }

    out
}
#[derive(Clone, Copy)]
pub enum Color {
    Blue,   // #3b82f6
    Grey,   // #a1a1ff
    Amber,  // #f59e0b
    Orange, // #f97316
    Green,  // #22c55e
}
impl Color {
    fn rgb(self) -> (u8, u8, u8) {
        match self {
            Color::Blue => (0x3b, 0x82, 0xf6),
            Color::Grey => (0xa1, 0xa5, 0xdb),
            Color::Amber => (0xf5, 0x9e, 0x0b),
            Color::Orange => (0xf9, 0x73, 0x16),
            Color::Green => (0x22, 0xc5, 0x5e),
        }
    }

    fn ansi_fg(self) -> String {
        let (r, g, b) = self.rgb();
        format!("\x1b[38;2;{};{};{}m", r, g, b)
    }
}

pub fn progress_bar(current: f64, total: f64, width: usize, color: Color) -> String {
    let ratio = if total <= 0.0 {
        0.0
    } else {
        (current / total).clamp(0.0, 1.0)
    };

    let filled = (ratio * width as f64).round() as usize;
    let empty = width.saturating_sub(filled);

    let filled_part = format!("{}{}{}", color.ansi_fg(), "■".repeat(filled), "\x1b[0m");
    let empty_part = format!("\x1b[90m{}\x1b[0m", "□".repeat(empty));

    let bar = format!("[{}{}]", filled_part, empty_part);
    let pct = format!("{:>5.1}%", ratio * 100.0);

    format!("{bar} {pct}")
}
