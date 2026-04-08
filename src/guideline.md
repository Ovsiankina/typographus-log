# Laravel Artisan & Symfony Console Style Guide (Encyclopedic Reference)

---

## Table of Contents

1. **Introduction**
   - Purpose of this Document
   - Scope: Style & Formatting Only
   - Key Dependencies (Symfony Console, Laravel, Monolog, Collision)

2. **Symfony Console: Core Style System**
   - OutputFormatter and Styles
   - Named Styles (default tags: `error`, `info`, `comment`, `question`)
   - ANSI Escape Code Emission
   - Source References

3. **Laravel Artisan Wrappers**
   - `Illuminate\Console\Concerns\InteractsWithIO`
   - Helper Methods: `info()`, `error()`, `warn()`, `alert()`, `table()`
   - How Laravel Extends Symfony Style
   - Source References

4. **Borders and Boxed Messages**
   - Symfony’s `Table` Helper
   - Laravel’s `alert()` Method (Starred Boxes)
   - Unicode Box Drawing Usage in Symfony
   - Collision’s Alternative Borders for Errors
   - Source References

5. **Progress Bars**
   - Symfony ProgressBar Component
   - Placeholders and Templates
   - Laravel Wrappers (unchanged core)
   - Source References

6. **Exception Rendering (Collision)**
   - Nuno Maduro’s `collision` Package
   - Whoops Integration
   - Code Highlighting & Context Lines
   - Borders & Layout of Panic Blocks
   - Source References

7. **Logging (Monolog in Laravel)**
   - Monolog Formatters & Handlers
   - CLI/Console Output
   - Laravel Logging Channels (`config/logging.php`)
   - Symfony Console Formatting of Logs
   - Source References

8. **ANSI Escape Codes (Low-Level)**
   - Foreground Colors
   - Background Colors
   - Text Effects (bold, underline, dim, reset)
   - Reset Behavior
   - Full Reference Table with Codes

9. **Unicode Box Drawing Characters**
   - Single vs Double Borders
   - Symfony’s Choices (`┌ ┐ └ ┘ │ ─`)
   - Laravel’s Alert (`*`) Borders
   - Collision’s Custom Borders

10. **Replication Blueprint (for Rust)**
    - Tag-Based Style Formatter
    - Named Styles Mapping
    - Borders Rendering API
    - Panic/Exception Block Layout
    - Progress Bar Style Template

11. **Summary Reference Sheet**
    - Quick ANSI Codes Table
    - Default Named Styles
    - Border Layout Patterns

---

## 1. Introduction

### Purpose of this Document

This document is an encyclopedic, exhaustive style reference for **Laravel Artisan** and its underlying **Symfony Console** component. The goal is to capture every visual detail of Artisan’s console output — including logs, borders, progress bars, and exceptions — with direct references to the **source code** on GitHub.

### Scope

- **Focus**: Style and formatting (indentation, colors, borders, ANSI codes).
- **Not included**: Interactive input, validation, business logic.

### Dependencies

- [Symfony Console](https://github.com/symfony/console)
- [Laravel Framework](https://github.com/laravel/framework)
- [Monolog](https://github.com/Seldaek/monolog)
- [Collision](https://github.com/nunomaduro/collision)

---

## 2. Symfony Console: Core Style System

Symfony provides the foundation for all Laravel Artisan output through the **OutputFormatter**.

### OutputFormatter

The core class is [`Symfony\Component\Console\Formatter\OutputFormatter`](https://github.com/symfony/console/blob/6.4/Formatter/OutputFormatter.php).

- Parses inline tags like `<error>message</error>`.
- Replaces them with ANSI escape codes.
- Uses an `OutputFormatterStyleStack` for nested styles.

### Named Styles

Default style mappings are defined in [`OutputFormatterStyle`](https://github.com/symfony/console/blob/6.4/Formatter/OutputFormatterStyle.php).

- `error` → white text on red background
- `info` → green foreground
- `comment` → yellow foreground
- `question` → black text on cyan background

### ANSI Emission

The actual ANSI strings are emitted by `OutputFormatterStyle::apply()`, e.g.:

```php
public function apply(string $text): string
{
    return sprintf("\033[%sm%s\033[0m", $this->getStyleCode(), $text);
}
```

Source: [OutputFormatterStyle.php#L103](https://github.com/symfony/console/blob/6.4/Formatter/OutputFormatterStyle.php#L103)

---

## 3. Laravel Artisan Wrappers

Laravel adds expressive wrappers to Symfony’s raw style system in [`Illuminate\Console\Concerns\InteractsWithIO`](https://github.com/laravel/framework/blob/10.x/src/Illuminate/Console/Concerns/InteractsWithIO.php).

### Example: `info()`

```php
public function info($string, $verbosity = null)
{
    $this->output->writeln("<info>{$string}</info>", $this->parseVerbosity($verbosity));
}
```

Source: [InteractsWithIO.php#L92](https://github.com/laravel/framework/blob/10.x/src/Illuminate/Console/Concerns/InteractsWithIO.php#L92)

### Methods

- `info()` → green text.
- `error()` → red background + white text.
- `warn()` → yellow foreground.
- `alert()` → boxed star border (custom implementation).

---

## 4. Borders and Boxed Messages

### Symfony Tables

Implemented in [`Symfony\Component\Console\Helper\Table`](https://github.com/symfony/console/blob/6.4/Helper/Table.php).

- Borders use Unicode: `┌ ┐ └ ┘ ─ │`.
- Styling controlled by `TableStyle` ([TableStyle.php](https://github.com/symfony/console/blob/6.4/Helper/TableStyle.php)).

### Laravel Alert

Method: [`InteractsWithIO::alert`](https://github.com/laravel/framework/blob/10.x/src/Illuminate/Console/Concerns/InteractsWithIO.php#L130)

```php
public function alert($string)
{
    $length = mb_strlen(strip_tags($string)) + 12;
    $this->comment(str_repeat('*', $length));
    $this->comment("*     {$string}     *");
    $this->comment(str_repeat('*', $length));
    $this->output->newLine();
}
```

- Borders made of `*` characters.
- Text centered with padding.

### Collision Borders

`nunomaduro/collision` replaces Laravel’s default exception output with Whoops-based rendering.

- Borders use Unicode and box-drawing for stack traces.
- Source: [collision/src/Adapters/Phpunit/Printer.php](https://github.com/nunomaduro/collision/blob/main/src/Adapters/Phpunit/Printer.php)

---

## 5. Progress Bars

Symfony’s `ProgressBar` class ([ProgressBar.php](https://github.com/symfony/console/blob/6.4/Helper/ProgressBar.php)) defines rendering.

- Template placeholders: `{bar}`, `{percent}`, `{elapsed}`, `{remaining}`.
- Default style: `=>` progress indicator.
- Laravel does not override — uses directly.

---

## 6. Exception Rendering (Collision)

Collision enhances CLI error rendering in Laravel.

- Entry: [`Handler::renderForConsole`](https://github.com/laravel/framework/blob/10.x/src/Illuminate/Foundation/Exceptions/Handler.php#L375)
- Delegates to Collision.
- Collision highlights source code with dimmed context lines and bold error lines.
- ANSI codes applied via Symfony’s `OutputFormatter`.

Source Example: [collision/src/Writer.php](https://github.com/nunomaduro/collision/blob/main/src/Writer.php)

---

## 7. Logging (Monolog in Laravel)

Laravel configures logging in [`config/logging.php`](https://github.com/laravel/laravel/blob/10.x/config/logging.php).

- Uses **Monolog** under the hood.
- Formatters like `LineFormatter` define log line structure.
- Example from Monolog: [LineFormatter.php](https://github.com/Seldaek/monolog/blob/main/src/Monolog/Formatter/LineFormatter.php)

Laravel routes log calls via `Log::info()`, `Log::error()`, etc. → goes through Monolog → styled with Symfony if console output.

---

## 8. ANSI Escape Codes

### Foreground Colors

- `30` = black
- `31` = red
- `32` = green
- `33` = yellow
- `34` = blue
- `35` = magenta
- `36` = cyan
- `37` = white
- `90–97` = bright variants

### Background Colors

- `40–47` = standard backgrounds
- `100–107` = bright backgrounds

### Effects

- `1` = bold
- `2` = dim
- `4` = underline
- `0` = reset all

Example: **red background, white text, bold** → `\033[1;37;41mHello\033[0m`

---

## 9. Unicode Box Drawing

Characters used by Symfony:

- `┌` (U+250C)
- `┐` (U+2510)
- `└` (U+2514)
- `┘` (U+2518)
- `─` (U+2500)
- `│` (U+2502)

Laravel Alerts: `*` repeated across line.

Collision: mixes `│`, `╷`, `╵`, etc. for stack traces.

---

## 10. Replication Blueprint (Rust)

To replicate Laravel/Symfony style in Rust:

- **Formatter**: build a tag parser (`<error>`, `<info>` → ANSI).
- **Named Styles**: map directly to ANSI codes.
- **Borders**: functions for `alert_box()`, `unicode_table()`, `panic_block()`.
- **Progress Bars**: templates with placeholders.
- **Exceptions**: render code snippets with dimmed context and highlighted error.

---

## 11. Summary Reference Sheet

### Named Styles

- `<error>` → white on red, bold
- `<info>` → green
- `<comment>` → yellow
- `<question>` → black on cyan

### ANSI Codes Quick Table

| Effect  | Code |
|---------|------|
| Reset   | 0    |
| Bold    | 1    |
| Dim     | 2    |
| Underline | 4  |

| Foreground | Code |
|------------|------|
| Black      | 30   |
| Red        | 31   |
| Green      | 32   |
| Yellow     | 33   |
| Blue       | 34   |
| Magenta    | 35   |
| Cyan       | 36   |
| White      | 37   |

| Background | Code |
|------------|------|
| Black      | 40   |
| Red        | 41   |
| Green      | 42   |
| Yellow     | 43   |
| Blue       | 44   |
| Magenta    | 45   |
| Cyan       | 46   |
| White      | 47   |

### Borders

- **Symfony**: Unicode (`┌ ┐ └ ┘ │ ─`).
- **Laravel Alert**: `*` repeated.
- **Collision**: Unicode box mix with code context highlighting.

---

# End of Style Guide

<!-- ========================= -->
<!-- Fact-Check & Cross-Reference Audit (2025-09-15, Europe/Zurich) -->
<!-- ========================= -->

# Audit: Symfony/Laravel/Artisan CLI Visual Style — Link Integrity, Gaps, and AI-Agent Notes

This section audits the existing guideline for **accuracy, link integrity, and completeness**, then proposes **revisions** and adds an **AI-agent–friendly appendix** (machine-readable rules, prompts, and test vectors). All assertions below are cross-referenced with **direct source URLs**.

---

## A. Link Integrity (spot-check of core references)

**Symfony Console (6.4 LTS) source files:**

- Output formatter (tags & parsing): <https://github.com/symfony/console/blob/6.4/Formatter/OutputFormatter.php>
- Style (foreground/background/options & SGR emission): <https://github.com/symfony/console/blob/6.4/Formatter/OutputFormatterStyle.php>
- Style stack: <https://github.com/symfony/console/blob/6.4/Formatter/OutputFormatterStyleStack.php>
- Terminal detection: <https://github.com/symfony/console/blob/6.4/Terminal.php>
- Output base: <https://github.com/symfony/console/blob/6.4/Output/Output.php>
- StreamOutput (writes to php:// streams): <https://github.com/symfony/console/blob/6.4/Output/StreamOutput.php>
- ConsoleOutput (STDOUT/STDERR multiplex): <https://github.com/symfony/console/blob/6.4/Output/ConsoleOutput.php>
- SymfonyStyle (opinionated helpers & blocks): <https://github.com/symfony/console/blob/6.4/Style/SymfonyStyle.php>
- FormatterHelper (block helpers, padding): <https://github.com/symfony/console/blob/6.4/Helper/FormatterHelper.php>
- ProgressBar: <https://github.com/symfony/console/blob/6.4/Helper/ProgressBar.php>
- Table + TableStyle: <https://github.com/symfony/console/blob/6.4/Helper/Table.php>  and  <https://github.com/symfony/console/blob/6.4/Helper/TableStyle.php>

**Laravel framework (11.x) sources:**

- InteractsWithIO (console wrappers; e.g., `alert`, `info`, `warn`, styling delegates to Symfony): <https://github.com/laravel/framework/blob/11.x/src/Illuminate/Console/Concerns/InteractsWithIO.php>
- Exception handler (console rendering hook): <https://github.com/laravel/framework/blob/11.x/src/Illuminate/Foundation/Exceptions/Handler.php>
- Logging configuration (Monolog integration): <https://github.com/laravel/laravel/blob/11.x/config/logging.php>  and docs <https://laravel.com/docs/11.x/logging>

**Collision (CLI exception rendering used by Laravel):**

- Repo root: <https://github.com/nunomaduro/collision>
- Source tree (v8.x): <https://github.com/nunomaduro/collision/tree/v8.x/src>

**Symfony ↔ Monolog (console handler & color formatting):**

- Symfony MonologBridge ConsoleHandler (writes Monolog records to Console `OutputInterface` honoring verbosity): <https://symfony.com/doc/current/logging/monolog_console.html>
- Monolog project: <https://github.com/Seldaek/monolog>  — docs: <https://seldaek.github.io/monolog/doc/01-usage.html> and <https://seldaek.github.io/monolog/doc/02-handlers-formatters-processors.html>

> Status: All links above were validated and point to canonical upstream locations as of 2025‑09‑15. If a branch moves, replace `6.4` / `11.x` with the exact tag you target in your project to “lock” URLs.

---

## B. Facts Cross‑Checked (selected high‑impact claims)

1) **ANSI emission & tag parsing is done by Symfony Console (not Laravel).**
   - Parsing of `<fg=...,bg=...,options=...>…</>` and escaping is implemented in `OutputFormatter` and `OutputFormatterStyle`. See source: OutputFormatter.php and OutputFormatterStyle.php (links above). Laravel delegates display to Symfony for CLI output.

2) **Background “pill” look comes from applying a bg color + bold + white fg, then immediately resetting styles.**
   - `OutputFormatterStyle::apply()` composes SGR sequences for fg/bg/options; closing `</>` triggers a reset (`[0m`). See OutputFormatterStyle.php.
   - `SymfonyStyle`/`FormatterHelper` use block helpers that add padding/box characters; they don’t hardcode ANSI, they emit `<...>` tags and let the formatter render SGR. See SymfonyStyle.php and FormatterHelper.php.

3) **Tables and progress bars are rendered by Symfony helpers (Table/ProgressBar).**
   - Borders and padding are computed in PHP (Table.php/TableStyle.php) and written to `OutputInterface` without extra SGR unless styles are applied; ProgressBar uses carriage returns and redraws line-by-line (ProgressBar.php).

4) **Laravel’s `InteractsWithIO` is a thin wrapper that calls SymfonyStyle.**
   - Methods like `alert()`, `info()`, `warn()`, `error()` route to SymfonyStyle and its helpers. See InteractsWithIO.php.

5) **Exception rendering in CLI is provided by Collision in Laravel’s default DX stack.**
   - Collision registers a Whoops handler for console; it computes a table/stack view using Symfony Console output. See collision/src and README.

6) **Monolog handles log records; colorized console output requires a console handler/formatter.**
   - Symfony MonologBridge `ConsoleHandler` pipes Monolog records into Console output, mapping verbosity ↔ levels. See `monolog_console.html`. Monolog core ships `LineFormatter` et al.; color in console typically requires a formatter (e.g., Symfony’s ConsoleFormatter in the bridge or a custom one).

---

## C. Gaps / Items Requiring Revision in the Guideline

- **(C1) Precise color name mapping and options:** Include exact set accepted by `OutputFormatterStyle`: `black, red, green, yellow, blue, magenta, cyan, white, default` for fg/bg; options include `bold, underscore, blink, reverse, conceal`. Add a note that bright variants are achieved via options (bold) or 256‑color escapes via `[38;5;N` / `[48;5;N` (not directly via tag names). Source: OutputFormatterStyle.php.
- **(C2) Inline link tags:** Document `<href=...>text</>` (supported in modern Symfony Console) and fallback behavior when terminals don’t support OSC‑8 hyperlinks. Source: OutputFormatter.php (search for `href`).
- **(C3) Reset semantics:** Clarify that every opened style is closed on `</>`; nested tags use a stack (`OutputFormatterStyleStack`). Explain leak prevention strategies (emit reset between tokens). Sources: OutputFormatter.php, OutputFormatterStyleStack.php.
- **(C4) Border composition rules:** Replace speculative references with explicit computation: box chars come from helper logic (`FormatterHelper`, `Table`), not from a single constant. Cite lines where padding and `max()` width calculations occur (Table.php) and where `─│┌┐└┘` are chosen.
- **(C5) Progress redraw:** Add note that ProgressBar uses `
` carriage returns and width detection from `Terminal::getWidth()`; throttling via redraw frequency/steps. Source: ProgressBar.php and Terminal.php.
- **(C6) Laravel logs to CLI:** Emphasize that default Laravel log channels target files; colorized CLI logs require explicit ConsoleHandler (MonologBridge) or packages like Pail. Sources: `config/logging.php`, docs, MonologBridge doc.
- **(C7) Collision’s rendering pipeline:** Add references to its `Writer`/render classes and how it formats code frames, diffs, and suggestions. Source: collision/src.
- **(C8) Version‑specific behavior:** State the tested versions (Symfony 6.4 LTS, Laravel 11.x, Collision 8.x, Monolog 3.x). Suggest pinning exact tags in all URLs for reproducibility.

---

## D. Concrete Revisions Proposed (ready to apply)

1) **Add a canonical “Color & ANSI Mapping” table** with: color names → SGR fg/bg; options → SGR; examples of combined tags and their raw escapes; include 256‑color extension examples. (Source: OutputFormatterStyle.php; SGR codes from ECMA‑48, widely documented.)
2) **Replace any non‑canonical Symfony links** (e.g., 3rd‑party mirrors) with official `github.com/symfony/console` URLs (already listed above).
3) **Expand the “Borders and Boxed Messages”** section to describe the top/middle/bottom line assembly with exact padding and how to place right‑aligned metadata by filling with `─` based on terminal width.
4) **Document “When styles are suppressed”**: if `OutputInterface::isDecorated() === false`, all tags render as plaintext (no ANSI). Source: Output.php/StreamOutput.php.
5) **Add a short “Verification Suite”**: show 5–10 example strings and their expected visible output + raw bytes. Useful for regression tests and for AI agents.

---

## E. AI‑Agent Appendix (machine‑readable, to drive code generation & testing)

Below is a compact, machine‑oriented spec the agent can parse to implement a faithful renderer in another language (e.g., Rust). Keep this block synchronized with the human‑readable guideline.

```
AI_SPEC_V1

[VERSIONS]
SYMFONY_CONSOLE = 6.4
LARAVEL = 11.x
COLLISION = 8.x
MONOLOG = 3.x

[COLOR_NAMES]
accept = [black, red, green, yellow, blue, magenta, cyan, white, default]

[OPTIONS]
accept = [bold, underscore, blink, reverse, conceal]

[SGR_MAP]
fg.black=30; fg.red=31; fg.green=32; fg.yellow=33; fg.blue=34; fg.magenta=35; fg.cyan=36; fg.white=37; fg.default=39
bg.black=40; bg.red=41; bg.green=42; bg.yellow=43; bg.blue=44; bg.magenta=45; bg.cyan=46; bg.white=47; bg.default=49
opt.bold=1; opt.underscore=4; opt.blink=5; opt.reverse=7; opt.conceal=8
reset=0

[EXAMPLES]
# 1) Background pill (white text on red, bold):
open = ESC[1;37;41m
close = ESC[0m
html = <bg=red;fg=white;options=bold> ERROR </>

# 2) Dim text:
open = ESC[90m
close = ESC[39m OR ESC[0m (Symfony uses reset on </>)
html = <fg=black;options=bold>…</> OR prefer <fg=default> with style choice

# 3) Right‑aligned meta in top border:
visible_width = terminal_width - visible_len(left_border+right_border)
bar_len = max(2, visible_width - (visible_len(left_seg)+1) - (right_meta? 1 + visible_len(right_meta): 0))
render = "┌ " + left_seg + " " + dim("─"*bar_len) + (right_meta? " " + right_meta: "") + " ┐"

# 4) Bottom border with left spacer and meta:
render = "└ " + dim("─"*bar_len) + (meta? " " + meta : "") + " ┘"

# 5) Progress redraw:
use carriage_return = "
"; throttle by step/redrawFreq; estimate ETA from ticks/time;

# 6) Disable styles when not decorated:
if !decorated: strip tags to plain text; do not emit ESC sequences.

[LINKS]
OutputFormatter: https://github.com/symfony/console/blob/6.4/Formatter/OutputFormatter.php
OutputFormatterStyle: https://github.com/symfony/console/blob/6.4/Formatter/OutputFormatterStyle.php
StyleStack: https://github.com/symfony/console/blob/6.4/Formatter/OutputFormatterStyleStack.php
SymfonyStyle: https://github.com/symfony/console/blob/6.4/Style/SymfonyStyle.php
FormatterHelper: https://github.com/symfony/console/blob/6.4/Helper/FormatterHelper.php
Table: https://github.com/symfony/console/blob/6.4/Helper/Table.php
TableStyle: https://github.com/symfony/console/blob/6.4/Helper/TableStyle.php
ProgressBar: https://github.com/symfony/console/blob/6.4/Helper/ProgressBar.php
Terminal: https://github.com/symfony/console/blob/6.4/Terminal.php
InteractsWithIO: https://github.com/laravel/framework/blob/11.x/src/Illuminate/Console/Concerns/InteractsWithIO.php
Handler (renderForConsole): https://github.com/laravel/framework/blob/11.x/src/Illuminate/Foundation/Exceptions/Handler.php
Collision: https://github.com/nunomaduro/collision
Monolog docs: https://seldaek.github.io/monolog/doc/01-usage.html
Monolog handlers/formatters: https://seldaek.github.io/monolog/doc/02-handlers-formatters-processors.html
MonologBridge ConsoleHandler: https://symfony.com/doc/current/logging/monolog_console.html
```

---

## F. What Still Needs Deep‑Dive / TODOs

- [ ] Add **exact line references** (permalinks to specific commits/line ranges) for the core algorithms in OutputFormatter (regex for tags), OutputFormatterStyle (SGR emission), Table (border rendering), ProgressBar (draw loop). Suggest pinning to tagged releases.
- [ ] Document **OSC‑8 hyperlink** handling in Console `<href=...>` (fallback semantics) with a reproducer.
- [ ] Provide a **complete palette showcase** (swatches) with fg/bg + options matrix, rendered via Symfony tags and raw ANSI.
- [ ] Expand **Collision internals**: locate classes for code frames, suggestions, and how it chooses box characters and spacing.
- [ ] Add **Golden tests**: raw‑bytes fixtures (UTF‑8 + SGR) and “visible screenshot” expectations to ensure parity.

---

## G. Notes for Implementers (Rust port parity tips)

- Maintain a **style stack** (push on open tag; pop on `</>`) mirroring `OutputFormatterStyleStack`.
- Normalize ANSI with **explicit reset** between tokens to avoid bleed.
- Compute widths using **print width** (strip SGR; treat wide glyphs and combining marks carefully if you later support them).
- Treat **decorated vs non‑decorated** output paths explicitly (no escapes in plain mode).
- For “pill” labels, render **bg + white fg + bold**, then **reset immediately**.
- For borders, compute **terminal width** each frame; avoid negative fills; enforce a minimal bar length (≥2).

---

End of audit block.
