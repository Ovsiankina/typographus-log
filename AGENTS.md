# Repository Guidelines

typographus-log is a thin, opinionated pretty layer for terminal output:
a `tracing` subscriber format, error rendering for any `std::error::Error`
(thiserror included), an optional eyre handler, and a panic hook. Two
priorities, in this order when they conflict: **dead-simple DX**, then
**low per-event cost**.

## Project Structure

- `src/lib.rs`: crate docs, re-exports, `init()` / `init_with_level()`.
- `src/typographus/`
  - `tracing_fmt.rs`: `TypographusFormat` (`FormatEvent`), field visitor, formatter tests.
  - `error.rs`: `render_error` / `render_chain`, `Report`, `Result`, field-mode flag.
  - `eyre_hook.rs`: `EyreHandler` impl + `install()` (feature `eyre`).
  - `style.rs`: `Palette`, `Spacing`, `StyleConfig`, `StyleBuilder`, level pills, global `RwLock` store.
  - `ansi.rs`: global color on/off (TTY + `NO_COLOR`), `strip`.
  - `clock.rs`: cached local offset, `write_hms` / `now_hms`.
  - `panic.rs`, `syntax.rs`, `table.rs`: panic box, tiny highlighter, box drawing.
- `examples/showcase.rs`: every feature at once (needs `--features eyre`). Other examples show one feature each.
- `README.md` is compiled as doctests (`ReadmeDoctests` in `lib.rs`): keep its code blocks building.
- `src/guideline.md`: design reference (Laravel/Symfony console style), not code.

## Commands

- `cargo fmt --all`
- `cargo clippy --all-targets -- -D warnings` **and** `cargo clippy --all-targets --all-features -- -D warnings`
- `cargo test --all-features` (unit tests + README/lib doctests)
- `cargo run --example showcase --features eyre` to eyeball output.

## Output Guarantees (don't break these)

- All output goes to stderr. Colors only when `ansi::enabled()`; otherwise strip.
- Fixed-width level pill; every colored segment self-resets (no ANSI bleed).
- Multiline message, field values and error chains are indented by `Spacing::continuation_indent`.
- Error fields (`&dyn Error`, `?Report`, fields named `error`/`err`, multiline values) render as a `✗` block, never inline.
- `init()` installs each part independently and at most once; a pre-existing subscriber/logger/hook must not stop the others.
- Hot path: one style read guard per event, one line buffer, no clones of `StyleConfig`. Don't run user `Debug`/`Display` while holding the style lock.

## Coding Style

- rustfmt defaults. snake_case / UpperCamelCase / SCREAMING_SNAKE_CASE.
- `//!` module docs and short `///` item docs; prefer examples that compile.
- Edition 2024, stable toolchain (`rust-version` in `Cargo.toml`).

## Testing

- Unit tests live next to the code in `#[cfg(test)]`.
- Compare styled output after `ansi::strip`.
- Tests that read or write the global style must hold `style::test_style_lock()`.
- For formatter behavior, use the `capture(|| ...)` helper in `tracing_fmt.rs` tests (a real subscriber writing to a buffer).

## Dependencies

Current: `tracing`, `tracing-subscriber` (no default features), `tracing-log`, `anstyle`, `time`, `once_cell`, optional `eyre`. Dev-only: `thiserror`, `log`.
Don't add dependencies without a clear reason stated in the PR.

## Commits & PRs

- Conventional Commits (`feat(tracing): ...`, `fix(clock): ...`, `refactor!: ...` with `BREAKING CHANGE:`).
- One design decision per branch so it can be dropped independently.
- PRs: what and why, before/after terminal output when formatting changes, README updated when the API changes.
