use once_cell::sync::Lazy;
use time::{
    OffsetDateTime,
    format_description::{self, FormatItem},
};

static TIME_FMT: Lazy<Vec<FormatItem<'static>>> =
    Lazy::new(|| format_description::parse("[hour]:[minute]:[second]").expect("time fmt"));

pub fn now_hms() -> String {
    OffsetDateTime::now_local()
        .unwrap_or_else(|_| OffsetDateTime::now_utc())
        .format(&TIME_FMT)
        .unwrap_or_else(|_| "??:??:??".into())
}
