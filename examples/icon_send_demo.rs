use fluix::*;
use gpui::*;

fn main() {
    env_logger::init();

    let app = Application::new().with_assets(fluix::Assets);

    app.run(move |cx| {
        let window_options = WindowOptions {
            window_bounds: Some(WindowBounds::Windowed(Bounds {
                origin: point(px(100.), px(100.)),
                size: size(px(600.), px(500.)),
            })),
            titlebar: Some(TitlebarOptions {
                title: Some("Send Icon Demo".into()),
                appears_transparent: false,
                ..Default::default()
            }),
            ..Default::default()
        };

        cx.open_window(window_options, |window, cx| {
            cx.new(|cx| IconSendDemo::new(window, cx))
        })
        .unwrap();
    });
}

struct IconSendDemo {
    scroll_handle: ScrollHandle,
}

impl IconSendDemo {
    fn new(_window: &mut Window, _cx: &mut Context<Self>) -> Self {
        Self {
            scroll_handle: ScrollHandle::new(),
        }
    }
}

impl Render for IconSendDemo {
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
                            .bg(rgb(0xF5F5F5))
                            .p_8()
                            .gap_8()
            .child(
                div()
                    .flex()
                    .flex_col()
                    .gap_2()
                    .child(
                        div()
                            .text_2xl()
                            .font_weight(FontWeight::BOLD)
                            .text_color(rgb(0x111827))
                            .child("Send Icon Demo")
                    )
                    .child(
                        div()
                            .text_sm()
                            .text_color(rgb(0x6B7280))
                            .child("Icons are square by default (equal width and height)")
                    )
            )
            .child(
                div()
                    .flex()
                    .flex_col()
                    .w_full()
                    .max_w(px(500.))
                    .p_6()
                    .bg(rgb(0xFFFFFF))
                    .border_1()
                    .border_color(rgb(0xE5E7EB))
                    .rounded(px(12.))
                    .gap_6()
                    .child(
                        div()
                            .text_lg()
                            .font_weight(FontWeight::SEMIBOLD)
                            .text_color(rgb(0x111827))
                            .child("Send Icon Sizes")
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
                                    .gap_3()
                                    .child(Icon::new(IconName::Send).xsmall())
                                    .child(
                                        div()
                                            .text_sm()
                                            .text_color(rgb(0x6B7280))
                                            .child("XSmall (12px) - Icon::new(IconName::Send).xsmall()")
                                    )
                            )
                            .child(
                                div()
                                    .flex()
                                    .items_center()
                                    .gap_3()
                                    .child(Icon::new(IconName::Send).small())
                                    .child(
                                        div()
                                            .text_sm()
                                            .text_color(rgb(0x6B7280))
                                            .child("Small (16px) - Icon::new(IconName::Send).small()")
                                    )
                            )
                            .child(
                                div()
                                    .flex()
                                    .items_center()
                                    .gap_3()
                                    .child(Icon::new(IconName::Send).medium())
                                    .child(
                                        div()
                                            .text_sm()
                                            .text_color(rgb(0x6B7280))
                                            .child("Medium (20px) - Icon::new(IconName::Send).medium()")
                                    )
                            )
                            .child(
                                div()
                                    .flex()
                                    .items_center()
                                    .gap_3()
                                    .child(Icon::new(IconName::Send).large())
                                    .child(
                                        div()
                                            .text_sm()
                                            .text_color(rgb(0x6B7280))
                                            .child("Large (24px) - Icon::new(IconName::Send).large()")
                                    )
                            )
                            .child(
                                div()
                                    .flex()
                                    .items_center()
                                    .gap_3()
                                    .child(Icon::new(IconName::Send).xlarge())
                                    .child(
                                        div()
                                            .text_sm()
                                            .text_color(rgb(0x6B7280))
                                            .child("XLarge (32px) - Icon::new(IconName::Send).xlarge()")
                                    )
                            )
                    )
            )
            .child(
                div()
                    .flex()
                    .flex_col()
                    .w_full()
                    .max_w(px(500.))
                    .p_6()
                    .bg(rgb(0xFFFFFF))
                    .border_1()
                    .border_color(rgb(0xE5E7EB))
                    .rounded(px(12.))
                    .gap_6()
                    .child(
                        div()
                            .text_lg()
                            .font_weight(FontWeight::SEMIBOLD)
                            .text_color(rgb(0x111827))
                            .child("Send Icon Colors")
                    )
                    .child(
                        div()
                            .flex()
                            .items_center()
                            .gap_4()
                            .child(
                                Icon::new(IconName::Send)
                                    .large()
                                    .color(rgb(0x3B82F6))
                            )
                            .child(
                                Icon::new(IconName::Send)
                                    .large()
                                    .color(rgb(0x10B981))
                            )
                            .child(
                                Icon::new(IconName::Send)
                                    .large()
                                    .color(rgb(0xF59E0B))
                            )
                            .child(
                                Icon::new(IconName::Send)
                                    .large()
                                    .color(rgb(0xEF4444))
                            )
                            .child(
                                Icon::new(IconName::Send)
                                    .large()
                                    .color(rgb(0x8B5CF6))
                            )
                    )
            )
            .child(
                div()
                    .p_4()
                    .bg(rgb(0xDCFCE7))
                    .border_1()
                    .border_color(rgb(0x86EFAC))
                    .rounded(px(8.))
                    .child(
                        div()
                            .flex()
                            .flex_col()
                            .gap_2()
                            .child(
                                div()
                                    .text_sm()
                                    .font_weight(FontWeight::SEMIBOLD)
                                    .text_color(rgb(0x166534))
                                    .child("✨ Icon Features")
                            )
                            .child(
                                div()
                                    .text_sm()
                                    .text_color(rgb(0x15803D))
                                    .child("• Icons are square by default (equal width and height)")
                            )
                            .child(
                                div()
                                    .text_sm()
                                    .text_color(rgb(0x15803D))
                                    .child("• Use .size() to set both width and height")
                            )
                            .child(
                                div()
                                    .text_sm()
                                    .text_color(rgb(0x15803D))
                                    .child("• 23 built-in icons including Send")
                            )
                    )
            )
                    )
            )
    }
}

