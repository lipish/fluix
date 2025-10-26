use gpui::*;
use fluix::{Button, ButtonVariant, ComponentSize};

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
                title: Some("Simple Button Test".into()),
                appears_transparent: false,
                ..Default::default()
            }),
            ..Default::default()
        };

        cx.open_window(window_options, |window, cx| {
            cx.new(|cx| SimpleTest::new(window, cx))
        }).unwrap();
    });
}

struct SimpleTest {
    primary_button: Entity<Button>,
}

impl SimpleTest {
    fn new(_window: &mut Window, cx: &mut Context<Self>) -> Self {
        let primary_button = cx.new(|_cx| {
            Button::new("Primary Button")
                .variant(ButtonVariant::Primary)
                .size(ComponentSize::Large)
        });

        Self { primary_button }
    }
}

impl Render for SimpleTest {
    fn render(&mut self, _: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .size_full()
            .flex()
            .flex_col()
            .items_center()
            .justify_center()
            .gap_4()
            .bg(rgb(0xFFFFFF))
            .child(
                div()
                    .text_xl()
                    .text_color(rgb(0x333333))
                    .child("Button should have WHITE text on PURPLE background")
            )
            .child(self.primary_button.clone())
            .child(
                div()
                    .px_6()
                    .py_3()
                    .rounded(px(8.))
                    .bg(rgb(0x696FC7))
                    .text_color(rgb(0xFFFFFF))
                    .font_weight(FontWeight::MEDIUM)
                    .child("Manual div with white text (reference)")
            )
    }
}
