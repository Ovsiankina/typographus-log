use anstyle::AnsiColor;
use typographus_log::panic as panicmod;

fn main() {
    // Foreground label with custom pointer and background enabled.
    let style = panicmod::PanicStyle {
        width: 80,
        context: 3,
        label_text: "ERROR",
        label_fg: AnsiColor::White,
        label_bold: true,
        label_bg: Some(AnsiColor::Red),
        pointer: "=>",
        ..Default::default()
    };
    panicmod::install_panic_hook(style);
    panic!("panic demo with background label and custom pointer");
}
