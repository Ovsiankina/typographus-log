use anstyle::AnsiColor;
use tracing::level_filters::LevelFilter;
use typographus_log::style::{Palette, set_palette};

fn main() {
    // Customize colors via setters before init
    let pal = Palette {
        bold_pills: true,
        time_fg: AnsiColor::BrightBlack, // dim timestamp
        body_fg: AnsiColor::BrightWhite, // brighter body
        error_fg: AnsiColor::BrightWhite,
        error_bg: AnsiColor::Red,
        warn_fg: AnsiColor::Black,
        warn_bg: AnsiColor::Yellow,
        info_fg: AnsiColor::Black,
        info_bg: AnsiColor::Cyan,
        debug_fg: AnsiColor::BrightWhite,
        debug_bg: AnsiColor::Blue,
        trace_fg: AnsiColor::BrightWhite,
        trace_bg: AnsiColor::Magenta,
    };
    set_palette(pal);

    typographus_log::init_with_level(LevelFilter::INFO);
    tracing::info!("custom palette (cyan info)");
    tracing::warn!("warns are black-on-yellow");
    tracing::error!("errors are bright white on red");
}
