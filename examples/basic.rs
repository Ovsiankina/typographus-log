use log::LevelFilter;
use typographus_log::logger::Logger;

fn main() {
    Logger::init_logger(LevelFilter::Trace);
    log::error!("error message");
    log::warn!("warning message");
    log::info!("info message");
    log::debug!("debug message");
    log::trace!("trace message");
}
