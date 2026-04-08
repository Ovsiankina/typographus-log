use typographus_log::panic as panicmod;

fn main() {
    // Use the logger's pill (background style) for the label.
    let style = panicmod::PanicStyle {
        use_logger_pill: true,
        context: 4,
        ..Default::default()
    };
    panicmod::install_panic_hook(style);
    panic!("panic with logger pill label");
}
