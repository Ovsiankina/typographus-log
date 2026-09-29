use anstyle::{AnsiColor, Reset, Style};
use log::Level;
use typographus_log::{
    clock::now_hms,
    table::{self, TableStyleBuilder},
};

fn dim(s: &str) -> String {
    let st = Style::new().fg_color(Some(AnsiColor::BrightBlack.into()));
    format!("{st}{s}{Reset}")
}

fn fg_pill(level: Level) -> String {
    let (fg, label) = match level {
        Level::Error => (AnsiColor::Red, "ERROR"),
        Level::Warn => (AnsiColor::Yellow, "WARNING"),
        Level::Info => (AnsiColor::Green, "INFO"),
        Level::Debug => (AnsiColor::Blue, "DEBUG"),
        Level::Trace => (AnsiColor::Magenta, "TRACE"),
    };
    let w = 7usize.max(label.len());
    let st = Style::new().fg_color(Some(fg.into())).to_string();
    let text = format!(" {:^w$} ", label);
    format!("{st}{text}{Reset}")
}

fn main() {
    // Showcase: change border color to cyan for this example only
    TableStyleBuilder::new()
        .accent_fg(AnsiColor::Cyan)
        .inside_pad(1)
        .apply();

    let width = table::terminal_width().unwrap_or(80);
    let left = format!("{}  {}", dim(&now_hms()), fg_pill(Level::Info));
    let top = table::box_top_line(&left, "colored border", width);
    println!("{}", top);
    println!(
        "{}",
        table::box_body_row("Showcasing a custom border color", width)
    );
    println!("{}", table::box_empty_row(width));
    println!("{}", table::box_bottom_line("done", width));
}
