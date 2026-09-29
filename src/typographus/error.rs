//! Pretty error rendering and a lightweight [`Report`] type.
//!
//! Works with any [`std::error::Error`] (thiserror enums included): the
//! top-level message is printed next to an `ERROR` pill, and every
//! `.source()` below it is listed as a dim "caused by" line.
//!
//! ```no_run
//! fn main() -> typographus_log::error::Result<()> {
//!     std::fs::read_to_string("missing.toml")?;
//!     Ok(())
//! }
//! ```

use crate::typographus::ansi;
use crate::typographus::logger::level_pill_for;
use anstyle::{AnsiColor, Reset, Style};
use std::error::Error;
use std::fmt;
use tracing::Level;

/// Render an error and its source chain as a styled, multi-line string.
///
/// Every colored segment self-resets.
pub fn render_error(err: &(dyn Error + 'static)) -> String {
    ansi::finish(format!(
        "{}  {}",
        level_pill_for(Level::ERROR),
        render_chain(err)
    ))
}

/// Render only the message and its "caused by" lines, without a pill.
/// Used when the pill is already drawn (tracing events, eyre handler).
pub fn render_chain(err: &(dyn Error + 'static)) -> String {
    let dim = Style::new().fg_color(Some(AnsiColor::BrightBlack.into()));
    let mut out = err.to_string();

    let mut source = err.source();
    let mut depth = 0;
    while let Some(cause) = source {
        let indent = "  ".repeat(depth + 1);
        out.push_str(&format!("\n{indent}{dim}└─ caused by:{Reset} {cause}"));
        source = cause.source();
        depth += 1;
    }
    ansi::finish(out)
}

/// Error wrapper for binaries that don't use eyre.
///
/// Any `std::error::Error` converts into it through `?`. Returning it from
/// `main` prints the pretty form, because std uses `Debug` for that. std
/// always puts `Error: ` in front; that prefix cannot be removed.
pub struct Report {
    inner: Box<dyn Error + Send + Sync + 'static>,
}

/// `Result` alias defaulting to [`Report`].
pub type Result<T, E = Report> = std::result::Result<T, E>;

impl Report {
    /// Wrap an error.
    pub fn new<E: Error + Send + Sync + 'static>(err: E) -> Self {
        Self {
            inner: Box::new(err),
        }
    }

    /// Borrow the wrapped error.
    pub fn inner(&self) -> &(dyn Error + Send + Sync + 'static) {
        &*self.inner
    }
}

// Report deliberately does not implement `Error`: that would conflict with
// the blanket `From<T> for T` (same trick as anyhow/eyre).
impl<E: Error + Send + Sync + 'static> From<E> for Report {
    fn from(err: E) -> Self {
        Self::new(err)
    }
}

impl fmt::Debug for Report {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&render_error(&*self.inner))
    }
}

impl fmt::Display for Report {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(&self.inner, f)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug)]
    struct Outer(std::io::Error);
    impl fmt::Display for Outer {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.write_str("could not load config")
        }
    }
    impl Error for Outer {
        fn source(&self) -> Option<&(dyn Error + 'static)> {
            Some(&self.0)
        }
    }

    #[test]
    fn renders_message_and_chain() {
        let err = Outer(std::io::Error::other("disk on fire"));
        let plain = crate::typographus::ansi::strip(&render_error(&err));
        let lines: Vec<_> = plain.lines().collect();
        assert_eq!(lines.len(), 2);
        assert!(lines[0].contains("ERROR") && lines[0].ends_with("could not load config"));
        assert_eq!(lines[1], "  └─ caused by: disk on fire");
    }

    #[test]
    fn report_debug_is_pretty_display_is_plain() {
        let report: Report = Outer(std::io::Error::other("x")).into();
        assert_eq!(report.to_string(), "could not load config");
        assert!(crate::typographus::ansi::strip(&format!("{report:?}")).contains("caused by: x"));
    }
}
