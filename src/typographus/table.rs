use anstyle::{AnsiColor, Reset, Style};
use once_cell::sync::Lazy;

/// Box-drawing characters for table borders.
#[derive(Clone, Copy, Debug)]
pub struct TableChars {
    pub top_left: char,
    pub top_right: char,
    pub bottom_left: char,
    pub bottom_right: char,
    pub horiz: char,
    pub vert: char,
}

impl Default for TableChars {
    fn default() -> Self {
        Self {
            top_left: '┌',
            top_right: '┐',
            bottom_left: '└',
            bottom_right: '┘',
            horiz: '─',
            vert: '│',
        }
    }
}

/// Styling and spacing for table rendering.
#[derive(Clone, Copy, Debug)]
pub struct TableStyle {
    pub chars: TableChars,
    /// Foreground color used for borders and fill bar (aka "accent").
    pub accent_fg: AnsiColor,
    /// Spaces after left borders (top/body) and before right borders.
    pub inside_pad: usize,
    /// Optional bottom-left text on the bottom border. Not shown by default.
    pub bottom_left_text: Option<&'static str>,
}

impl Default for TableStyle {
    fn default() -> Self {
        Self {
            chars: TableChars::default(),
            accent_fg: AnsiColor::BrightBlack,
            inside_pad: 1,
            bottom_left_text: None,
        }
    }
}

static TABLE_STYLE: Lazy<std::sync::RwLock<TableStyle>> =
    Lazy::new(|| std::sync::RwLock::new(TableStyle::default()));

pub struct TableStyleBuilder {
    style: TableStyle,
}

impl Default for TableStyleBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl TableStyleBuilder {
    pub fn new() -> Self {
        Self {
            style: TableStyle::default(),
        }
    }
    pub fn chars(mut self, chars: TableChars) -> Self {
        self.style.chars = chars;
        self
    }
    pub fn accent_fg(mut self, fg: AnsiColor) -> Self {
        self.style.accent_fg = fg;
        self
    }
    pub fn inside_pad(mut self, n: usize) -> Self {
        self.style.inside_pad = n;
        self
    }
    pub fn bottom_left_text(mut self, t: Option<&'static str>) -> Self {
        self.style.bottom_left_text = t;
        self
    }
    pub fn apply(self) {
        *TABLE_STYLE.write().expect("table style lock") = self.style;
    }
}

pub fn set_table_style(style: TableStyle) {
    *TABLE_STYLE.write().expect("table style lock") = style;
}

/// Best-effort terminal width detection without extra deps.
/// Uses the `COLUMNS` environment variable if set.
pub fn terminal_width() -> Option<usize> {
    std::env::var("COLUMNS")
        .ok()
        .and_then(|s| s.parse::<usize>().ok())
}

#[inline]
fn accent_on() -> String {
    let fg = TABLE_STYLE.read().expect("table style lock").accent_fg;
    Style::new().fg_color(Some(fg.into())).render().to_string()
}

fn strip_ansi(s: &str) -> String {
    let mut out: Vec<u8> = Vec::with_capacity(s.len());
    let mut bytes = s.as_bytes().iter().copied().peekable();
    while let Some(b) = bytes.next() {
        if b == 0x1b && matches!(bytes.peek(), Some(b'[')) {
            let _ = bytes.next(); // skip '['
            for c in bytes.by_ref() {
                if c == b'm' {
                    break;
                }
            }
            continue;
        }
        out.push(b);
    }
    String::from_utf8(out).unwrap_or_default()
}

fn visible_len(s: &str) -> usize {
    strip_ansi(s).chars().count()
}

/// Truncate a possibly ANSI-styled string to `max_visible` visible characters.
/// Ensures any opened style is closed by appending a Reset (by caller row style).
fn truncate_visible(s: &str, max_visible: usize) -> String {
    let mut out: Vec<u8> = Vec::with_capacity(s.len());
    let mut bytes = s.as_bytes().iter().copied().peekable();
    let mut vis = 0usize;
    while let Some(b) = bytes.next() {
        if b == 0x1b && matches!(bytes.peek(), Some(b'[')) {
            // copy entire SGR sequence through 'm'
            out.push(b);
            if let Some(next) = bytes.next() {
                out.push(next);
            }
            for c in bytes.by_ref() {
                out.push(c);
                if c == b'm' {
                    break;
                }
            }
            continue;
        }
        if vis >= max_visible {
            break;
        }
        out.push(b);
        vis += 1;
    }
    String::from_utf8(out).unwrap_or_default()
}

/// ┌ <left> <bar> <right> ┐
pub fn box_top_line(left_seg: &str, right_meta: &str, width: usize) -> String {
    let cfg = *TABLE_STYLE.read().expect("table style lock");
    let pad = cfg.inside_pad;
    let sp = " ".repeat(pad);
    let chars = cfg.chars;
    let left_border = format!("{tl}{sp}", tl = chars.top_left, sp = sp);
    let right_border = format!("{sp}{tr}", tr = chars.top_right, sp = sp);

    // Compute inner width (everything between the left and right border segments)
    let inner_width = width.saturating_sub(visible_len(&left_border) + visible_len(&right_border));

    let left_len = visible_len(left_seg) + 1; // +1 space after left
    // Compute/crop right meta to fit, leave at least space for bar when possible
    let available = inner_width.saturating_sub(left_len);
    let mut right_part = String::new();
    let mut right_vis = 0usize;
    if !right_meta.is_empty() && available > 0 {
        let right_max = available.saturating_sub(1); // 1 space before right meta
        if right_max > 0 {
            let vis_len = visible_len(right_meta);
            right_part.push(' ');
            if vis_len <= right_max {
                right_part.push_str(right_meta);
                right_vis = 1 + vis_len;
            } else {
                let ell = if right_max >= 1 { 1 } else { 0 };
                let trunc = truncate_visible(right_meta, right_max.saturating_sub(ell));
                right_part.push_str(&trunc);
                if ell == 1 {
                    right_part.push('…');
                }
                right_vis = visible_len(&right_part);
            }
        }
    }
    let bar_len = inner_width.saturating_sub(left_len + right_vis);

    // Prepare pieces with self-reset styling to avoid ANSI bleed.
    let acc = accent_on();
    let mut out = String::with_capacity(width);
    // left border
    out.push_str(&acc);
    out.push_str(&left_border);
    out.push_str(&format!("{Reset}"));
    // left content + space
    out.push_str(left_seg);
    out.push(' ');
    // bar
    out.push_str(&acc);
    for _ in 0..bar_len {
        out.push(chars.horiz);
    }
    out.push_str(&format!("{Reset}"));
    // right meta if any (already prefixed)
    out.push_str(&right_part);
    // right border
    out.push_str(&acc);
    out.push_str(&right_border);
    out.push_str(&format!("{Reset}"));

    out
}

/// │ <content...>········································ │
pub fn box_body_row(content: &str, width: usize) -> String {
    let cfg = *TABLE_STYLE.read().expect("table style lock");
    let pad = cfg.inside_pad;
    let sp = " ".repeat(pad);
    let chars = cfg.chars;
    let left_border = format!("{vt}{sp}", vt = chars.vert, sp = sp);
    let right_border = format!("{sp}{vt}", vt = chars.vert, sp = sp);

    let inner_width = width.saturating_sub(visible_len(&left_border) + visible_len(&right_border));
    let content_vis = visible_len(content);
    let (rendered, pad_right) = if content_vis > inner_width {
        let ell = if inner_width >= 1 { 1 } else { 0 };
        let trunc = truncate_visible(content, inner_width.saturating_sub(ell));
        let mut s = trunc;
        if ell == 1 {
            s.push('…');
        }
        (s, 0)
    } else {
        (content.to_string(), inner_width - content_vis)
    };

    let acc = accent_on();
    let mut out = String::with_capacity(width);
    out.push_str(&acc);
    out.push_str(&left_border);
    out.push_str(&format!("{Reset}"));
    out.push_str(&rendered);
    for _ in 0..pad_right {
        out.push(' ');
    }
    out.push_str(&acc);
    out.push_str(&right_border);
    out.push_str(&format!("{Reset}"));
    out
}

/// │ <spaces> │
pub fn box_empty_row(width: usize) -> String {
    box_body_row("", width)
}

/// └ <bar> <meta> ┘
pub fn box_bottom_line(right_meta: &str, width: usize) -> String {
    let cfg = *TABLE_STYLE.read().expect("table style lock");
    let sp = " ".repeat(cfg.inside_pad);
    let chars = cfg.chars;
    let left_border = format!("{bl}", bl = chars.bottom_left);
    let right_border = format!("{sp}{br}", br = chars.bottom_right, sp = sp);

    let inner_width = width.saturating_sub(visible_len(&left_border) + visible_len(&right_border));

    // Optional bottom-left text segment
    let mut left_seg = String::new();
    if let Some(text) = cfg.bottom_left_text
        && !text.is_empty()
    {
        left_seg.push(' ');
        left_seg.push_str(text);
        left_seg.push(' ');
    }
    let left_len = visible_len(&left_seg);

    // Compute/crop right meta to fit
    let mut right_part = String::new();
    let mut right_vis = 0usize;
    if !right_meta.is_empty() && inner_width > left_len {
        let right_max = inner_width - left_len;
        let right_max = right_max.saturating_sub(1); // space before meta
        if right_max > 0 {
            let vis_len = visible_len(right_meta);
            right_part.push(' ');
            if vis_len <= right_max {
                right_part.push_str(right_meta);
                right_vis = 1 + vis_len;
            } else {
                let ell = if right_max >= 1 { 1 } else { 0 };
                let trunc = truncate_visible(right_meta, right_max.saturating_sub(ell));
                right_part.push_str(&trunc);
                if ell == 1 {
                    right_part.push('…');
                }
                right_vis = visible_len(&right_part);
            }
        }
    }
    let bar_len = inner_width.saturating_sub(left_len + right_vis);

    let acc = accent_on();
    let mut out = String::with_capacity(width);
    // left corner
    out.push_str(&acc);
    out.push_str(&left_border);
    out.push_str(&format!("{Reset}"));
    // left text
    out.push_str(&left_seg);
    // bar
    out.push_str(&acc);
    for _ in 0..bar_len {
        out.push(chars.horiz);
    }
    out.push_str(&format!("{Reset}"));
    // right meta (already prefixed)
    out.push_str(&right_part);
    // right corner
    out.push_str(&acc);
    out.push_str(&right_border);
    out.push_str(&format!("{Reset}"));

    out
}

/// Compose a full box as a single string with newlines.
pub fn compose_box(
    left_seg: &str,
    right_meta: &str,
    body_lines: &[&str],
    bottom_right_meta: &str,
    width: usize,
) -> String {
    let mut out = String::new();
    out.push_str(&box_top_line(left_seg, right_meta, width));
    out.push('\n');
    for l in body_lines {
        out.push_str(&box_body_row(l, width));
        out.push('\n');
    }
    out.push_str(&box_empty_row(width));
    out.push('\n');
    out.push_str(&box_bottom_line(bottom_right_meta, width));
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_top_line_shapes_correctly() {
        // Use builder to exercise configuration API
        TableStyleBuilder::new()
            .accent_fg(AnsiColor::Cyan)
            .inside_pad(1)
            .apply();
        let width = 40usize;
        let left = "00:00:00  ERROR";
        let right = "app.rs:12";
        let line = box_top_line(left, right, width);
        let plain = strip_ansi(&line);
        assert_eq!(plain.chars().count(), width);
        assert!(plain.starts_with('┌'));
        assert!(plain.ends_with('┐'));
    }

    #[test]
    fn body_and_bottom_lines_have_borders_and_padding() {
        // Customize chars to exercise TableChars usage
        let chars = TableChars {
            horiz: '╌',
            ..TableChars::default()
        };
        set_table_style(TableStyle {
            chars,
            ..TableStyle::default()
        });
        let width = 24usize;
        let body = box_body_row("hello", width);
        let plain_body = strip_ansi(&body);
        assert_eq!(plain_body.chars().next(), Some('│'));
        assert_eq!(plain_body.chars().last(), Some('│'));
        assert_eq!(plain_body.chars().count(), width);

        let bot = box_bottom_line("meta", width);
        let plain_bot = strip_ansi(&bot);
        assert!(plain_bot.starts_with('└'));
        assert!(plain_bot.ends_with('┘'));
        assert_eq!(plain_bot.chars().count(), width);
    }

    #[test]
    fn compose_box_has_correct_width_and_corners() {
        let width = 50usize;
        let out = compose_box("L", "R", &["line1", "line2"], "meta", width);
        for (i, line) in strip_ansi(&out).lines().enumerate() {
            assert_eq!(line.chars().count(), width, "line {i} width mismatch");
        }
        let plain = strip_ansi(&out);
        let mut it = plain.lines();
        let top = it.next().unwrap();
        assert!(top.starts_with('┌') && top.ends_with('┐'));
        let bottom = plain.lines().last().unwrap();
        assert!(bottom.starts_with('└') && bottom.ends_with('┘'));
    }
}
