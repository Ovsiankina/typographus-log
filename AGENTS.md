# Repository Guidelines

## Project Structure & Module Organization
- `src/lib.rs`: library entry; re-exports `typographus::{clock, logger}`.
- `src/main.rs`: simple demo binary showing logging output.
- `src/typographus/`: modules (`clock.rs`, `logger.rs`, `mod.rs`).
- `src/test.rs`: development/demo utilities; not part of the public API.
- Cargo metadata: `Cargo.toml`, lockfile; no custom build scripts.

## Build, Test, and Development Commands
- `cargo build` — compile the crate.
- `cargo run` — run the demo binary.
- `cargo test` — run doctests and any unit tests.
- `cargo fmt --all` — format with rustfmt.
- `cargo clippy --all-targets -- -D warnings` — lint; treat warnings as errors.

## Coding Style & Naming Conventions
- Use rustfmt defaults (4-space indents, imports grouped). Commit formatted code.
- Naming: snake_case for modules/functions, UpperCamelCase for types/traits, SCREAMING_SNAKE_CASE for consts.
- Logger output: keep fixed-width level “pill”, dim timestamps, indent multiline bodies by two spaces, and ensure every colored segment self-resets (see `logger.rs`).
- Prefer module docs (`//!`) and brief `///` item docs with runnable examples when useful.

## Testing Guidelines
- Favor small unit tests colocated with code (`#[cfg(test)]`) and doctests in examples.
- For assertions on styled output, strip ANSI escapes before comparing. Example regex: `\x1b\[[0-9;]*m`.
- Run full suite locally: `cargo test`. Add focused tests for spacing/pill width where behavior is sensitive.

## Commit & Pull Request Guidelines
- History shows no established convention; use Conventional Commits.
  - Examples: `feat(logger): add fixed pill width`, `fix(clock): local offset fallback`, `chore: run rustfmt`.
- PRs should include: clear description of the change and rationale, linked issues, and before/after terminal screenshots when output formatting changes.
- Keep changes minimal and focused; update README or docs when behavior or API changes.

## Agent-Specific Instructions
- Do not introduce new dependencies without discussion. Prefer small, surgical changes.
- Preserve logger guarantees: fixed-width pill, no ANSI bleed, stable spacing; update doctests if formatting changes.
- Keep code compiling on stable toolchains; avoid nightly-only features unless justified in the PR.

## Implementation Tasks (Develop Branch)
- [ ] Add configurable style types: `Palette`, `Spacing`, `StyleConfig` with sane defaults.
- [ ] Add global, thread-safe style store (`RwLock<StyleConfig>`).
- [ ] Implement `LoggerStyleBuilder` with `.palette()`, `.spacing()`, `.pill_width()`, `.apply()`.
- [ ] Make helpers palette-aware: `dim_on()`, `dim(&str)`, `white(&str)`.
- [ ] Rework `level_pill` to use per-level fg/bg, optional bold, fixed interior width.
- [ ] Apply configurable spacing and continuation indent in `impl Log for Logger`.
- [ ] Expose public setters: `set_palette`, `set_spacing`, `set_pill_width`.
- [ ] Keep defaults matching current output (back-compat look-and-feel).
- [ ] Build and run locally: `cargo build`, `cargo run`.
- [ ] Update README with a short usage snippet (optional, if behavior changes).

### New Tasks (Tables & Panic Context)
- [ ] Table module with configurable chars and accent color (`TableChars`, `TableStyle`, builder, `set_table_style`).
- [ ] Public renderers: `box_top_line`, `box_body_row`, `box_empty_row`, `box_bottom_line` with strict ANSI self-resets.
- [ ] Panic module with runtime source-context resolution via `info.location()`; no hardcoded paths.
- [ ] Public `PanicStyle` (context size, width, colors/bold for focus and context lines) and `install_panic_hook`.
- [ ] Optional syntax highlighting without new dependencies: simple token highlighter (comments, strings, keywords, numbers) with configurable colors.
- [ ] Unit tests first: table borders/width; panic block shape; code context rendering and highlighting (ensure ANSI strip equals original text).
- [ ] Examples: `examples/tables.rs` and `examples/panicdemo.rs` demonstrating customization and runtime context capture.

## What’s Implemented (Recap)

- Logger
  - Global style config with `Palette`, `Spacing`, `StyleConfig` and builder.
  - Background pill: `level_pill_for(level)` fixed interior width, per-level fg/bg.
  - New foreground pill: `level_pill_fg(level)` uses per-level fg, honors pill width and bold.
  - Public setters: `set_palette`, `set_spacing`, `set_pill_width`.

- Table
  - `TableChars`, `TableStyle`, `TableStyleBuilder`, and `set_table_style`.
  - Accent color for borders; optional bottom-left text (`bottom_left_text: Option<&'static str>`).
  - Renderers: `box_top_line`, `box_body_row` (with truncation + ellipsis), `box_empty_row`, `box_bottom_line` (left text + right meta cropping).
  - `terminal_width()` (reads `COLUMNS`) and `compose_box(...)` to build full box text.
  - Borders dim by default in examples; a separate example shows colored borders.

- Panic
  - `PanicStyle`: width, context, focus/context colors, pointer symbol, label options (`label_text/fg/bold/bg`), `use_logger_pill` for pill label.
  - `install_panic_hook(style)`: resolves location at runtime; box spans full terminal width (if known) or fits longest content; tiny terminals print a short notice.
  - `render_panic_block_auto(...)` and `format_code_context*` utils.
  - Highlighter: `syntax::{Language::{Plain,Rust,Auto}, SyntaxStyle, highlight_line, guess_language_from_path}`.

- Examples
  - `examples/tables.rs`: boxes like the panic demo with dim borders and foreground pills for INFO/WARN/ERROR/DEBUG/TRACE; full-width; uses `level_pill_fg` and `compose_box`.
  - `examples/tables_colored_border.rs`: same layout but shows cyan border accent.
  - `examples/panicdemo.rs`, `examples/panicdemo_bglabel.rs`, `examples/panicdemo_pill.rs`.

- Tests
  - 19 unit tests: table width/borders/compose, panic autowidth/context rendering, logger pills, syntax ANSI preservation.

## Documentation Notes

- README documents tables and panic usage, including:
  - Foreground vs background label (pill) for panic.
  - Auto-width spanning terminal and truncation with ellipsis.
  - Border customization and bottom-left text for boxes.
  - Syntax highlighting and language auto-detection.

## Follow-ups

- Optional: extend language guessing (`.toml`, `.json`) and colorization.
- Optional: helper to compute visible width across segments when composing mixed tokens.

### Summary of Requested Features
- Users can override colors per level and text accents via a simple API.
- Users can control spacing between time, level pill, and body, plus continuation indent.
- Defaults remain the same as today; the library stays “standard” yet easily customizable.
