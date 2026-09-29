use anstyle::AnsiColor;
use tracing::level_filters::LevelFilter;
use typographus_log::style::{Palette, set_palette};

fn main() {
    let pal = Palette {
        bold_pills: false,
        info_bg: AnsiColor::Blue, // visible even without bold
        info_fg: AnsiColor::BrightWhite,
        ..Palette::default()
    };
    set_palette(pal);

    typographus_log::init_with_level(LevelFilter::INFO);
    tracing::info!("no bold pills");
}
