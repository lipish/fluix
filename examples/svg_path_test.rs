use gpui::*;

fn main() {
    env_logger::init();

    let app = Application::new();

    app.run(move |cx| {
        let window_options = WindowOptions {
            window_bounds: Some(WindowBounds::Windowed(Bounds {
                origin: point(px(100.), px(100.)),
                size: size(px(600.), px(400.)),
            })),
            titlebar: Some(TitlebarOptions {
                title: Some("SVG Path Test".into()),
                appears_transparent: false,
                ..Default::default()
            }),
            ..Default::default()
        };

        cx.open_window(window_options, |window, cx| {
            cx.new(|cx| SvgPathTest::new(window, cx))
        }).unwrap();
    });
}

struct SvgPathTest {}

impl SvgPathTest {
    fn new(_window: &mut Window, _cx: &mut Context<Self>) -> Self {
        Self {}
    }
}

impl Render for SvgPathTest {
    fn render(&mut self, _: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .size_full()
            .bg(rgb(0xFFFFFF))
            .flex()
            .flex_col()
            .items_center()
            .justify_center()
            .gap_8()
            .child(
                div()
                    .text_xl()
                    .font_weight(FontWeight::BOLD)
                    .child("SVG Path Loading Test")
            )
            .child(
                div()
                    .flex()
                    .flex_col()
                    .gap_4()
                    .child(
                        div()
                            .flex()
                            .items_center()
                            .gap_4()
                            .child("Method 1: svg().path() with relative path:")
                            .child(
                                svg()
                                    .path("icons/arrow-down.svg")
                                    .size(px(48.))
                                    .text_color(rgb(0x000000))
                            )
                    )
                    .child(
                        div()
                            .flex()
                            .items_center()
                            .gap_4()
                            .child("Method 2: svg().path() with absolute path:")
                            .child(
                                svg()
                                    .path(concat!(env!("CARGO_MANIFEST_DIR"), "/icons/arrow-down.svg"))
                                    .size(px(48.))
                                    .text_color(rgb(0x000000))
                            )
                    )
                    .child(
                        div()
                            .flex()
                            .items_center()
                            .gap_4()
                            .child("Method 3: svg().path() with crate root:")
                            .child(
                                svg()
                                    .path("crate://fluix/icons/arrow-down.svg")
                                    .size(px(48.))
                                    .text_color(rgb(0x000000))
                            )
                    )
            )
            .child(
                div()
                    .text_sm()
                    .text_color(rgb(0x666666))
                    .child("If you see icons above, one of these methods works!")
            )
    }
}

