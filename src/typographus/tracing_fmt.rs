//! `tracing` event formatter using the typographus look.
//!
//! Layout: `HH:MM:SS  [ PILL ]  span{fields} › span  message  key=value`
//! Errors recorded as fields (`error = &e as &dyn Error`) are expanded with
//! their full source chain on the following lines.

use crate::typographus::ansi;
use crate::typographus::clock::now_hms;
use crate::typographus::error::render_chain;
use crate::typographus::logger::{level_pill_for, style_snapshot};
use anstyle::{AnsiColor, Reset, Style};
use std::error::Error;
use std::fmt::{self, Write as _};
use tracing::field::{Field, Visit};
use tracing::{Event, Level, Subscriber};
use tracing_subscriber::fmt::format::Writer;
use tracing_subscriber::fmt::{FmtContext, FormatEvent, FormatFields, FormattedFields};
use tracing_subscriber::registry::LookupSpan;

/// Event formatter plugged into `tracing_subscriber::fmt`.
#[derive(Clone, Copy, Debug, Default)]
pub struct TypographusFormat;

fn to_log_level(level: &Level) -> log::Level {
    match *level {
        Level::ERROR => log::Level::Error,
        Level::WARN => log::Level::Warn,
        Level::INFO => log::Level::Info,
        Level::DEBUG => log::Level::Debug,
        Level::TRACE => log::Level::Trace,
    }
}

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
        let cfg = style_snapshot();
        let dim = Style::new().fg_color(Some(cfg.palette.time_fg.into()));
        let body = Style::new().fg_color(Some(cfg.palette.body_fg.into()));
        let s1 = " ".repeat(cfg.spacing.between_time_pill);
        let s2 = " ".repeat(cfg.spacing.between_pill_body);
        let indent = &cfg.spacing.continuation_indent;
        let nl_indent = format!("\n{indent}");

        // Built in one buffer so colors can be stripped in one pass.
        let mut line = String::with_capacity(128);
        let pill = level_pill_for(to_log_level(event.metadata().level()));
        write!(line, "{dim}{}{Reset}{s1}{pill}{s2}", now_hms())?;

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

        let mut visitor = FieldVisitor::default();
        event.record(&mut visitor);

        let message = visitor.message.replace('\n', &nl_indent);
        write!(line, "{body}{message}{Reset}")?;

        for (key, value) in &visitor.fields {
            let value = value.replace('\n', &nl_indent);
            write!(line, "  {dim}{key}={Reset}{value}")?;
        }
        let red = Style::new().fg_color(Some(AnsiColor::Red.into()));
        for chain in &visitor.errors {
            let chain = chain.replace('\n', &nl_indent);
            write!(line, "{nl_indent}{red}✗{Reset} {chain}")?;
        }
        writeln!(writer, "{}", ansi::finish(line))
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
