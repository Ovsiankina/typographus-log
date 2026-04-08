use log::LevelFilter;
use typographus_log::logger::{Logger, set_pill_width};

fn main() {
    set_pill_width(12);
    Logger::init_logger(LevelFilter::Trace);
    log::error!("wider pills keep columns steady");
    log::warn!("across all levels");
    log::info!("even for longer labels like WARNING");
}
