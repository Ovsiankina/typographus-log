# typographus-log

Pretty, opinionated terminal output for Rust apps: `tracing` events, errors
(thiserror, eyre) and panics, all in one consistent look. One call to set up.

```text
11:42:30    INFO     server listening  port=8080  tls=true
11:42:30   WARNING   app › handle_request{id=2}  slow upstream  latency_ms=812
11:42:30    ERROR    app  request aborted
  ✗ startup failed
    └─ caused by: could not read config file `/etc/app.toml`
      └─ caused by: No such file or directory (os error 2)
```

## Install

```toml
[dependencies]
typographus-log = { git = "https://github.com/Ovsiankina/typographus-log" }
tracing = "0.1"

# optional: style eyre::Report too
# typographus-log = { git = "...", features = ["eyre"] }
```

## Quick start

```rust,no_run
fn main() -> typographus_log::error::Result<()> {
    typographus_log::init();

    tracing::info!(port = 8080, "server listening");
    std::fs::read_to_string("config.toml")?; // any std error works with `?`
    Ok(())
}
```

`init()` installs, once:

- a `tracing` subscriber with the typographus format (INFO and up;
  `init_with_level(LevelFilter::TRACE)` for more),
- a bridge so crates still using `log` show up the same way,
- a panic hook (boxed message + source context),
- the eyre hook, if the `eyre` feature is on.

If your app already set its own subscriber, `log` logger or hook, that part
is left alone and the rest is still installed.

## Errors

typographus doesn't care how an error was made. Anything implementing
`std::error::Error` (thiserror enums, `io::Error`, ...) is shown with its
message and every `.source()` below it.

Pick the setup that matches your crate:

| You're writing | Do this | Result |
|---|---|---|
| a **library** | Define errors with thiserror; log with `tracing`. Don't depend on typographus. | The app using your lib renders them. |
| a **binary**, no eyre | `fn main() -> typographus_log::error::Result<()>` | `?` converts any error; `main` prints it pretty. |
| a **binary** with eyre | enable `features = ["eyre"]`, keep `fn main() -> eyre::Result<()>` | `eyre::Report` prints in the same style, `.wrap_err()` layers included. |

std always prints `Error: ` before the error returned from `main`; that
prefix can't be removed.

### Errors inside log events

```rust,no_run
use std::error::Error;

# fn demo(e: std::io::Error, report: typographus_log::error::Report) {
tracing::error!(error = &e as &dyn Error, "request aborted"); // message + full chain
tracing::warn!(error = ?report, "retrying");                  // Report / eyre::Report: full chain
tracing::warn!(err = %e, "falling back");                     // message only
# }
```

Fields named `error` or `err`, and any multiline value, are drawn as a `✗`
block under the line instead of inline `key=value`.

## Output and colors

- Everything goes to **stderr**. stdout stays free for your program's data
  (`app | jq` is safe).
- Colors are on when stderr is a terminal and `NO_COLOR` is unset.
  Override with `typographus_log::ansi::set_enabled(bool)`, e.g. from a
  `--color` flag.

## Customizing the look

```rust,no_run
use typographus_log::style::{AnsiColor, Palette, Spacing, StyleBuilder};

StyleBuilder::new()
    .palette(Palette { info_bg: AnsiColor::BrightCyan, ..Palette::default() })
    .spacing(Spacing { between_pill_body: 1, ..Spacing::default() })
    .pill_width(9)
    .apply();
typographus_log::init();
```

Safe to call before or after `init()`, and at runtime. There are also
one-off setters: `style::set_palette`, `set_spacing`, `set_pill_width`.

## Panics

`init()` installs the hook with defaults. To change the look, install it
yourself after `init()`:

```rust,no_run
use typographus_log::panic::{install_panic_hook, PanicStyle};

typographus_log::init();
install_panic_hook(PanicStyle { context: 4, pointer: "=>", ..Default::default() });
```

The box shows the message, the panicking thread, and the source lines
around the panic with light syntax highlighting (`highlight: false` to turn
it off). With `RUST_BACKTRACE=1` a backtrace is printed under it.

## Boxes

`table::compose_box` and the `box_*` helpers draw the same bordered boxes
the panic hook uses, for your own output. See `examples/tables.rs`.

## Performance

Filtered-out events cost what `tracing` costs: nothing is formatted. An
enabled event takes one read lock and builds one line buffer before a
single write to stderr. Errors are only rendered when they're printed.

## Examples

```sh
cargo run --example showcase --features eyre   # everything at once
cargo run --example basic
cargo run --example custom_palette_builder
cargo run --example panicdemo
cargo run --example tables
```

All examples live in `examples/`.
