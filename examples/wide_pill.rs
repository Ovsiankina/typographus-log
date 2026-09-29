use tracing::level_filters::LevelFilter;
use typographus_log::style::set_pill_width;

fn main() {
    set_pill_width(12);
    typographus_log::init_with_level(LevelFilter::TRACE);
    tracing::error!("wider pills keep columns steady");
    tracing::warn!("across all levels");
    tracing::info!("even for longer labels like WARNING");
}
