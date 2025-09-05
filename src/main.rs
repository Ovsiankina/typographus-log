use log::LevelFilter;
use typographus_log::logger::Logger;

fn main() {
    Logger::init_logger(LevelFilter::Trace);
    // section("Logging demo (all levels)");
    log::error!("This is an error message");
    log::warn!("This is a warning message");
    log::info!("This is an info message");
    log::debug!("This is a debug message\nspanning\nmultiple lines");
    log::trace!("This is a trace message");
}
// use std::{
//     fs::{self, File},
//     io::{self, BufRead, BufReader, Read, Seek, SeekFrom},
//     path::PathBuf,
//     thread,
//     time::{Duration, Instant},
// };
//
// use anstream::{eprintln as a_eprintln, println as a_println};
// use anstyle::{AnsiColor, Effects, Reset, Style};
// use clap::{Parser, Subcommand};
// use dialoguer::{Confirm, Input};
// use indicatif::{ProgressBar, ProgressStyle};
// use log::{Level, LevelFilter, Log, Metadata, Record};
// use notify::{Config, Event, RecommendedWatcher, RecursiveMode, Watcher, recommended_watcher};
// use once_cell::sync::Lazy;
// use rand::Rng;
// use regex::Regex;
// use terminal_size::{Width, terminal_size};
// use time::OffsetDateTime;
// use time::format_description::{self, FormatItem};
//
// /// --------------------------- style helpers ---------------------------------
//
// // Background “pill” for level. Ends with {Reset} so bg never leaks.
// fn level_pill(level: Level) -> String {
//     let (bg, label) = match level {
//         Level::Error => (AnsiColor::Red, "ERROR"),
//         Level::Warn => (AnsiColor::Yellow, "WARNING"),
//         Level::Info => (AnsiColor::Green, "INFO"),
//         Level::Debug => (AnsiColor::Blue, "DEBUG"),
//         Level::Trace => (AnsiColor::Magenta, "TRACE"),
//     };
//
//     let start = Style::new()
//         .bg_color(Some(bg.into()))
//         .fg_color(Some(AnsiColor::BrightBlack.into()))
//         .effects(Effects::BOLD);
//
//     // Padding lives inside the pill; immediate Reset at end.
//     format!("{start} {label} {Reset}")
// }
//
// #[inline]
// fn dim_on() -> String {
//     Style::new()
//         .fg_color(Some(AnsiColor::BrightBlack.into()))
//         .render()
//         .to_string() // IMPORTANT: no reset here; caller must add {Reset}
// }
//
// /// visible length without ANSI
// fn visible_len(s: &str) -> usize {
//     static RE: once_cell::sync::Lazy<regex::Regex> =
//         once_cell::sync::Lazy::new(|| regex::Regex::new(r"\x1b\[[0-9;]*m").unwrap());
//     RE.replace_all(s, "").len()
// }
//
// /// Fully dim the gutter `│ `, then reset.
// fn gutter_dim() -> String {
//     format!("{}│ {Reset}", dim_on())
// }
//
// fn dim(s: &str) -> String {
//     let st = Style::new().fg_color(Some(AnsiColor::BrightBlack.into()));
//     format!("{st}{s}{Reset}")
// }
//
// fn white(s: &str) -> String {
//     let st = Style::new().fg_color(Some(AnsiColor::White.into()));
//     format!("{st}{s}{Reset}")
// }
//
// fn section(title: &str) {
//     let s = Style::new()
//         .fg_color(Some(AnsiColor::BrightBlack.into()))
//         .effects(Effects::BOLD);
//     a_println!("{}── {} {}", s.render(), title, Reset);
// }
//
// /// --------------------------- time helpers ----------------------------------
// static TIME_FMT: Lazy<Vec<FormatItem<'static>>> =
//     Lazy::new(|| format_description::parse("[hour]:[minute]:[second]").expect("time fmt"));
//
// fn now_hms() -> String {
//     OffsetDateTime::now_local()
//         .unwrap_or_else(|_| OffsetDateTime::now_utc())
//         .format(&TIME_FMT)
//         .unwrap_or_else(|_| "??:??:??".into())
// }
//
// /// --------------------------- pretty logger ---------------------------------
// struct PrettyLogger;
//
// impl Log for PrettyLogger {
//     fn enabled(&self, metadata: &Metadata) -> bool {
//         metadata.level() <= log::max_level()
//     }
//     fn log(&self, record: &Record) {
//         if !self.enabled(record.metadata()) {
//             return;
//         }
//
//         // Build segments (each self-resets).
//         let ts = dim(&now_hms());
//         let pill = level_pill(record.level());
//
//         // indent multiline body, then color white (self-reset).
//         let indent = "  ";
//         let mut body = format!("{}", record.args());
//         body = body.replace('\n', &format!("\n{indent}"));
//         let body = white(&body);
//
//         // Hard resets between tokens so spaces are plain; start & end with Reset.
//         a_println!();
//         a_println!("{Reset}{ts}{Reset}  {pill}{Reset}  {body}{Reset}");
//     }
//     fn flush(&self) {}
// }
//
// static LOGGER: PrettyLogger = PrettyLogger;
//
// fn init_logger(level: LevelFilter) {
//     let _ = log::set_logger(&LOGGER);
//     log::set_max_level(level);
// }
//
// /// --------------------------- demo: logging ---------------------------------
// fn demo_logging() {
//     section("Logging demo (all levels)");
//     log::error!("This is an error message");
//     log::warn!("This is a warning message");
//     log::info!("This is an info message");
//     log::debug!("This is a debug message\nspanning\nmultiple lines");
//     log::trace!("This is a trace message");
// }
//
// /// --------------------------- Pail-like blocks ------------------------------
// fn term_cols() -> usize {
//     match terminal_size() {
//         Some((Width(w), _)) => w as usize,
//         None => 100,
//     }
// }
//
// /// Top border line:
// /// ┌ <time>  <PILL>  ─────── <right_meta> ┐
// /// (dim the whole bar only, not the meta)
// fn box_top_line(left_seg: &str, right_meta: &str) -> String {
//     let width = term_cols();
//     let left_border = "┌ ";
//     let right_border = " ┐";
//
//     let inner_width = width.saturating_sub(visible_len(left_border) + visible_len(right_border));
//
//     // layout: LEFT + ' ' + DIM(BAR) + [ ' ' + RIGHT_META ]
//     let left_len = visible_len(left_seg) + 1; // +1 space after left
//     let right_len = if right_meta.is_empty() {
//         0
//     } else {
//         1 + visible_len(right_meta)
//     };
//
//     let bar_len = inner_width.saturating_sub(left_len + right_len);
//
//     let bar = format!("{}{}{}", dim_on(), "─".repeat(bar_len.max(0)), Reset);
//
//     format!(
//         "{dim}{left_border}{left} {bar}{right_pad}{dim}{right_border}",
//         dim = dim_on(),
//         left = left_seg,
//         right_pad = if right_len == 0 {
//             "".to_string()
//         } else {
//             format!(" {}", right_meta)
//         },
//     )
// }
//
// /// Empty inner row with both borders:
// /// │ <spaces> │
// fn box_empty_row() -> String {
//     let width = term_cols();
//     let left_border = "│ ";
//     let right_border = " │";
//     let inner_width = width.saturating_sub(visible_len(left_border) + visible_len(right_border));
//     format!(
//         "{dim}{left}{spaces}{right}",
//         dim = dim_on(),
//         left = left_border,
//         spaces = " ".repeat(inner_width),
//         right = right_border
//     )
// }
//
// /// Body row (content may include ANSI). Pads to right border.
// /// │ <content...>········································ │
// fn box_body_row(content: &str) -> String {
//     let width = term_cols();
//     let left_border = "│ ";
//     let right_border = " │";
//     let inner_width = width.saturating_sub(visible_len(left_border) + visible_len(right_border));
//     let content_len = visible_len(content);
//     let pad = inner_width.saturating_sub(content_len);
//     format!(
//         "{dim}{left}{content}{spaces}{dim}{right}",
//         dim = dim_on(),
//         left = left_border,
//         content = content,
//         spaces = " ".repeat(pad),
//         right = right_border
//     )
// }
//
// /// Bottom border line:
// /// └   ───────────── <meta> ┘
// /// (two-space left spacer before the dimmed bar)
// fn box_bottom_line(right_meta: &str) -> String {
//     let width = term_cols();
//     let left_border = "└";
//     let right_border = " ┘";
//
//     let inner_width = width.saturating_sub(visible_len(left_border) + visible_len(right_border));
//
//     let left_len = 0; // +1 space after left seg
//     let right_len = if right_meta.is_empty() {
//         0
//     } else {
//         1 + visible_len(right_meta)
//     };
//
//     let bar_len = inner_width.saturating_sub(left_len + right_len);
//
//     let bar = format!("{}{}{}", dim_on(), "─".repeat(bar_len.max(0)), Reset);
//
//     format!(
//         "{left_border}{bar}{right_pad}{dim}{right_border}",
//         dim = dim_on(),
//         right_pad = if right_len == 0 {
//             "".to_string()
//         } else {
//             format!(" {}", right_meta)
//         },
//     )
// }
//
// fn strip_ansi_len(s: &str) -> usize {
//     let re = regex::Regex::new(r"\x1b\[[0-9;]*m").unwrap();
//     re.replace_all(s, "").len()
// }
//
// fn draw_rule(left: &str, right: &str) -> String {
//     let cols = term_cols();
//     let left_len = strip_ansi_len(left);
//     let right_len = strip_ansi_len(right);
//     let mut fill = cols.saturating_sub(left_len + right_len + 2); // spaces around fill
//     if fill < 2 {
//         fill = 2;
//     }
//
//     // dim ONLY the "─…", leave `left` and `right` in their own styling
//     format!(
//         "{dim}{left} {bar} {right}",
//         dim = dim_on(),
//         bar = "─".repeat(fill),
//     )
// }
//
// fn render_log_block(level: Level, message: &str, right_meta: &str, bottom_meta: &str) {
//     let time = dim(&now_hms()); // gray time, self-reset
//     let pill = level_pill(level); // bg pill, self-reset
//     let left = format!("{time}  {pill}");
//
//     // ┌ time  PILL  ─── right_meta ┐
//     let top = box_top_line(&left, right_meta);
//
//     // body (white), lines padded to right border
//     let body = white(&message.replace('\n', "\n  "));
//     let body_lines: Vec<&str> = body.split('\n').collect();
//
//     a_println!();
//     a_println!("{top}");
//     for line in body_lines {
//         a_println!("{}", box_body_row(line));
//     }
//     // explicit blank inner row
//     a_println!("{}", box_empty_row());
//     // └   ─── bottom_meta ┘
//     a_println!("{}", box_bottom_line(bottom_meta));
// }
//
// fn demo_table() {
//     section("Pail-like log blocks");
//
//     // INFO
//     render_log_block(
//         Level::Info,
//         "This is some useful information.",
//         "GET: /   •   Auth ID: guest",
//         "request_id=ea7c9d trace_id=2f9ac1",
//     );
//
//     // ERROR
//     render_log_block(
//         Level::Error,
//         "Error Processing Request",
//         "routes/web.php:19",
//         "exception=RuntimeException code=500",
//     );
//
//     // WARNING
//     render_log_block(
//         Level::Warn,
//         "Something could be going wrong.",
//         "GET: /settings   •   Auth ID: guest",
//         "throttle=near-limit",
//     );
// }
//
// /// --------------------------- demo: progress --------------------------------
// fn simulate_expensive_step() {
//     let mut rng = rand::rng();
//     let millis = rng.random_range(0..=300);
//     thread::sleep(Duration::from_millis(millis));
// }
//
// fn demo_progress() {
//     // original bar
//     section("Progress demo");
//     let pb = ProgressBar::new(120);
//     pb.set_style(
//         ProgressStyle::default_bar()
//             .template("{prefix:.bold} {wide_bar} {pos}/{len} ({eta})")
//             .expect("template")
//             .progress_chars("█░ "),
//     );
//     pb.set_prefix(level_pill(Level::Info));
//     for i in 0..=120 {
//         pb.set_position(i);
//         thread::sleep(Duration::from_millis(10));
//     }
//     pb.finish_with_message(format!("{} Completed.", level_pill(Level::Info)));
//
//     // timed bar
//     section("Timed operation demo");
//     let total_steps = 100u64;
//     let pb2 = ProgressBar::new(total_steps);
//     pb2.set_style(
//         ProgressStyle::default_bar()
//             .template("{prefix:.bold} {wide_bar} {pos}/{len} • elapsed: {msg}")
//             .expect("template")
//             .progress_chars("█░ "),
//     );
//     pb2.set_prefix(level_pill(Level::Info));
//
//     let start = Instant::now();
//     for i in 0..=total_steps {
//         simulate_expensive_step();
//         pb2.set_position(i);
//         pb2.set_message(format!("{:.2?}", start.elapsed()));
//     }
//     let elapsed = start.elapsed();
//     pb2.finish_with_message(format!(
//         "{} Completed in {:.2?}.",
//         level_pill(Level::Info),
//         elapsed
//     ));
// }
//
// /// --------------------------- demo: prompts ---------------------------------
// fn demo_prompts() -> anyhow::Result<()> {
//     section("Prompts demo");
//     let project: String = Input::new().with_prompt("Project name").interact_text()?;
//     let proceed = Confirm::new()
//         .with_prompt("Generate files?")
//         .default(true)
//         .interact()?;
//
//     log::info!("Scaffold request\n  project: {project}\n  proceed: {proceed}");
//     Ok(())
// }
//
// /// --------------------------- CLI setup -------------------------------------
// #[derive(Parser)]
// #[command(name = "artisanish", about = "Tiny artisan-style CLI demo")]
// struct Cli {
//     #[arg(short = 'q', long, action = clap::ArgAction::Count)]
//     quiet: u8,
//
//     #[command(subcommand)]
//     cmd: Commands,
// }
//
// #[derive(Subcommand)]
// enum Commands {
//     Tables,
//     Progress,
//     Prompts,
//     Logging,
//     /// Tail a file; optionally filter with a regex
//     Tail {
//         #[arg(value_name = "FILE")]
//         file: PathBuf,
//         #[arg(long, value_name = "REGEX")]
//         grep: Option<String>,
//     },
//     /// Trigger a sample runtime error to demo the context printer
//     PanicDemo,
//     /// Trigger a manual bail! that still shows context but returns an error
//     BailDemo,
// }
//
// /// --------------------------- Tail ------------------------------------------
// fn tail_file(path: PathBuf, grep: Option<Regex>) -> anyhow::Result<()> {
//     section(&format!("Tailing {}  (Ctrl+C to exit)", path.display()));
//
//     // Start by showing last ~50 lines
//     show_last_lines(&path, 50, grep.as_ref())?;
//
//     let mut offset = fs::metadata(&path).map(|m| m.len()).unwrap_or(0);
//
//     loop {
//         if let Err(e) = read_new_lines(&path, &mut offset, grep.as_ref()) {
//             log::warn!("tail poll error: {e}");
//         }
//         thread::sleep(Duration::from_millis(300));
//     }
// }
//
// fn show_last_lines(path: &PathBuf, n: usize, grep: Option<&Regex>) -> anyhow::Result<()> {
//     let mut f = File::open(path)?;
//     let mut buf = String::new();
//     f.read_to_string(&mut buf)?;
//     let lines: Vec<_> = buf.lines().collect();
//     let start = lines.len().saturating_sub(n);
//     for line in &lines[start..] {
//         if let Some(re) = grep {
//             if !re.is_match(line) {
//                 continue;
//             }
//         }
//         render_log_line(line);
//     }
//     Ok(())
// }
//
// fn read_new_lines(path: &PathBuf, offset: &mut u64, grep: Option<&Regex>) -> anyhow::Result<()> {
//     let mut f = File::open(path)?;
//     let sz = f.metadata()?.len();
//     if sz < *offset {
//         *offset = 0;
//     }
//     f.seek(SeekFrom::Start(*offset))?;
//
//     let mut r = BufReader::new(f);
//     let mut buf = String::new();
//     let mut read = 0_u64;
//
//     while let Ok(n) = r.read_line(&mut buf) {
//         if n == 0 {
//             break;
//         }
//         read += n as u64;
//         let line = buf.trim_end_matches(['\n', '\r']);
//         if let Some(re) = grep {
//             if !re.is_match(line) {
//                 buf.clear();
//                 continue;
//             }
//         }
//         render_log_line(line);
//         buf.clear();
//     }
//     *offset += read;
//     Ok(())
// }
//
// fn render_log_line(line: &str) {
//     let level = if line.contains("ERROR") || line.contains("Exception") {
//         Level::Error
//     } else if line.contains("WARN") || line.contains("WARNING") {
//         Level::Warn
//     } else {
//         Level::Info
//     };
//     log::log!(level, "{line}");
// }
//
// fn format_code_context(file: &str, line: u32, ctx: usize) -> io::Result<String> {
//     let f = File::open(file)?;
//     let reader = BufReader::new(f);
//
//     let start = line.saturating_sub(ctx as u32).max(1);
//     let end = line + ctx as u32;
//
//     let mut out = String::new();
//     let mut idx: u32 = 1;
//     for l in reader.lines() {
//         let l = l?;
//         if idx >= start && idx <= end {
//             let number = format!("{idx:>6}");
//             if idx == line {
//                 // -> bold white focus line
//                 let styled = format!(
//                     "{}-> {}  {}{}",
//                     Style::new()
//                         .fg_color(Some(AnsiColor::White.into()))
//                         .effects(Effects::BOLD)
//                         .render(),
//                     number,
//                     l,
//                     Reset
//                 );
//                 out.push_str(&styled);
//             } else {
//                 // dim context lines
//                 let styled = format!("{}   {}  {}{}", dim_on(), number, l, Reset);
//                 out.push_str(&styled);
//             }
//             out.push('\n');
//         }
//         if idx > end {
//             break;
//         }
//         idx += 1;
//     }
//     Ok(out.trim_end_matches('\n').to_string())
// }
//
// fn render_panic_block(file: &str, line: u32, msg: &str) {
//     let time = dim(&now_hms());
//     let pill = level_pill(Level::Error);
//     let left = format!("{time}  {pill}");
//     let right = format!("{file}:{line}");
//
//     // Top
//     a_println!();
//     a_println!("{}", box_top_line(&left, &right));
//
//     // Message line (white)
//     if !msg.is_empty() {
//         a_println!("{}", box_body_row(&white(msg)));
//     }
//
//     // Code context
//     match format_code_context(file, line, 8) {
//         Ok(ctx) => {
//             for line in ctx.split('\n') {
//                 a_println!("{}", box_body_row(line));
//             }
//         }
//         Err(e) => {
//             a_println!(
//                 "{}",
//                 box_body_row(&white(&format!("(failed to load source context: {e})")))
//             );
//         }
//     }
//
//     // Blank row and bottom meta
//     a_println!("{}", box_empty_row());
//     a_println!(
//         "{}",
//         box_bottom_line("panic; set RUST_BACKTRACE=1 for backtrace")
//     );
// }
//
// /// --------------------------- Panic context ---------------------------------
// fn install_panic_context() {
//     std::panic::set_hook(Box::new(|info| {
//         let loc = info.location();
//         let mut msg = String::new();
//         if let Some(s) = info.payload().downcast_ref::<&str>() {
//             msg = s.to_string();
//         } else if let Some(s) = info.payload().downcast_ref::<String>() {
//             msg = s.clone();
//         }
//
//         if let Some(loc) = loc {
//             render_panic_block(loc.file(), loc.line(), &msg);
//         } else {
//             // no location available; still show a block
//             let fake_file = "<unknown>";
//             render_panic_block(fake_file, 0, &msg);
//         }
//     }));
// }
//
// fn print_code_context(file: &str, line: u32, ctx: usize) -> io::Result<()> {
//     let f = File::open(file)?;
//     let reader = BufReader::new(f);
//     let start = line.saturating_sub(ctx as u32).max(1);
//     let end = line + ctx as u32;
//
//     let mut idx: u32 = 1;
//     for l in reader.lines() {
//         let l = l?;
//         if idx >= start && idx <= end {
//             let marker = if idx == line { ">" } else { " " };
//             let number = format!("{idx:>6}");
//             let body = if idx == line {
//                 format!(
//                     "{}{}{}",
//                     Style::new().effects(Effects::BOLD).render(),
//                     l,
//                     Reset
//                 )
//             } else {
//                 l
//             };
//             a_eprintln!("  {marker} {number}  {body}");
//         }
//         if idx > end {
//             break;
//         }
//         idx += 1;
//     }
//     Ok(())
// }
//
// /// --------------------------- bail_here! macro ------------------------------
// macro_rules! bail_here {
//     ($msg:expr) => {{
//         let file = file!();
//         let line = line!();
//         log::error!("[bail] {}", $msg);
//         let _ = print_code_context(file, line, 8);
//         return Err(anyhow::anyhow!($msg));
//     }};
// }
//
// /// --------------------------- entrypoint ------------------------------------
// fn main() -> anyhow::Result<()> {
//     let cli = Cli::parse();
//     install_panic_context();
//     let level = match cli.quiet {
//         0 => LevelFilter::Info,
//         1 => LevelFilter::Warn,
//         _ => LevelFilter::Error,
//     };
//     init_logger(LevelFilter::Trace);
//
//     match cli.cmd {
//         Commands::Tables => demo_table(),
//         Commands::Progress => demo_progress(),
//         Commands::Prompts => demo_prompts()?,
//         Commands::Logging => demo_logging(),
//         Commands::Tail { file, grep } => {
//             let re = grep.map(|g| Regex::new(&g).expect("invalid regex"));
//             if let Err(e) = tail_file(file, re) {
//                 log::error!("tail failed: {e}");
//             }
//         }
//         Commands::PanicDemo => {
//             // runtime error: force a panic
//             panic!("forced panic for demo");
//         }
//         Commands::BailDemo => {
//             bail_here!("something went wrong while doing the thing");
//         }
//     }
//     Ok(())
// }
//
