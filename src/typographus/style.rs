//! Style configuration: palette, spacing, pill width, and the level pills.

use anstyle::{AnsiColor, Effects, Reset, Style};
use once_cell::sync::Lazy;
use tracing::Level;

// ───────────────────── style configuration (NEW) ─────────────────────────────
#[derive(Clone, Copy, Debug)]
pub struct Palette {
    pub time_fg: AnsiColor,
    pub body_fg: AnsiColor,
    pub error_fg: AnsiColor,
    pub error_bg: AnsiColor,
    pub warn_fg: AnsiColor,
    pub warn_bg: AnsiColor,
    pub info_fg: AnsiColor,
    pub info_bg: AnsiColor,
    pub debug_fg: AnsiColor,
    pub debug_bg: AnsiColor,
    pub trace_fg: AnsiColor,
    pub trace_bg: AnsiColor,
    pub bold_pills: bool,
}

impl Default for Palette {
    fn default() -> Self {
        Self {
            time_fg: AnsiColor::BrightBlack,
            body_fg: AnsiColor::White,
            error_fg: AnsiColor::White,
            error_bg: AnsiColor::Red,
            warn_fg: AnsiColor::Black,
            warn_bg: AnsiColor::Yellow,
            info_fg: AnsiColor::Black,
            info_bg: AnsiColor::Green,
            debug_fg: AnsiColor::White,
            debug_bg: AnsiColor::Blue,
            trace_fg: AnsiColor::White,
            trace_bg: AnsiColor::Magenta,
            bold_pills: true,
        }
    }
}

#[derive(Clone, Debug)]
pub struct Spacing {
    pub between_time_pill: usize,
    pub between_pill_body: usize,
    pub continuation_indent: String,
}

impl Default for Spacing {
    fn default() -> Self {
        Self {
            between_time_pill: 2,
            between_pill_body: 2,
            continuation_indent: "  ".into(),
        }
    }
}

#[derive(Clone, Debug)]
pub struct StyleConfig {
    pub palette: Palette,
    pub spacing: Spacing,
    pub pill_width: usize, // interior width of label, to prevent jitter
}

impl Default for StyleConfig {
    fn default() -> Self {
        Self {
            palette: Palette::default(),
            spacing: Spacing::default(),
            pill_width: 7,
        }
    }
}

// Global, runtime-configurable style (lazy + RwLock to avoid extra deps).
static STYLE_CFG: Lazy<std::sync::RwLock<StyleConfig>> =
    Lazy::new(|| std::sync::RwLock::new(StyleConfig::default()));

pub struct StyleBuilder {
    cfg: StyleConfig,
}

impl StyleBuilder {
    pub fn new() -> Self {
        Self {
            cfg: StyleConfig::default(),
        }
    }
    pub fn palette(mut self, pal: Palette) -> Self {
        self.cfg.palette = pal;
        self
    }
    pub fn spacing(mut self, sp: Spacing) -> Self {
        self.cfg.spacing = sp;
        self
    }
    pub fn pill_width(mut self, w: usize) -> Self {
        self.cfg.pill_width = w;
        self
    }

    /// Apply immediately. Safe to call before or after `typographus_log::init`.
    pub fn apply(self) {
        *STYLE_CFG.write().expect("poisoned style lock") = self.cfg;
    }
}

impl Default for StyleBuilder {
    fn default() -> Self {
        Self::new()
    }
}

// ───────────────────── style helpers (CHANGED) ───────────────────────────────

// ───────────────────── level pill (CHANGED) ──────────────────────────────────
pub(crate) fn level_pill_for(level: Level) -> String {
    let cfg = STYLE_CFG.read().expect("style lock").clone();
    let pal = cfg.palette;

    let (fg, bg, label) = match level {
        Level::ERROR => (pal.error_fg, pal.error_bg, "ERROR"),
        Level::WARN => (pal.warn_fg, pal.warn_bg, "WARNING"),
        Level::INFO => (pal.info_fg, pal.info_bg, "INFO"),
        Level::DEBUG => (pal.debug_fg, pal.debug_bg, "DEBUG"),
        Level::TRACE => (pal.trace_fg, pal.trace_bg, "TRACE"),
    };

    // Fixed interior width to keep columns steady.
    let w = cfg.pill_width.max(label.len());
    let padded = format!(" {:^w$} ", label);

    let mut st = Style::new()
        .bg_color(Some(bg.into()))
        .fg_color(Some(fg.into()));
    if pal.bold_pills {
        st = st.effects(Effects::BOLD);
    }

    format!("{st}{padded}{Reset}")
}

/// Current style config (cloned; cheap, taken once per event).
pub(crate) fn style_snapshot() -> StyleConfig {
    STYLE_CFG.read().expect("style lock").clone()
}

// ───────────────────── public style API (NEW) ────────────────────────────────
/// Foreground-only level pill using configured palette and pill width.
/// Width equals `StyleConfig.pill_width` interior, padded with a single space
/// on both sides (same as the background pill), and uses the per-level fg color.
pub fn level_pill_fg(level: Level) -> String {
    let cfg = STYLE_CFG.read().expect("style lock").clone();
    let pal = cfg.palette;
    let (fg, label) = match level {
        Level::ERROR => (pal.error_fg, "ERROR"),
        Level::WARN => (pal.warn_fg, "WARNING"),
        Level::INFO => (pal.info_fg, "INFO"),
        Level::DEBUG => (pal.debug_fg, "DEBUG"),
        Level::TRACE => (pal.trace_fg, "TRACE"),
    };
    let w = cfg.pill_width.max(label.len());
    let text = format!(" {:^w$} ", label);
    let mut st = Style::new().fg_color(Some(fg.into()));
    if pal.bold_pills {
        st = st.effects(Effects::BOLD);
    }
    format!("{st}{text}{Reset}")
}
pub fn set_palette(palette: Palette) {
    let mut cfg = STYLE_CFG.write().expect("style lock");
    cfg.palette = palette;
}
pub fn set_spacing(spacing: Spacing) {
    let mut cfg = STYLE_CFG.write().expect("style lock");
    cfg.spacing = spacing;
}
pub fn set_pill_width(width: usize) {
    let mut cfg = STYLE_CFG.write().expect("style lock");
    cfg.pill_width = width.max(1);
}

// ------------------------------- tests ---------------------------------------
#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::{Mutex, MutexGuard};

    static TEST_MUTEX: Lazy<Mutex<()>> = Lazy::new(|| Mutex::new(()));

    fn test_lock() -> MutexGuard<'static, ()> {
        TEST_MUTEX.lock().expect("mutex poisoned")
    }

    fn strip_ansi(s: &str) -> String {
        let mut out: Vec<u8> = Vec::with_capacity(s.len());
        let mut bytes = s.as_bytes().iter().copied().peekable();
        while let Some(b) = bytes.next() {
            if b == 0x1b /* ESC */ && matches!(bytes.peek(), Some(b'[')) {
                let _ = bytes.next(); // skip '['
                for c in bytes.by_ref() {
                    if c == b'm' {
                        break;
                    }
                }
                continue;
            }
            out.push(b);
        }
        String::from_utf8(out).unwrap_or_default()
    }

    #[test]
    fn palette_default_values() {
        let _g = test_lock();
        let p = Palette::default();
        assert_eq!(p.time_fg as u8, AnsiColor::BrightBlack as u8);
        assert_eq!(p.body_fg as u8, AnsiColor::White as u8);
        assert_eq!(p.error_fg as u8, AnsiColor::White as u8);
        assert_eq!(p.error_bg as u8, AnsiColor::Red as u8);
        assert_eq!(p.warn_fg as u8, AnsiColor::Black as u8);
        assert_eq!(p.warn_bg as u8, AnsiColor::Yellow as u8);
        assert_eq!(p.info_fg as u8, AnsiColor::Black as u8);
        assert_eq!(p.info_bg as u8, AnsiColor::Green as u8);
        assert_eq!(p.debug_fg as u8, AnsiColor::White as u8);
        assert_eq!(p.debug_bg as u8, AnsiColor::Blue as u8);
        assert_eq!(p.trace_fg as u8, AnsiColor::White as u8);
        assert_eq!(p.trace_bg as u8, AnsiColor::Magenta as u8);
        assert!(p.bold_pills);
    }

    #[test]
    fn spacing_default_values() {
        let _g = test_lock();
        let s = Spacing::default();
        assert_eq!(s.between_time_pill, 2);
        assert_eq!(s.between_pill_body, 2);
        assert_eq!(s.continuation_indent, "  ");
    }

    #[test]
    fn styleconfig_default_values() {
        let _g = test_lock();
        let c = StyleConfig::default();
        assert_eq!(c.pill_width, 7);
        assert_eq!(c.palette.time_fg as u8, AnsiColor::BrightBlack as u8);
        assert_eq!(c.palette.body_fg as u8, AnsiColor::White as u8);
        assert_eq!(c.spacing.between_time_pill, 2);
    }

    #[test]
    fn builder_apply_updates_global() {
        let _g = test_lock();
        let pal = Palette {
            time_fg: AnsiColor::Cyan,
            body_fg: AnsiColor::BrightWhite,
            error_fg: AnsiColor::Black,
            error_bg: AnsiColor::BrightRed,
            warn_fg: AnsiColor::BrightBlack,
            warn_bg: AnsiColor::BrightYellow,
            info_fg: AnsiColor::BrightBlack,
            info_bg: AnsiColor::BrightGreen,
            debug_fg: AnsiColor::BrightWhite,
            debug_bg: AnsiColor::BrightBlue,
            trace_fg: AnsiColor::BrightWhite,
            trace_bg: AnsiColor::BrightMagenta,
            bold_pills: false,
        };
        let spacing = Spacing {
            between_time_pill: 3,
            between_pill_body: 1,
            continuation_indent: "..".into(),
        };
        StyleBuilder::new()
            .palette(pal)
            .spacing(spacing.clone())
            .pill_width(11)
            .apply();
        let cfg = STYLE_CFG.read().unwrap().clone();
        assert_eq!(cfg.pill_width, 11);
        assert_eq!(cfg.palette.time_fg as u8, AnsiColor::Cyan as u8);
        assert_eq!(cfg.spacing.between_time_pill, 3);
        assert_eq!(cfg.spacing.between_pill_body, 1);
        assert_eq!(cfg.spacing.continuation_indent, "..");
        assert!(!cfg.palette.bold_pills);
    }

    #[test]
    fn setters_update_global() {
        let _g = test_lock();
        let pal = Palette {
            time_fg: AnsiColor::Green,
            ..Palette::default()
        };
        set_palette(pal);
        let sp = Spacing {
            between_time_pill: 5,
            between_pill_body: 4,
            continuation_indent: "--".into(),
        };
        set_spacing(sp.clone());
        set_pill_width(0); // clamps to 1
        let cfg = STYLE_CFG.read().unwrap().clone();
        assert_eq!(cfg.palette.time_fg as u8, AnsiColor::Green as u8);
        assert_eq!(cfg.spacing.between_time_pill, 5);
        assert_eq!(cfg.spacing.between_pill_body, 4);
        assert_eq!(cfg.spacing.continuation_indent, "--");
        assert_eq!(cfg.pill_width, 1);
    }

    #[test]
    fn level_pill_fixed_width_and_style() {
        let _g = test_lock();
        *STYLE_CFG.write().unwrap() = StyleConfig::default();
        let warn = level_pill_for(Level::WARN);
        let stripped = strip_ansi(&warn);
        let w = STYLE_CFG.read().unwrap().pill_width.max("WARNING".len());
        assert_eq!(stripped, format!(" {:^w$} ", "WARNING"));

        let st = Style::new()
            .bg_color(Some(AnsiColor::Yellow.into()))
            .fg_color(Some(AnsiColor::Black.into()))
            .effects(Effects::BOLD)
            .render()
            .to_string();
        assert!(warn.starts_with(&st));
        assert!(warn.ends_with(format!("{Reset}").as_str()));
    }

    #[test]
    fn level_pill_fg_fixed_width_and_style() {
        let _g = test_lock();
        *STYLE_CFG.write().unwrap() = StyleConfig::default();
        let info = level_pill_fg(Level::INFO);
        let stripped = strip_ansi(&info);
        let w = STYLE_CFG.read().unwrap().pill_width.max("INFO".len());
        assert_eq!(stripped, format!(" {:^w$} ", "INFO"));
        let st = Style::new()
            .fg_color(Some(AnsiColor::Black.into()))
            .effects(Effects::BOLD)
            .render()
            .to_string();
        assert!(info.starts_with(&st));
        assert!(info.ends_with(format!("{Reset}").as_str()));
    }
}
