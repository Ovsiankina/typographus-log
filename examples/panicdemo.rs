use typographus_log::panic as panicmod;

fn main() {
    // Install a runtime panic hook that captures file/line and prints context.
    panicmod::install_panic_hook(panicmod::PanicStyle {
        width: 80,
        context: 4,
        ..Default::default()
    });

    // Trigger a panic to show the block; the hook resolves source path+line at runtime.
    panic!("demonstration panic with runtime source context");
}
