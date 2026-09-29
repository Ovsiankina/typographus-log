use crate::typographus::{clock::now_hms, logger::level_pill_for, syntax, table};
use anstyle::{AnsiColor, Effects, Reset, Style};
use std::io::{self, BufRead, BufReader};
use std::{fs::File, path::Path};
use tracing::Level;

#[derive(Clone, Copy, Debug)]
pub struct PanicStyle {
    /// Text style for focus line (fg/effects).
    pub focus_fg: AnsiColor,
    pub focus_bold: bool,
    /// Text style for context lines (fg only; dim-like).
    pub context_fg: AnsiColor,
    /// Table width used for formatting.
    pub width: usize,
    /// How many context lines around focus to show.
    pub context: usize,
    /// Enable simple syntax highlighting for code context.
    pub highlight: bool,
    /// Language hint for highlighter; auto-guessed by extension if `Auto` not available.
    pub language: syntax::Language,
    /// Label shown near time (default foreground only). Example: "ERROR".
    pub label_text: &'static str,
    pub label_fg: AnsiColor,
    pub label_bold: bool,
    /// Optional background color for the label. None by default.
    pub label_bg: Option<AnsiColor>,
    /// Pointer symbol to mark the focus line.
    pub pointer: &'static str,
    /// Use the global logger pill for the label (background style),
    /// instead of the foreground-only label fields above.
    pub use_logger_pill: bool,
}

impl Default for PanicStyle {
    fn default() -> Self {
        Self {
            focus_fg: AnsiColor::White,
            focus_bold: true,
            context_fg: AnsiColor::BrightBlack,
            width: 80,
            context: 8,
            highlight: true,
            language: syntax::Language::Auto,
            label_text: "ERROR",
            label_fg: AnsiColor::Red,
            label_bold: true,
            label_bg: None,
            pointer: "->",
            use_logger_pill: false,
        }
    }
}

/// Format a code context from any reader, highlighting the given line.
pub fn format_code_context_from_reader<R: BufRead>(
    mut reader: R,
    line: u32,
    ctx: usize,
    style: PanicStyle,
) -> io::Result<String> {
    let start = line.saturating_sub(ctx as u32).max(1);
    let end = line + ctx as u32;

    let mut out = String::new();
    let mut idx: u32 = 1;
    let mut buf = String::new();
    loop {
        buf.clear();
        let n = reader.read_line(&mut buf)?;
        if n == 0 {
            break;
        }
        // Trim ending newlines for consistent rendering
        let l = buf.trim_end_matches(['\n', '\r']);
        if idx >= start && idx <= end {
            let number = format!("{idx:>6}");
            let content_line = if style.highlight {
                let lang = style.language;
                syntax::highlight_line(lang, l, syntax::SyntaxStyle::default())
            } else {
                l.to_string()
            };
            if idx == line {
                let mut st = Style::new().fg_color(Some(style.focus_fg.into()));
                if style.focus_bold {
                    st = st.effects(Effects::BOLD);
                }
                let seg = format!("{st}{} {number}  {content_line}{Reset}\n", style.pointer);
                out.push_str(&seg);
            } else {
                let st = Style::new().fg_color(Some(style.context_fg.into()));
                let seg = format!("{st}   {number}  {content_line}{Reset}\n");
                out.push_str(&seg);
            }
        }
        if idx > end {
            break;
        }
        idx += 1;
    }
    if out.ends_with('\n') {
        out.pop();
    }
    Ok(out)
}

pub fn format_code_context(
    path: &Path,
    line: u32,
    ctx: usize,
    mut style: PanicStyle,
) -> io::Result<String> {
    // If language is Auto, guess from file extension
    if matches!(style.language, syntax::Language::Auto) {
        style.language = syntax::guess_language_from_path(path);
    }
    let f = File::open(path)?;
    let r = BufReader::new(f);
    format_code_context_from_reader(r, line, ctx, style)
}

/// Render a full panic block into a single String (newline-separated lines).
fn make_label(style: &PanicStyle) -> String {
    let mut st = Style::new().fg_color(Some(style.label_fg.into()));
    if style.label_bold {
        st = st.effects(Effects::BOLD);
    }
    if let Some(bg) = style.label_bg {
        st = st.bg_color(Some(bg.into()));
    }
    format!("{st}{}{Reset}", style.label_text)
}

pub fn render_panic_block(file: &str, line: u32, msg: &str, style: PanicStyle) -> String {
    let width = style.width;
    let time = now_hms();
    let ts = Style::new()
        .fg_color(Some(AnsiColor::BrightBlack.into()))
        .render()
        .to_string();
    let time_seg = format!("{ts}{time}{Reset}");
    let label = if style.use_logger_pill {
        level_pill_for(Level::ERROR)
    } else {
        make_label(&style)
    };
    let left = format!("{time_seg}  {label}");
    let right = format!("{file}:{line}");

    let mut out = String::new();
    // Top
    out.push_str(&table::box_top_line(&left, &right, width));
    out.push('\n');

    // Message line (if any)
    if !msg.is_empty() {
        let st = Style::new()
            .fg_color(Some(AnsiColor::White.into()))
            .render()
            .to_string();
        let body = format!("{st}{msg}{Reset}");
        out.push_str(&table::box_body_row(&body, width));
        out.push('\n');
    }

    // Code context: try to load, otherwise print a message.
    match format_code_context(Path::new(file), line, style.context, style) {
        Ok(ctxs) => {
            for l in ctxs.split('\n') {
                out.push_str(&table::box_body_row(l, width));
                out.push('\n');
            }
        }
        Err(e) => {
            let st = Style::new()
                .fg_color(Some(AnsiColor::White.into()))
                .render()
                .to_string();
            let msg = format!("{st}(failed to load source context: {e}){Reset}");
            out.push_str(&table::box_body_row(&msg, width));
            out.push('\n');
        }
    }

    // Empty spacer row
    out.push_str(&table::box_empty_row(width));
    out.push('\n');
    // Bottom with hint
    out.push_str(&table::box_bottom_line(
        "panic; set RUST_BACKTRACE=1 for backtrace",
        width,
    ));
    out
}

/// Auto-size the panic block using terminal width if available and longest content.
/// If the terminal is very small (< 40 cols), returns a short notice instead of a block.
pub fn render_panic_block_auto(file: &str, line: u32, msg: &str, mut style: PanicStyle) -> String {
    // Build context to measure longest line length
    let ctx_res = format_code_context(std::path::Path::new(file), line, style.context, style);
    let ctx_str = match &ctx_res {
        Ok(s) => s.as_str(),
        Err(_) => "",
    };
    // Compute desired width from content
    let mut max_content = 0usize;
    fn strip_ansi_local(s: &str) -> String {
        let mut out: Vec<u8> = Vec::with_capacity(s.len());
        let mut bytes = s.as_bytes().iter().copied().peekable();
        while let Some(b) = bytes.next() {
            if b == 0x1b && matches!(bytes.peek(), Some(b'[')) {
                let _ = bytes.next();
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
    for l in ctx_str.lines() {
        let vlen = strip_ansi_local(l).chars().count();
        if vlen > max_content {
            max_content = vlen;
        }
    }
    max_content = max_content.max(msg.len());

    // Borders and paddings: roughly add 6 for corners/spaces
    let desired = (max_content + 6).max(40);
    if let Some(term) = table::terminal_width() {
        if term < 40 {
            return "(terminal too small to render panic block)".to_string();
        }
        style.width = term; // span full terminal width
    } else {
        if desired < 40 {
            return "(terminal too small to render panic block)".to_string();
        }
        style.width = desired;
    }
    render_panic_block(file, line, msg, style)
}

/// Install a panic hook that prints a styled panic block to stderr.
pub fn install_panic_hook(style: PanicStyle) {
    std::panic::set_hook(Box::new(move |info| {
        let loc = info.location();
        let mut msg = String::new();
        if let Some(s) = info.payload().downcast_ref::<&str>() {
            msg = s.to_string();
        } else if let Some(s) = info.payload().downcast_ref::<String>() {
            msg = s.clone();
        }

        let rendered = if let Some(loc) = loc {
            render_panic_block_auto(loc.file(), loc.line(), &msg, style)
        } else {
            render_panic_block_auto("<unknown>", 0, &msg, style)
        };
        eprintln!("{}", crate::typographus::ansi::finish(rendered));
    }));
}

#[cfg(test)]
mod tests {
    use super::*;

    fn strip_ansi(s: &str) -> String {
        let mut out: Vec<u8> = Vec::with_capacity(s.len());
        let mut bytes = s.as_bytes().iter().copied().peekable();
        while let Some(b) = bytes.next() {
            if b == 0x1b && matches!(bytes.peek(), Some(b'[')) {
                let _ = bytes.next();
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

    #[test]
    fn code_context_marks_focus_and_dims_context() {
        let src = "line1\nline2\nline3\nline4\nline5\n";
        let mut r = BufReader::new(src.as_bytes());
        let st = PanicStyle::default();
        let out = format_code_context_from_reader(&mut r, 3, 1, st).unwrap();
        let plain = strip_ansi(&out);
        assert!(plain.contains("->      3  line3"));
        assert!(plain.contains("      2  line2"));
        assert!(plain.contains("      4  line4"));
    }

    #[test]
    fn render_block_has_borders_and_sections() {
        let st = PanicStyle {
            width: 60,
            ..PanicStyle::default()
        };
        let block = render_panic_block("<unknown>", 0, "boom", st);
        let plain = strip_ansi(&block);
        let mut lines = plain.lines();
        let top = lines.next().unwrap();
        assert!(top.starts_with('┌') && top.ends_with('┐'));
        // Message row next
        let msg = lines.next().unwrap();
        assert!(msg.starts_with("│ "));
        // there should be a bottom with corners
        let bottom = plain.lines().last().unwrap();
        assert!(bottom.starts_with('└') && bottom.ends_with('┘'));
    }

    #[test]
    fn file_context_loads_when_file_exists() {
        let content = "a\nb\nc\nd\ne\n";
        let dir = std::env::temp_dir();
        let path = dir.join("typographus_test_ctx.txt");
        std::fs::write(&path, content).unwrap();
        let st = PanicStyle {
            context: 1,
            width: 40,
            ..PanicStyle::default()
        };
        let block = render_panic_block(path.to_str().unwrap(), 3, "", st);
        let plain = strip_ansi(&block);
        assert!(plain.contains("      3  c"));
        assert!(plain.contains("      2  b"));
        assert!(plain.contains("      4  d"));
        let _ = std::fs::remove_file(&path);
    }

    #[test]
    fn auto_width_uses_terminal_columns_and_truncates() {
        // Set fake terminal width
        let prev = std::env::var("COLUMNS").ok();
        unsafe {
            std::env::set_var("COLUMNS", "72");
        }
        let st = PanicStyle {
            context: 0,
            width: 10,
            ..PanicStyle::default()
        };
        let block = render_panic_block_auto("<unknown>", 0, &"x".repeat(200), st);
        let plain = strip_ansi(&block);
        let first = plain.lines().next().unwrap();
        assert_eq!(first.chars().count(), 72);
        // Clean up
        if let Some(val) = prev {
            unsafe {
                std::env::set_var("COLUMNS", val);
            }
        } else {
            unsafe {
                std::env::remove_var("COLUMNS");
            }
        }
    }
}
