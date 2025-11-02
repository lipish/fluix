use gpui::*;
use fluix::{Icon, IconName};

fn main() {
    env_logger::init();

    let app = Application::new().with_assets(fluix::Assets);

    app.run(move |cx| {
        let window_options = WindowOptions {
            window_bounds: Some(WindowBounds::Windowed(Bounds {
                origin: point(px(100.), px(100.)),
                size: size(px(600.), px(400.)),
            })),
            titlebar: Some(TitlebarOptions {
                title: Some("Fluix Icon Colors Demo".into()),
                appears_transparent: false,
                ..Default::default()
            }),
            ..Default::default()
        };

        cx.open_window(window_options, |window, cx| {
            cx.new(|cx| IconColorsDemo::new(window, cx))
        }).unwrap();
    });
}

struct IconColorsDemo {
    scroll_handle: ScrollHandle,
}

impl IconColorsDemo {
    fn new(_window: &mut Window, _cx: &mut Context<Self>) -> Self {
        Self {
            scroll_handle: ScrollHandle::new(),
        }
    }
}

impl Render for IconColorsDemo {
    fn render(&mut self, _: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .size_full()
            .overflow_hidden()
            .child(
                div()
                    .id("scroll-container")
                    .size_full()
                    .overflow_y_scroll()
                    .track_scroll(&self.scroll_handle)
                    .child(
                        div()
                            .flex()
                            .flex_col()
                            .w_full()
                            .bg(rgb(0xF5F5F5))
                            .p_8()
                            .gap_8()
                            .child(
                                div()
                                    .flex()
                                    .flex_col()
                                    .w_full()
                                    .max_w(px(500.))
                                    .mx_auto()
                                    .gap_8()
                                    .child(
                                        div()
                                            .flex()
                                            .flex_col()
                                            .w_full()
                                            .p_6()
                                            .bg(rgb(0xFFFFFF))
                                            .border_1()
                                            .border_color(rgb(0xE0E0E0))
                                            .rounded(px(8.))
                                            .gap_6()
                                            .child(
                                                div()
                                                    .flex()
                                                    .flex_col()
                                                    .gap_1()
                                                    .child(
                                                        div()
                                                            .text_lg()
                                                            .font_weight(FontWeight::SEMIBOLD)
                                                            .text_color(rgb(0x333333))
                                                            .child("Icon Colors")
                                                    )
                                                    .child(
                                                        div()
                                                            .text_sm()
                                                            .text_color(rgb(0x666666))
                                                            .child("Custom colors for different contexts")
                                                    )
                                            )
                                            .child(
                                                div()
                                                    .flex()
                                                    .flex_row()
                                                    .items_center()
                                                    .justify_center()
                                                    .gap_8()
                                                    .child(
                                                        div()
                                                            .flex()
                                                            .flex_col()
                                                            .items_center()
                                                            .gap_2()
                                                            .child(Icon::new(IconName::Heart).large().color(rgb(0xEF4444)))
                                                            .child(
                                                                div()
                                                                    .text_xs()
                                                                    .text_color(rgb(0x666666))
                                                                    .child("Red")
                                                            )
                                                    )
                                                    .child(
                                                        div()
                                                            .flex()
                                                            .flex_col()
                                                            .items_center()
                                                            .gap_2()
                                                            .child(Icon::new(IconName::Success).large().color(rgb(0x22C55E)))
                                                            .child(
                                                                div()
                                                                    .text_xs()
                                                                    .text_color(rgb(0x666666))
                                                                    .child("Green")
                                                            )
                                                    )
                                                    .child(
                                                        div()
                                                            .flex()
                                                            .flex_col()
                                                            .items_center()
                                                            .gap_2()
                                                            .child(Icon::new(IconName::Info).large().color(rgb(0x3B82F6)))
                                                            .child(
                                                                div()
                                                                    .text_xs()
                                                                    .text_color(rgb(0x666666))
                                                                    .child("Blue")
                                                            )
                                                    )
                                                    .child(
                                                        div()
                                                            .flex()
                                                            .flex_col()
                                                            .items_center()
                                                            .gap_2()
                                                            .child(Icon::new(IconName::Warning).large().color(rgb(0xF59E0B)))
                                                            .child(
                                                                div()
                                                                    .text_xs()
                                                                    .text_color(rgb(0x666666))
                                                                    .child("Orange")
                                                            )
                                                    )
                                                    .child(
                                                        div()
                                                            .flex()
                                                            .flex_col()
                                                            .items_center()
                                                            .gap_2()
                                                            .child(Icon::new(IconName::Star).large().color(rgb(0x8B5CF6)))
                                                            .child(
                                                                div()
                                                                    .text_xs()
                                                                    .text_color(rgb(0x666666))
                                                                    .child("Purple")
                                                            )
                                                    )
                                            )
                                    )
                            )
                    )
            )
    }
}

