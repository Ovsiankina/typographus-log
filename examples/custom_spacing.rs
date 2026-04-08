use log::LevelFilter;
use typographus_log::logger::{Logger, Spacing, set_spacing};

fn main() {
    set_spacing(Spacing {
        between_time_pill: 3,
        between_pill_body: 1,
        continuation_indent: "--> ".into(),
    });

    Logger::init_logger(LevelFilter::Trace);
    log::info!("single-line body");
    log::debug!("multi-line\nbody\nwith custom indent");
}
