use anstyle::AnsiColor;
use log::LevelFilter;
use typographus_log::logger::{Logger, Palette, set_palette};

fn main() {
    let pal = Palette {
        bold_pills: false,
        info_bg: AnsiColor::Blue, // visible even without bold
        info_fg: AnsiColor::BrightWhite,
        ..Palette::default()
    };
    set_palette(pal);

    Logger::init_logger(LevelFilter::Info);
    log::info!("no bold pills");
}
