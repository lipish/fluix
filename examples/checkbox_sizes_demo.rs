use gpui::*;
use fluix::{Checkbox, ComponentSize};

fn main() {
    env_logger::init();

    let app = Application::new().with_assets(fluix::Assets);

    app.run(move |cx| {
        let window_options = WindowOptions {
            window_bounds: Some(WindowBounds::Windowed(Bounds {
                origin: point(px(100.), px(100.)),
                size: size(px(600.), px(600.)),
            })),
            titlebar: Some(TitlebarOptions {
                title: Some("Fluix Checkbox Sizes Demo".into()),
                appears_transparent: false,
                ..Default::default()
            }),
            ..Default::default()
        };

        cx.open_window(window_options, |window, cx| {
            cx.new(|cx| CheckboxSizesDemo::new(window, cx))
        }).unwrap();
    });
}

struct CheckboxSizesDemo {
    scroll_handle: ScrollHandle,
    xsmall: Entity<Checkbox>,
    small: Entity<Checkbox>,
    medium: Entity<Checkbox>,
    large: Entity<Checkbox>,
    xlarge: Entity<Checkbox>,
}

impl CheckboxSizesDemo {
    fn new(_window: &mut Window, cx: &mut Context<Self>) -> Self {
        let scroll_handle = ScrollHandle::new();
        let xsmall = cx.new(|cx| {
            Checkbox::new(cx)
                .label("XSmall checkbox")
                .size(ComponentSize::XSmall)
        });
        let small = cx.new(|cx| {
            Checkbox::new(cx)
                .label("Small checkbox")
                .size(ComponentSize::Small)
        });
        let medium = cx.new(|cx| {
            Checkbox::new(cx)
                .label("Medium checkbox")
                .size(ComponentSize::Medium)
        });
        let large = cx.new(|cx| {
            Checkbox::new(cx)
                .label("Large checkbox")
                .size(ComponentSize::Large)
        });
        let xlarge = cx.new(|cx| {
            Checkbox::new(cx)
                .label("XLarge checkbox")
                .size(ComponentSize::XLarge)
        });

        Self {
            scroll_handle,
            xsmall,
            small,
            medium,
            large,
            xlarge,
        }
    }
}

impl Render for CheckboxSizesDemo {
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
                            .h_full()
                            .bg(rgb(0xF5F5F5))
                            .p_8()
                            .gap_8()
                            .items_center()
                            .justify_center()
                            .child(
                                div()
                                    .flex()
                                    .flex_col()
                                    .w_full()
                                    .max_w(px(400.))
                                    .gap_4()
                                    .p_6()
                                    .bg(rgb(0xFFFFFF))
                                    .border_1()
                                    .border_color(rgb(0xE0E0E0))
                                    .rounded(px(8.))
                                    .child(
                                        div()
                                            .text_sm()
                                            .text_color(rgb(0x666666))
                                            .mb_4()
                                            .child("Checkbox Sizes")
                                    )
                                    .child(
                                        div()
                                            .flex()
                                            .flex_col()
                                            .gap_4()
                                            .child(self.xsmall.clone())
                                            .child(self.small.clone())
                                            .child(self.medium.clone())
                                            .child(self.large.clone())
                                            .child(self.xlarge.clone())
                                    )
                            )
                    )
            )
    }
}

