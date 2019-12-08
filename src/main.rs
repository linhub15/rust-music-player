use orbtk::prelude::*;

fn main() {
    Application::new()
        .window(|ctx| {
            Window::create()
                .title("Rust Music Player")
                .position((100.0, 100.0))
                .size(400.0, 400.0)
                .child(TextBlock::create().text("OrbTk").build(ctx))
                .build(ctx)
        })
        .run();
}
