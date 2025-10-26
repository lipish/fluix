use gpui::*;
use fluix::{Button, ButtonVariant, ComponentSize};

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
                title: Some("Color Test".into()),
                appears_transparent: false,
                ..Default::default()
            }),
            ..Default::default()
        };

        cx.open_window(window_options, |window, cx| {
            cx.new(|cx| ColorTest::new(window, cx))
        }).unwrap();
    });
}

struct ColorTest {
    primary_button: Entity<Button>,
}

impl ColorTest {
    fn new(_window: &mut Window, cx: &mut Context<Self>) -> Self {
        let primary_button = cx.new(|_cx| {
            Button::new("White Text on Purple")
                .variant(ButtonVariant::Primary)
                .size(ComponentSize::Large)
        });

        Self {
            primary_button,
        }
    }
}

impl Render for ColorTest {
    fn render(&mut self, _: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .size_full()
            .flex()
            .items_center()
            .justify_center()
            .bg(rgb(0xF5F5F5))
            .child(
                div()
                    .flex()
                    .flex_col()
                    .gap_4()
                    .child(self.primary_button.clone())
                    .child(
                        div()
                            .px_4()
                            .py_2()
                            .bg(rgb(0x696FC7))
                            .text_color(rgb(0xFFFFFF))
                            .child("Direct white text test")
                    )
            )
    }
}
