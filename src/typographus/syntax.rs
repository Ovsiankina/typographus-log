use anstyle::{AnsiColor, Reset, Style};

#[derive(Clone, Copy, Debug)]
pub struct SyntaxStyle {
    pub keyword_fg: AnsiColor,
    pub string_fg: AnsiColor,
    pub comment_fg: AnsiColor,
    pub number_fg: AnsiColor,
}

impl Default for SyntaxStyle {
    fn default() -> Self {
        Self {
            keyword_fg: AnsiColor::Cyan,
            string_fg: AnsiColor::Green,
            comment_fg: AnsiColor::BrightBlack,
            number_fg: AnsiColor::Magenta,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Language {
    Plain,
    Rust,
    Auto,
}

/// Very small, zero-dep highlighter.
/// - Colors line comments starting at `//`.
/// - Colors string segments delimited by `"` (basic escapes for `\\` and `\"`).
/// - Highlights Rust-like keywords/numbers in non-comment, non-string segments.
pub fn highlight_line(lang: Language, line: &str, style: SyntaxStyle) -> String {
    // Treat Auto as Plain here; callers should map Auto to a concrete language where possible.
    if matches!(lang, Language::Plain | Language::Auto) {
        return line.to_string();
    }
    let (kw, st, cm, num) = (
        Style::new()
            .fg_color(Some(style.keyword_fg.into()))
            .render()
            .to_string(),
        Style::new()
            .fg_color(Some(style.string_fg.into()))
            .render()
            .to_string(),
        Style::new()
            .fg_color(Some(style.comment_fg.into()))
            .render()
            .to_string(),
        Style::new()
            .fg_color(Some(style.number_fg.into()))
            .render()
            .to_string(),
    );

    // First, split off comment if present (// not in a string is simplistic but sufficient for demo)
    let mut out = String::with_capacity(line.len() + 16);
    let mut chars = line.chars().peekable();

    // Scan string/comment minimally
    let mut in_string = false;
    while let Some(c) = chars.next() {
        if in_string {
            if c == '"' {
                out.push_str(&format!("{st}\"{Reset}"));
                in_string = false;
            } else if c == '\\' {
                // emit escape and the next char in string style
                if let Some(n) = chars.next() {
                    out.push_str(&format!("{st}\\{n}{Reset}"));
                } else {
                    out.push_str(&format!("{st}\\{Reset}"));
                }
            } else {
                out.push_str(&format!("{st}{c}{Reset}"));
            }
            continue;
        }

        // non-string
        if c == '"' {
            in_string = true;
            // start the string with the quote
            out.push_str(&format!("{st}\"{Reset}"));
            continue;
        }
        if c == '/' && matches!(chars.peek(), Some('/')) {
            // comment to end of line
            let _ = chars.next();
            out.push_str(&format!("{cm}//"));
            for ch in chars {
                out.push(ch);
            }
            out.push_str(&format!("{Reset}"));
            return out;
        }

        // simple token-level highlight
        if c.is_ascii_digit() {
            out.push_str(&format!("{num}{c}{Reset}"));
            continue;
        }
        if c.is_alphanumeric() || c == '_' {
            // collect token
            let mut tok = String::new();
            tok.push(c);
            while let Some(&nc) = chars.peek() {
                if nc.is_alphanumeric() || nc == '_' {
                    tok.push(nc);
                    let _ = chars.next();
                } else {
                    break;
                }
            }
            if is_rust_keyword(&tok) {
                out.push_str(&format!("{kw}{tok}{Reset}"));
            } else {
                out.push_str(&tok);
            }
            continue;
        }

        // passthrough
        out.push(c);
    }
    out
}

pub fn guess_language_from_path(path: &std::path::Path) -> Language {
    match path.extension().and_then(|e| e.to_str()).unwrap_or("") {
        "rs" => Language::Rust,
        _ => Language::Plain,
    }
}

fn is_rust_keyword(tok: &str) -> bool {
    matches!(
        tok,
        "fn" | "let"
            | "mut"
            | "pub"
            | "struct"
            | "enum"
            | "impl"
            | "trait"
            | "use"
            | "mod"
            | "crate"
            | "const"
            | "static"
            | "match"
            | "if"
            | "else"
            | "loop"
            | "for"
            | "while"
            | "return"
            | "break"
            | "continue"
            | "as"
            | "where"
            | "type"
            | "dyn"
            | "move"
            | "ref"
            | "super"
            | "Self"
            | "self"
    )
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
    fn highlight_preserves_text_and_inserts_ansi() {
        let line = r#"fn main() { let s = "hi"; } // comment"#;
        let styled = highlight_line(Language::Rust, line, SyntaxStyle::default());
        assert_eq!(strip_ansi(&styled), line);
        assert!(styled.contains("\u{1b}["), "expected ANSI escapes present");
    }
}
