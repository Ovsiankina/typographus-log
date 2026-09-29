//! Everything at once: tracing, thiserror, eyre, log bridge, panic hook.
//!
//! ```sh
//! cargo run --example showcase --features eyre
//! ```

use eyre::WrapErr;
use std::error::Error;
use thiserror::Error;
use tracing::level_filters::LevelFilter;
use tracing::{debug, error, info, info_span, instrument, trace, warn};

// ── thiserror: typed errors with a source chain ──────────────────────────────

#[derive(Debug, Error)]
enum ConfigError {
    #[error("could not read config file `{path}`")]
    Read {
        path: String,
        #[source]
        source: std::io::Error,
    },
    #[error("key `{0}` is missing")]
    MissingKey(&'static str),
}

#[derive(Debug, Error)]
enum AppError {
    #[error("startup failed")]
    Startup(#[from] ConfigError),
}

fn read_config(path: &str) -> Result<String, ConfigError> {
    std::fs::read_to_string(path).map_err(|source| ConfigError::Read {
        path: path.into(),
        source,
    })
}

fn start() -> Result<(), AppError> {
    read_config("/definitely/not/here.toml")?;
    Ok(())
}

// ── tracing: spans + instrumented functions ──────────────────────────────────

#[instrument(fields(user = "ovsiankina"))]
fn handle_request(id: u32) {
    info!(id, "handling request");
    debug!(cache = "miss", "looking up session");
    if id.is_multiple_of(2) {
        warn!(latency_ms = 812, "slow upstream");
    }
}

fn main() -> eyre::Result<()> {
    typographus_log::init_with_level(LevelFilter::TRACE);

    // 1. Plain events, every level.
    trace!("trace message");
    debug!("debug message");
    info!("info message");
    warn!("warning message");
    error!("error message");

    // 2. Structured fields and a multiline body.
    info!(port = 8080, tls = true, "server listening");
    info!("multiline body:\nsecond line\nthird line");

    // 3. Spans: the breadcrumb shows where the event came from.
    let _app = info_span!("app").entered();
    for id in 1..=2 {
        handle_request(id);
    }
    {
        let _db = info_span!("db").entered();
        trace!(rows = 42, "query done");
    }

    // 4. Old `log` crate records go through the same formatter.
    log::info!("hello from the `log` crate");

    // 5. thiserror error logged as a tracing field: full chain rendered.
    if let Err(e) = start() {
        error!(error = &e as &dyn Error, "request aborted");
    }
    let missing = ConfigError::MissingKey("database.url");
    warn!(error = &missing as &dyn Error, "falling back to defaults");

    // 6. typographus::error::Report for bins without eyre.
    let report: typographus_log::error::Report = start().unwrap_err().into();
    eprintln!("\n-- Report {{:?}} (what `main` would print) --\n{report:?}\n");

    // 7. Panic hook, caught on a thread so the demo keeps going.
    let _ = std::thread::spawn(|| {
        let v: Vec<u8> = Vec::new();
        v[3]
    })
    .join();

    // 8. eyre: wrap context on top of a thiserror chain, return from main.
    eprintln!("\n-- eyre::Report returned from main --");
    start()
        .wrap_err("could not boot the showcase")
        .wrap_err("typographus demo exited")?;
    Ok(())
}
