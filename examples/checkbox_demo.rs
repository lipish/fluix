use gpui::prelude::FluentBuilder;
use gpui::*;
use fluix::{Checkbox, CheckboxEvent, ComponentSize};

fn main() {
    env_logger::init();

    let app = Application::new().with_assets(fluix::Assets);

    app.run(move |cx| {
        let window_options = WindowOptions {
            window_bounds: Some(WindowBounds::Windowed(Bounds {
                origin: point(px(100.), px(100.)),
                size: size(px(800.), px(700.)),
            })),
            titlebar: Some(TitlebarOptions {
                title: Some("Fluix Checkbox Demo".into()),
                appears_transparent: false,
                ..Default::default()
            }),
            ..Default::default()
        };

        cx.open_window(window_options, |window, cx| {
            cx.new(|cx| CheckboxDemo::new(window, cx))
        }).unwrap();
    });
}

struct CheckboxDemo {
    scroll_handle: ScrollHandle,
    basic_checkbox: Entity<Checkbox>,
    checked_checkbox: Entity<Checkbox>,
    disabled_checkbox: Entity<Checkbox>,
    checked_disabled_checkbox: Entity<Checkbox>,
    checkbox_state: bool,
}

impl CheckboxDemo {
    fn new(_window: &mut Window, cx: &mut Context<Self>) -> Self {
        let scroll_handle = ScrollHandle::new();

        let basic_checkbox = cx.new(|cx| {
            Checkbox::new(cx)
                .label("Accept terms and conditions")
        });

        let checked_checkbox = cx.new(|cx| {
            Checkbox::new(cx)
                .label("Newsletter subscription")
                .checked(true)
        });

        let disabled_checkbox = cx.new(|cx| {
            Checkbox::new(cx)
                .label("Disabled checkbox")
                .disabled(true)
        });

        let checked_disabled_checkbox = cx.new(|cx| {
            Checkbox::new(cx)
                .label("Checked & disabled")
                .checked(true)
                .disabled(true)
        });

        cx.subscribe_in(&basic_checkbox, _window, |this: &mut Self, _checkbox, event: &CheckboxEvent, _window, cx| {
            let CheckboxEvent::Changed(checked) = event;
            this.checkbox_state = *checked;
            println!("Checkbox state changed: {}", checked);
            cx.notify();
        }).detach();

        Self {
            scroll_handle,
            basic_checkbox,
            checked_checkbox,
            disabled_checkbox,
            checked_disabled_checkbox,
            checkbox_state: false,
        }
    }
}

impl Render for CheckboxDemo {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
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
                                // Header
                                div()
                                    .flex()
                                    .flex_col()
                                    .gap_2()
                                    .child(
                                        div()
                                            .text_2xl()
                                            .font_weight(FontWeight::BOLD)
                                            .text_color(rgb(0x333333))
                                            .child("Fluix Checkbox Component")
                                    )
                                    .child(
                                        div()
                                            .text_sm()
                                            .text_color(rgb(0x666666))
                                            .child("Single checkbox component with various states")
                                    )
                            )
                            .child(
                                // Basic checkbox section
                                div()
                                    .flex()
                                    .flex_col()
                                    .w_full()
                                    .max_w(px(600.))
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
                                                    .child("Basic Checkbox")
                                            )
                                            .child(
                                                div()
                                                    .text_sm()
                                                    .text_color(rgb(0x666666))
                                                    .child("Standard checkbox with label")
                                            )
                                    )
                                    .child(
                                        div()
                                            .flex()
                                            .flex_col()
                                            .gap_4()
                                            .child(self.basic_checkbox.clone())
                                            .when(self.checkbox_state, |this| {
                                                this.child(
                                                    div()
                                                        .text_xs()
                                                        .text_color(rgb(0x666666))
                                                        .child("✓ Checkbox is checked")
                                                )
                                            })
                                    )
                            )
                            .child(
                                // Checkbox variants section
                                div()
                                    .flex()
                                    .flex_col()
                                    .w_full()
                                    .max_w(px(600.))
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
                                                    .child("Checkbox States")
                                            )
                                            .child(
                                                div()
                                                    .text_sm()
                                                    .text_color(rgb(0x666666))
                                                    .child("Different states and configurations")
                                            )
                                    )
                                    .child(
                                        div()
                                            .flex()
                                            .flex_col()
                                            .gap_4()
                                            .child(self.checked_checkbox.clone())
                                            .child(self.disabled_checkbox.clone())
                                            .child(self.checked_disabled_checkbox.clone())
                                    )
                            )
                            .child(
                                // Sizes section
                                div()
                                    .flex()
                                    .flex_col()
                                    .w_full()
                                    .max_w(px(600.))
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
                                                    .child("Checkbox Sizes")
                                            )
                                            .child(
                                                div()
                                                    .text_sm()
                                                    .text_color(rgb(0x666666))
                                                    .child("Different sizes for different contexts")
                                            )
                                    )
                                    .child(
                                        div()
                                            .flex()
                                            .flex_col()
                                            .gap_4()
                                            .child(
                                                cx.new(|cx| {
                                                    Checkbox::new(cx)
                                                        .label("XSmall checkbox")
                                                        .size(ComponentSize::XSmall)
                                                }).clone()
                                            )
                                            .child(
                                                cx.new(|cx| {
                                                    Checkbox::new(cx)
                                                        .label("Small checkbox")
                                                        .size(ComponentSize::Small)
                                                }).clone()
                                            )
                                            .child(
                                                cx.new(|cx| {
                                                    Checkbox::new(cx)
                                                        .label("Medium checkbox")
                                                        .size(ComponentSize::Medium)
                                                }).clone()
                                            )
                                            .child(
                                                cx.new(|cx| {
                                                    Checkbox::new(cx)
                                                        .label("Large checkbox")
                                                        .size(ComponentSize::Large)
                                                }).clone()
                                            )
                                            .child(
                                                cx.new(|cx| {
                                                    Checkbox::new(cx)
                                                        .label("XLarge checkbox")
                                                        .size(ComponentSize::XLarge)
                                                }).clone()
                                            )
                                    )
                            )
                    )
            )
    }
}

