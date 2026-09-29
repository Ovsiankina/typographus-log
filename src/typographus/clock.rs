//! Wall-clock timestamps for log lines.
//!
//! The local UTC offset is resolved once and cached. On Unix, `time` refuses
//! to read it once the process has several threads (the libc call isn't
//! thread-safe), so [`init`] should run early in `main`;
//! `typographus_log::init()` does this for you.

use once_cell::sync::Lazy;
use std::sync::OnceLock;
use time::{
    OffsetDateTime, UtcOffset,
    format_description::{self, FormatItem},
};

static TIME_FMT: Lazy<Vec<FormatItem<'static>>> =
    Lazy::new(|| format_description::parse("[hour]:[minute]:[second]").expect("time fmt"));

static LOCAL_OFFSET: OnceLock<UtcOffset> = OnceLock::new();

/// Resolve and cache the local offset. Falls back to UTC if it can't be
/// determined (e.g. already multi-threaded on Unix). Idempotent.
pub fn init() {
    LOCAL_OFFSET.get_or_init(|| UtcOffset::current_local_offset().unwrap_or(UtcOffset::UTC));
}

/// Current local time as `HH:MM:SS`.
pub fn now_hms() -> String {
    init();
    let offset = *LOCAL_OFFSET.get().unwrap_or(&UtcOffset::UTC);
    OffsetDateTime::now_utc()
        .to_offset(offset)
        .format(&TIME_FMT)
        .unwrap_or_else(|_| "??:??:??".into())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn is_two_digits(s: &str) -> bool {
        s.len() == 2 && s.chars().all(|c| c.is_ascii_digit())
    }

    #[test]
    fn now_hms_has_expected_shape() {
        let s = now_hms();
        assert_eq!(s.len(), 8, "expected HH:MM:SS or '??:??:??'");
        assert_eq!(s.as_bytes()[2], b':');
        assert_eq!(s.as_bytes()[5], b':');

        let hh = &s[0..2];
        let mm = &s[3..5];
        let ss = &s[6..8];

        let valid = (is_two_digits(hh) || hh == "??")
            && (is_two_digits(mm) || mm == "??")
            && (is_two_digits(ss) || ss == "??");
        assert!(valid, "unexpected time format: {s}");
    }
}
