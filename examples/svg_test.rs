use gpui::*;

fn main() {
    env_logger::init();

    let app = Application::new();

    app.run(move |cx| {
        let window_options = WindowOptions {
            window_bounds: Some(WindowBounds::Windowed(Bounds {
                origin: point(px(100.), px(100.)),
                size: size(px(400.), px(300.)),
            })),
            titlebar: Some(TitlebarOptions {
                title: Some("SVG Test".into()),
                appears_transparent: false,
                ..Default::default()
            }),
            ..Default::default()
        };

        cx.open_window(window_options, |window, cx| {
            cx.new(|cx| SvgTest::new(window, cx))
        }).unwrap();
    });
}

struct SvgTest {}

impl SvgTest {
    fn new(_window: &mut Window, _cx: &mut Context<Self>) -> Self {
        Self {}
    }
}

impl Render for SvgTest {
    fn render(&mut self, _: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .size_full()
            .bg(rgb(0xFFFFFF))
            .flex()
            .flex_col()
            .items_center()
            .justify_center()
            .gap_4()
            .child(
                div()
                    .text_lg()
                    .child("SVG Test")
            )
            .child(
                div()
                    .flex()
                    .gap_4()
                    .child(
                        div()
                            .flex()
                            .flex_col()
                            .items_center()
                            .gap_2()
                            .child("Direct SVG:")
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
                            .flex_col()
                            .items_center()
                            .gap_2()
                            .child("Unfold More:")
                            .child(
                                svg()
                                    .path("icons/unfold-more.svg")
                                    .size(px(48.))
                                    .text_color(rgb(0x000000))
                            )
                    )
            )
    }
}

