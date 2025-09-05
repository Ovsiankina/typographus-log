use crate::typographus::clock::{self, now_hms};
use log::{Level, LevelFilter, Log, Metadata, Record};
use owo_colors::{
    OwoColorize,
    colors::{Blue, Green, Magenta, Red, Yellow, css::LightGray},
};

static LOGGER: Logger = Logger;

pub struct Logger;

impl Logger {
    pub fn level_pill(level: Level) -> String {
        let label: String = match level {
            Level::Error => format!("{}", "ERROR".bg::<Red>().bold()),
            Level::Warn => format!("{}", "WARNING".bg::<Yellow>().bold()),
            Level::Info => format!("{}", "INFO".bg::<Green>().bold()),
            Level::Debug => format!("{}", "DEBUG".bg::<Blue>().bold()),
            Level::Trace => format!("{}", "TRACE".bg::<Magenta>().bold()),
        };
        label
    }
    pub fn init_logger(level: LevelFilter) {
        let _ = log::set_logger(&LOGGER);
        log::set_max_level(level);
    }
}

impl Log for Logger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= log::max_level()
    }
    fn log(&self, record: &Record) {
        if !self.enabled(record.metadata()) {
            return;
        }

        // Build segments (each self-resets).
        let time = &now_hms();

        // indent multiline body, then color white (self-reset).
        let indent = "  ";
        let mut body = format!("{}", record.args());
        body = body.replace('\n', &format!("\n{indent}"));

        let time = time.dimmed();
        let pill = Logger::level_pill(record.level());
        let body = body.white();

        println!("{time}{pill}{body}")
        // Hard resets between tokens so spaces are plain; start & end with Reset.
        // a_println!();
        // a_println!("{Reset}{ts}{Reset}  {pill}{Reset}  {body}{Reset}");
    }
    fn flush(&self) {}
}
