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
