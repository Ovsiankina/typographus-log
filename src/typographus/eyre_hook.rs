//! eyre integration (feature `eyre`).
//!
//! Installs a global [`eyre::EyreHandler`] so any `eyre::Report` returned
//! from `main` (or printed with `{:?}`) uses the typographus look. User code
//! stays plain eyre.

use crate::typographus::ansi;
use crate::typographus::error::{is_field_mode, render_chain, render_error};
use anstyle::{AnsiColor, Reset, Style};
use std::error::Error;
use std::fmt;
use std::panic::Location;

/// Handler attached to every `eyre::Report` once installed.
#[derive(Debug, Default)]
pub struct TypographusHandler {
    location: Option<&'static Location<'static>>,
}

impl eyre::EyreHandler for TypographusHandler {
    fn debug(&self, error: &(dyn Error + 'static), f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if f.alternate() {
            return fmt::Debug::fmt(error, f);
        }
        if is_field_mode() {
            return f.write_str(&render_chain(error));
        }
        f.write_str(&render_error(error))?;
        if let Some(loc) = self.location {
            let dim = Style::new().fg_color(Some(AnsiColor::BrightBlack.into()));
            let at = format!("\n  {dim}at {}:{}{Reset}", loc.file(), loc.line());
            f.write_str(&ansi::finish(at))?;
        }
        Ok(())
    }

    fn track_caller(&mut self, location: &'static Location<'static>) {
        self.location = Some(location);
    }
}

/// Install the handler. Must run before the first `eyre::Report` is built.
/// Returns an error if another hook was already installed.
pub fn install() -> Result<(), eyre::InstallError> {
    eyre::set_hook(Box::new(|_| Box::new(TypographusHandler::default())))
}
