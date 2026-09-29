use tracing::level_filters::LevelFilter;
use typographus_log::logger::{Spacing, set_spacing};

fn main() {
    set_spacing(Spacing {
        between_time_pill: 3,
        between_pill_body: 1,
        continuation_indent: "--> ".into(),
    });

    typographus_log::init_with_level(LevelFilter::TRACE);
    tracing::info!("single-line body");
    tracing::debug!("multi-line\nbody\nwith custom indent");
}
