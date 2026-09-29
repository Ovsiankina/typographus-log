use anstyle::AnsiColor;
use tracing::level_filters::LevelFilter;
use typographus_log::style::{Palette, Spacing, set_palette, set_spacing};

fn main() {
    typographus_log::init_with_level(LevelFilter::INFO);
    tracing::info!("defaults");

    // Update spacing at runtime
    set_spacing(Spacing {
        between_time_pill: 1,
        between_pill_body: 3,
        continuation_indent: "  ".into(),
    });
    tracing::info!("spacing changed at runtime");

    // Update palette at runtime
    let pal = Palette {
        info_bg: AnsiColor::BrightCyan,
        info_fg: AnsiColor::Black,
        ..Palette::default()
    };
    set_palette(pal);
    tracing::info!("palette changed at runtime");
}
