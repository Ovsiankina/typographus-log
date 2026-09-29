//! Color on/off decision, made once for all typographus output.
//!
//! Default: on only when stderr is a terminal and `NO_COLOR` is unset.

use once_cell::sync::Lazy;
use std::io::IsTerminal;
use std::sync::atomic::{AtomicBool, Ordering};

static ENABLED: Lazy<AtomicBool> = Lazy::new(|| {
    let on = std::io::stderr().is_terminal() && std::env::var_os("NO_COLOR").is_none();
    AtomicBool::new(on)
});

/// Whether typographus output should contain ANSI escapes.
pub fn enabled() -> bool {
    ENABLED.load(Ordering::Relaxed)
}

/// Force colors on or off (e.g. from a `--color` CLI flag).
pub fn set_enabled(on: bool) {
    ENABLED.store(on, Ordering::Relaxed);
}

/// Remove `ESC [ ... m` sequences.
pub fn strip(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    let mut chars = s.chars();
    while let Some(c) = chars.next() {
        if c == '\x1b' {
            for c in chars.by_ref() {
                if c == 'm' {
                    break;
                }
            }
        } else {
            out.push(c);
        }
    }
    out
}

/// Return `s` unchanged when colors are on, stripped otherwise.
pub(crate) fn finish(s: String) -> String {
    if enabled() { s } else { strip(&s) }
}

#[cfg(test)]
mod tests {
    use super::strip;

    #[test]
    fn strip_removes_sgr_only() {
        assert_eq!(strip("\x1b[1m\x1b[31mhi\x1b[0m there"), "hi there");
    }
}
