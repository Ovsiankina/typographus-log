//! `tracing` event formatter using the typographus look.
//!
//! Layout: `HH:MM:SS  [ PILL ]  span{fields} › span  message  key=value`
//! Errors recorded as fields (`error = &e as &dyn Error`) are expanded with
//! their full source chain on the following lines.

use crate::typographus::ansi;
use crate::typographus::clock::write_hms;
use crate::typographus::error::render_chain;
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
        } else {
            self.fields.push((field.name(), value.to_string()));
        }
    }

    fn record_error(&mut self, _field: &Field, value: &(dyn Error + 'static)) {
        self.errors.push(render_chain(value));
    }

    fn record_debug(&mut self, field: &Field, value: &dyn fmt::Debug) {
        if field.name().starts_with("log.") {
            return;
        }
        if field.name() == "message" {
            let _ = write!(self.message, "{value:?}");
        } else {
            self.fields.push((field.name(), format!("{value:?}")));
        }
    }
}
