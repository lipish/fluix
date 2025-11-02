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
                title: Some("Fluix Icon Sizes Demo".into()),
                appears_transparent: false,
                ..Default::default()
            }),
            ..Default::default()
        };

        cx.open_window(window_options, |window, cx| {
            cx.new(|cx| IconSizesDemo::new(window, cx))
        }).unwrap();
    });
}

struct IconSizesDemo {
    scroll_handle: ScrollHandle,
}

impl IconSizesDemo {
    fn new(_window: &mut Window, _cx: &mut Context<Self>) -> Self {
        Self {
            scroll_handle: ScrollHandle::new(),
        }
    }
}

impl Render for IconSizesDemo {
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
                                                            .child("Icon Sizes")
                                                    )
                                                    .child(
                                                        div()
                                                            .text_sm()
                                                            .text_color(rgb(0x666666))
                                                            .child("Different sizes from XSmall to XLarge")
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
                                                            .child(Icon::new(IconName::Star).xsmall())
                                                            .child(
                                                                div()
                                                                    .text_xs()
                                                                    .text_color(rgb(0x666666))
                                                                    .child("XSmall")
                                                            )
                                                    )
                                                    .child(
                                                        div()
                                                            .flex()
                                                            .flex_col()
                                                            .items_center()
                                                            .gap_2()
                                                            .child(Icon::new(IconName::Star).small())
                                                            .child(
                                                                div()
                                                                    .text_xs()
                                                                    .text_color(rgb(0x666666))
                                                                    .child("Small")
                                                            )
                                                    )
                                                    .child(
                                                        div()
                                                            .flex()
                                                            .flex_col()
                                                            .items_center()
                                                            .gap_2()
                                                            .child(Icon::new(IconName::Star).medium())
                                                            .child(
                                                                div()
                                                                    .text_xs()
                                                                    .text_color(rgb(0x666666))
                                                                    .child("Medium")
                                                            )
                                                    )
                                                    .child(
                                                        div()
                                                            .flex()
                                                            .flex_col()
                                                            .items_center()
                                                            .gap_2()
                                                            .child(Icon::new(IconName::Star).large())
                                                            .child(
                                                                div()
                                                                    .text_xs()
                                                                    .text_color(rgb(0x666666))
                                                                    .child("Large")
                                                            )
                                                    )
                                                    .child(
                                                        div()
                                                            .flex()
                                                            .flex_col()
                                                            .items_center()
                                                            .gap_2()
                                                            .child(Icon::new(IconName::Star).xlarge())
                                                            .child(
                                                                div()
                                                                    .text_xs()
                                                                    .text_color(rgb(0x666666))
                                                                    .child("XLarge")
                                                            )
                                                    )
                                            )
                                    )
                            )
                    )
            )
    }
}

