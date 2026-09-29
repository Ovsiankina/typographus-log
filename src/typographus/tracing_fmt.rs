//! `tracing` event formatter using the typographus look.
//!
//! Layout: `HH:MM:SS  [ PILL ]  span{fields} › span  message  key=value`
//! Errors are shown as a `✗` block under the line:
//! - `error = &e as &dyn Error` → message plus full source chain,
//! - `error = ?report` (typographus or eyre `Report`) → same, no second pill,
//! - `error = %e` or any field named `error`/`err` → its text,
//! - any multiline field value → indented block.

use crate::typographus::ansi;
use crate::typographus::clock::write_hms;
use crate::typographus::error::{in_field_mode, render_chain};
use crate::typographus::style::{style_read, write_level_pill};
use anstyle::{AnsiColor, Reset, Style};
use std::error::Error;
use std::fmt::{self, Write as _};
use tracing::field::{Field, Visit};
use tracing::{Event, Subscriber};
use tracing_subscriber::fmt::format::Writer;
use tracing_subscriber::fmt::{FmtContext, FormatEvent, FormatFields, FormattedFields};
use tracing_subscriber::registry::LookupSpan;

/// Event formatter plugged into `tracing_subscriber::fmt`.
#[derive(Clone, Copy, Debug, Default)]
pub struct TypographusFormat;

impl<S, N> FormatEvent<S, N> for TypographusFormat
where
    S: Subscriber + for<'a> LookupSpan<'a>,
    N: for<'a> FormatFields<'a> + 'static,
{
    fn format_event(
        &self,
        ctx: &FmtContext<'_, S, N>,
        mut writer: Writer<'_>,
        event: &Event<'_>,
    ) -> fmt::Result {
        // Visit fields first: they may format user types, which must not run
        // while we hold the style lock.
        let mut visitor = FieldVisitor::default();
        event.record(&mut visitor);

        let cfg = style_read();
        let dim = Style::new().fg_color(Some(cfg.palette.time_fg.into()));
        let body = Style::new().fg_color(Some(cfg.palette.body_fg.into()));
        let indent = cfg.spacing.continuation_indent.as_str();

        // One buffer per event so colors can be stripped in one pass.
        let mut line = String::with_capacity(128);
        write!(line, "{dim}")?;
        write_hms(&mut line)?;
        write!(line, "{Reset}{:1$}", "", cfg.spacing.between_time_pill)?;
        write_level_pill(&mut line, *event.metadata().level(), &cfg)?;
        write!(line, "{:1$}", "", cfg.spacing.between_pill_body)?;

        // Span breadcrumb, root first, with each span's fields.
        if let Some(scope) = ctx.event_scope() {
            let mut first = true;
            for span in scope.from_root() {
                if !first {
                    write!(line, "{dim} › {Reset}")?;
                }
                first = false;
                write!(line, "{dim}{}", span.name())?;
                let ext = span.extensions();
                if let Some(fields) = ext.get::<FormattedFields<N>>()
                    && !fields.is_empty()
                {
                    write!(line, "{{{fields}}}")?;
                }
                write!(line, "{Reset}")?;
            }
            if !first {
                line.push_str("  ");
            }
        }

        write!(line, "{body}")?;
        push_indented(&mut line, &visitor.message, indent);
        write!(line, "{Reset}")?;

        for (key, value) in &visitor.fields {
            write!(line, "  {dim}{key}={Reset}")?;
            push_indented(&mut line, value, indent);
        }
        let red = Style::new().fg_color(Some(AnsiColor::Red.into()));
        for chain in &visitor.errors {
            write!(line, "\n{indent}{red}✗{Reset} ")?;
            push_indented(&mut line, chain, indent);
        }
        drop(cfg);

        if ansi::enabled() {
            writeln!(writer, "{line}")
        } else {
            writeln!(writer, "{}", ansi::strip(&line))
        }
    }
}

/// Append `s`, putting `indent` after every newline.
fn push_indented(out: &mut String, s: &str, indent: &str) {
    let mut parts = s.split('\n');
    if let Some(first) = parts.next() {
        out.push_str(first);
    }
    for part in parts {
        out.push('\n');
        out.push_str(indent);
        out.push_str(part);
    }
}

fn is_error_name(name: &str) -> bool {
    matches!(name, "error" | "err")
}

#[derive(Default)]
struct FieldVisitor {
    message: String,
    fields: Vec<(&'static str, String)>,
    errors: Vec<String>,
}

impl Visit for FieldVisitor {
    fn record_str(&mut self, field: &Field, value: &str) {
        // Metadata injected by the log → tracing bridge; noise in a terminal.
        if field.name().starts_with("log.") {
            return;
        }
        if field.name() == "message" {
            self.message.push_str(value);
        } else if is_error_name(field.name()) {
            self.errors.push(value.to_string());
        } else {
            self.fields.push((field.name(), value.to_string()));
        }
    }

    fn record_error(&mut self, _field: &Field, value: &(dyn Error + 'static)) {
        self.errors.push(render_chain(value));
    }

    fn record_debug(&mut self, field: &Field, value: &dyn fmt::Debug) {
        let name = field.name();
        if name.starts_with("log.") {
            return;
        }
        if name == "message" {
            let _ = write!(self.message, "{value:?}");
            return;
        }
        let text = in_field_mode(|| format!("{value:?}"));
        // `error = ?report`, `err = %e`, or anything multiline: own block.
        if is_error_name(name) || text.contains('\n') {
            self.errors.push(text);
        } else {
            self.fields.push((name, text));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::typographus::ansi::strip;
    use std::sync::{Arc, Mutex};
    use tracing_subscriber::fmt::MakeWriter;

    #[derive(Clone, Default)]
    struct Buf(Arc<Mutex<Vec<u8>>>);
    impl std::io::Write for Buf {
        fn write(&mut self, b: &[u8]) -> std::io::Result<usize> {
            self.0.lock().unwrap().extend_from_slice(b);
            Ok(b.len())
        }
        fn flush(&mut self) -> std::io::Result<()> {
            Ok(())
        }
    }
    impl<'a> MakeWriter<'a> for Buf {
        type Writer = Buf;
        fn make_writer(&'a self) -> Buf {
            self.clone()
        }
    }

    /// Run `f` under a TypographusFormat subscriber, return stripped output.
    fn capture(f: impl FnOnce()) -> String {
        let _g = crate::typographus::style::test_style_lock();
        crate::typographus::style::StyleBuilder::new().apply();
        let buf = Buf::default();
        let sub = tracing_subscriber::fmt()
            .event_format(TypographusFormat)
            .with_writer(buf.clone())
            .with_max_level(tracing::Level::TRACE)
            .finish();
        tracing::subscriber::with_default(sub, f);
        strip(&String::from_utf8(buf.0.lock().unwrap().clone()).unwrap())
    }

    #[derive(Debug)]
    struct Outer(std::io::Error);
    impl fmt::Display for Outer {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.write_str("outer failed")
        }
    }
    impl Error for Outer {
        fn source(&self) -> Option<&(dyn Error + 'static)> {
            Some(&self.0)
        }
    }
    fn outer() -> Outer {
        Outer(std::io::Error::other("inner cause"))
    }

    #[test]
    fn dyn_error_field_renders_chain_block() {
        let out = capture(|| {
            let e = outer();
            tracing::error!(error = &e as &dyn Error, "boom");
        });
        let lines: Vec<_> = out.lines().collect();
        assert!(lines[0].ends_with("boom"), "{out}");
        assert_eq!(lines[1], "  ✗ outer failed");
        assert!(lines[2].ends_with("└─ caused by: inner cause"), "{out}");
    }

    #[test]
    fn debug_report_field_has_no_second_pill() {
        let out = capture(|| {
            let r: crate::typographus::error::Report = outer().into();
            tracing::warn!(error = ?r, "degraded");
        });
        assert_eq!(out.matches("ERROR").count(), 0, "{out}");
        assert!(out.contains("✗ outer failed"), "{out}");
        assert!(out.contains("caused by: inner cause"), "{out}");
    }

    #[test]
    fn display_error_field_goes_to_block() {
        let out = capture(|| tracing::info!(err = %outer(), "x"));
        assert!(
            out.lines().nth(1).unwrap().ends_with("✗ outer failed"),
            "{out}"
        );
    }

    #[test]
    fn plain_fields_inline_multiline_indented() {
        let out = capture(|| {
            tracing::info!(port = 8080, "a\nb");
            tracing::info!(dump = "l1\nl2", "m");
        });
        let lines: Vec<_> = out.lines().collect();
        assert!(lines[0].ends_with("a"), "{out}");
        assert_eq!(lines[1], "  b  port=8080");
        assert!(lines[2].ends_with("m  dump=l1"), "{out}");
        assert_eq!(lines[3], "  l2");
    }

    #[test]
    fn span_breadcrumb_includes_fields() {
        let out = capture(|| {
            let _s = tracing::info_span!("req", id = 3).entered();
            tracing::info!("hi");
        });
        assert!(out.contains("req{id=3}  hi"), "{out}");
    }
}
