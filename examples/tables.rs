use anstyle::{AnsiColor, Reset, Style};
use tracing::Level;
use typographus_log::{
    clock::now_hms,
    logger::level_pill_fg,
    table::{self, TableStyleBuilder},
};

fn dim(s: &str) -> String {
    let st = Style::new().fg_color(Some(AnsiColor::BrightBlack.into()));
    format!("{st}{s}{Reset}")
}

fn render_box(level: Level, right_meta: &str, body_lines: &[&str], bottom_right: &str) {
    // Dim borders to match panic style; span full terminal width if known
    TableStyleBuilder::new()
        .accent_fg(AnsiColor::BrightBlack)
        .inside_pad(1)
        .bottom_left_text(None)
        .apply();

    let width = table::terminal_width().unwrap_or(80);
    let time = dim(&now_hms());
    let pill = level_pill_fg(level);
    let left = format!("{time}  {pill}");
    let block = table::compose_box(&left, right_meta, body_lines, bottom_right, width);
    println!("{}", block);
}

fn main() {
    // INFO
    render_box(
        Level::INFO,
        "route=/ home",
        &["Welcome to the app", "Rendering home page"],
        "ok",
    );

    // WARN with a bottom-left note
    render_box(
        Level::WARN,
        "user=guest",
        &[
            "Configuration near limits",
            "Consider increasing cache size",
        ],
        "throttle=near-limit",
    );

    // ERROR
    render_box(
        Level::ERROR,
        "controllers/app.rs:42",
        &[
            "Failed to process request",
            "Runtime error: Database unreachable",
        ],
        "status=500",
    );

    // DEBUG
    render_box(
        Level::DEBUG,
        "debug-session=on",
        &["State before: x=1", "State after: x=2"],
        "trace_id=abcd",
    );

    // TRACE
    render_box(
        Level::TRACE,
        "path=/api/data",
        &[
            "Entering handler",
            "Fetching records",
            "Serializing response",
        ],
        "done",
    );
}
