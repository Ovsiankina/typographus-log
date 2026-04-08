# typographus-log

Pretty, configurable console logging for Rust, inspired by Laravel/Symfony.

## Quick start

```rust
use log::LevelFilter;
use typographus_log::logger::Logger;

fn main() {
    Logger::init_logger(LevelFilter::Info);
    log::info!("Hello, styled world!");
}
```

## Customization

You can adjust colors, spacing, and the fixed width of the level “pill”.

```rust
use anstyle::AnsiColor;
use log::LevelFilter;
use typographus_log::logger::{Logger, LoggerStyleBuilder, Palette, Spacing};

fn main() {
    let pal = Palette { info_bg: AnsiColor::BrightCyan, info_fg: AnsiColor::Black, ..Palette::default() };
    let spacing = Spacing { between_time_pill: 3, between_pill_body: 1, continuation_indent: "  ".into() };

    LoggerStyleBuilder::new()
        .palette(pal)
        .spacing(spacing)
        .pill_width(9)
        .apply();

    Logger::init_logger(LevelFilter::Info);
    log::info!("custom palette and spacing");
}
```

More examples are available in `examples/`:

- `examples/basic.rs`
- `examples/custom_palette_builder.rs`
- `examples/custom_palette_setters.rs`
- `examples/custom_spacing.rs`
- `examples/no_bold_pills.rs`
- `examples/runtime_update.rs`
- `examples/wide_pill.rs`
- `examples/tables.rs`
- `examples/panicdemo.rs`
- `examples/panicdemo_bglabel.rs`

## Tables

Tables/boxes are rendered with Unicode borders and a configurable accent color and padding.

Key API:
- `table::TableChars` to customize border characters
- `table::TableStyle` and `TableStyleBuilder` to configure accent color and spacing
- `box_top_line`, `box_body_row`, `box_empty_row`, `box_bottom_line`

Example with dim borders and foreground pills (panic-style):

```rust
use anstyle::AnsiColor;
use log::Level;
use typographus_log::{clock::now_hms, logger::level_pill_fg, table::{self, TableStyleBuilder}};

fn main() {
    // Dim borders
    TableStyleBuilder::new().accent_fg(AnsiColor::BrightBlack).apply();
    let w = table::terminal_width().unwrap_or(80);
    let left = format!("{}  {}", now_hms(), level_pill_fg(Level::Info));
    let body = ["Welcome to the app", "Rendering home page"];
    let block = table::compose_box(&left, "route=/ home", &body, "ok", w);
    println!("{}", block);
}
```

Colored border showcase (separate example): see `examples/tables_colored_border.rs`.

Width policy: boxes span the terminal width (from `COLUMNS` env var) when available; otherwise, they size to the content. If the terminal is too small (< 40 columns), a short notice is printed instead of a full box.

## Panic Context

Capture runtime panics with a styled block including a code frame. The panic hook resolves file and line at runtime and highlights a configurable number of context lines.

Key API:
- `panic::install_panic_hook(style)`
- `panic::PanicStyle` fields: `width`, `context`, `focus_fg`, `focus_bold`, `context_fg`, `highlight`, `language`, `label_text`, `label_fg`, `label_bold`, `label_bg`, `pointer`
- `panic::render_panic_block_auto(...)` for manual rendering with auto width
- Set `use_logger_pill: true` to reuse the logger’s pill for the label (background style)
 - Use `logger::level_pill_fg(level)` for foreground pills in tables/boxes

Defaults:
- Label (near the timestamp) is foreground-only by default (no background). Users can opt-in to a background.
- Pointer to the focus line defaults to `->` and is customizable.

Examples:

Foreground label (default):
```rust
use typographus_log::panic as panicmod;

fn main() {
    panicmod::install_panic_hook(panicmod::PanicStyle { context: 4, ..Default::default() });
    panic!("demonstration panic with runtime source context");
}
```

Background label and custom pointer:
```rust
use anstyle::AnsiColor;
use typographus_log::panic as panicmod;

fn main() {
    let style = panicmod::PanicStyle { label_bg: Some(AnsiColor::Red), label_fg: AnsiColor::White, pointer: "=>", ..Default::default() };
    panicmod::install_panic_hook(style);
    panic!("panic with background label and custom pointer");
}
```

Syntax highlighting: a tiny built-in highlighter colors comments, strings, numbers, and Rust-like keywords. Disable via `highlight: false` or choose `language: syntax::Language::Plain`.

Auto width and truncation:
- With a detected terminal width (`COLUMNS` env variable), panic boxes span the full terminal width; long content and metadata are truncated with an ellipsis.
- Without a detected width, the box grows to the longest context line (with padding).
- If the terminal is very narrow (< 40 columns), a short warning is printed instead of a malformed box.
