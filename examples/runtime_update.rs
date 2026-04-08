use anstyle::AnsiColor;
use log::LevelFilter;
use typographus_log::logger::{Logger, Palette, Spacing, set_palette, set_spacing};

fn main() {
    Logger::init_logger(LevelFilter::Info);
    log::info!("defaults");

    // Update spacing at runtime
    set_spacing(Spacing {
        between_time_pill: 1,
        between_pill_body: 3,
        continuation_indent: "  ".into(),
    });
    log::info!("spacing changed at runtime");

    // Update palette at runtime
    let pal = Palette {
        info_bg: AnsiColor::BrightCyan,
        info_fg: AnsiColor::Black,
        ..Palette::default()
    };
    set_palette(pal);
    log::info!("palette changed at runtime");
}
