//! Pretty, opinionated terminal output for `tracing` events and errors.
//!
//! One call sets everything up:
//!
//! ```no_run
//! fn main() -> typographus_log::error::Result<()> {
//!     typographus_log::init();
//!     tracing::info!("ready");
//!     Ok(())
//! }
//! ```
//!
//! - `tracing` events (and `log` records, bridged) print with the pill layout.
//! - Any `std::error::Error` (thiserror included) renders with its source chain,
//!   through [`error::Report`] or as a tracing field.
//! - With the `eyre` feature, `eyre::Report` uses the same look.

mod typographus;
#[cfg(feature = "eyre")]
pub use typographus::eyre_hook;
pub use typographus::{ansi, clock, error, panic, style, table, tracing_fmt};

use tracing::level_filters::LevelFilter;

/// Install the subscriber (INFO and above), `log` bridge, panic hook and
/// eyre hook. All output goes to stderr; colors follow [`ansi::enabled`].
///
/// Each part is installed independently: if the app already set its own
/// subscriber or `log` style, that part is kept and the rest still
/// installs. Calling `init` again is a no-op.
pub fn init() {
    init_with_level(LevelFilter::INFO);
}

/// Same as [`init`] with an explicit max level.
pub fn init_with_level(level: LevelFilter) {
    static ONCE: std::sync::Once = std::sync::Once::new();
    ONCE.call_once(|| {
        // Before anything spawns threads: see `clock` docs.
        clock::init();
        let subscriber = tracing_subscriber::fmt()
            .event_format(tracing_fmt::TypographusFormat)
            .with_writer(std::io::stderr)
            .with_max_level(level)
            .finish();
        // Each of these fails only if someone else got there first.
        let _ = tracing::subscriber::set_global_default(subscriber);
        let _ = tracing_log::LogTracer::init();

        panic::install_panic_hook(panic::PanicStyle::default());
        #[cfg(feature = "eyre")]
        let _ = eyre_hook::install();
    });
}
