use tracing::level_filters::LevelFilter;

fn main() {
    typographus_log::init_with_level(LevelFilter::TRACE);
    tracing::error!("error message");
    tracing::warn!("warning message");
    tracing::info!("info message");
    tracing::debug!("debug message");
    tracing::trace!("trace message");
}
