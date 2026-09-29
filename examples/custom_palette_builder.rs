use anstyle::AnsiColor;
use tracing::level_filters::LevelFilter;
use typographus_log::style::{Palette, StyleBuilder};

fn main() {
    let pal = Palette {
        time_fg: AnsiColor::BrightBlack,
        body_fg: AnsiColor::White,
        error_fg: AnsiColor::White,
        error_bg: AnsiColor::BrightRed,
        warn_fg: AnsiColor::Black,
        warn_bg: AnsiColor::BrightYellow,
        info_fg: AnsiColor::Black,
        info_bg: AnsiColor::BrightGreen,
        debug_fg: AnsiColor::BrightWhite,
        debug_bg: AnsiColor::BrightBlue,
        trace_fg: AnsiColor::BrightWhite,
        trace_bg: AnsiColor::BrightMagenta,
        bold_pills: false, // demo no-bold pills
    };

    StyleBuilder::new().palette(pal).pill_width(9).apply();

    typographus_log::init_with_level(LevelFilter::INFO);
    tracing::info!("builder-configured palette; pill width = 9");
}
