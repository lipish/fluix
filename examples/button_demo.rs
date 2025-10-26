use gpui::*;
use fluix::{Button, ButtonVariant, ComponentSize, ButtonEvent};

fn main() {
    env_logger::init();

    let app = Application::new();

    app.run(move |cx| {
        let window_options = WindowOptions {
            window_bounds: Some(WindowBounds::Windowed(Bounds {
                origin: point(px(100.), px(100.)),
                size: size(px(800.), px(700.)),
            })),
            titlebar: Some(TitlebarOptions {
                title: Some("Fluix Button Demo".into()),
                appears_transparent: false,
                ..Default::default()
            }),
            ..Default::default()
        };

        cx.open_window(window_options, |window, cx| {
            cx.new(|cx| ButtonDemo::new(window, cx))
        }).unwrap();
    });
}

struct ButtonDemo {
    click_count: usize,
    scroll_handle: ScrollHandle,
    primary_button: Entity<Button>,
    secondary_button: Entity<Button>,
    outline_button: Entity<Button>,
    text_button: Entity<Button>,
    danger_button: Entity<Button>,
    disabled_button: Entity<Button>,
    loading_button: Entity<Button>,
}

impl ButtonDemo {
    fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        let scroll_handle = ScrollHandle::new();
        let primary_button = cx.new(|_cx| {
            Button::new("Primary Button")
                .variant(ButtonVariant::Primary)
                .size(ComponentSize::Medium)
        });

        let secondary_button = cx.new(|_cx| {
            Button::new("Secondary Button")
                .variant(ButtonVariant::Secondary)
                .size(ComponentSize::Medium)
        });

        let outline_button = cx.new(|_cx| {
            Button::new("Outline Button")
                .variant(ButtonVariant::Outline)
                .size(ComponentSize::Medium)
        });

        let text_button = cx.new(|_cx| {
            Button::new("Text Button")
                .variant(ButtonVariant::Text)
                .size(ComponentSize::Medium)
        });

        let danger_button = cx.new(|_cx| {
            Button::new("Danger Button")
                .variant(ButtonVariant::Danger)
                .size(ComponentSize::Medium)
        });

        let disabled_button = cx.new(|_cx| {
            Button::new("Disabled Button")
                .variant(ButtonVariant::Primary)
                .size(ComponentSize::Medium)
                .disabled(true)
        });

        let loading_button = cx.new(|_cx| {
            Button::new("Loading...")
                .variant(ButtonVariant::Primary)
                .size(ComponentSize::Medium)
                .loading(true)
        });

        // Subscribe to button events
        let _sub1 = cx.subscribe_in(&primary_button, window, Self::on_button_click);
        let _sub2 = cx.subscribe_in(&secondary_button, window, Self::on_button_click);
        let _sub3 = cx.subscribe_in(&outline_button, window, Self::on_button_click);
        let _sub4 = cx.subscribe_in(&text_button, window, Self::on_button_click);
        let _sub5 = cx.subscribe_in(&danger_button, window, Self::on_button_click);

        Self {
            click_count: 0,
            scroll_handle,
            primary_button,
            secondary_button,
            outline_button,
            text_button,
            danger_button,
            disabled_button,
            loading_button,
        }
    }

    fn on_button_click(
        &mut self,
        _button: &Entity<Button>,
        _event: &ButtonEvent,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.click_count += 1;
        println!("🔘 Button clicked! Total clicks: {}", self.click_count);
        cx.notify();
    }
}

impl Render for ButtonDemo {
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
                    .w_full()
                    .bg(rgb(0xF5F5F5))
                    .p_8()
                    .gap_8()
                    .child(
                div()
                    .flex()
                    .flex_col()
                    .w_full()
                    .max_w(px(800.))
                    .mx_auto()
                    .gap_8()
                    // Header
                    .child(
                        div()
                            .flex()
                            .flex_col()
                            .gap_2()
                            .child(
                                div()
                                    .text_2xl()
                                    .font_weight(FontWeight::BOLD)
                                    .text_color(rgb(0x333333))
                                    .child("Fluix Button Component")
                            )
                            .child(
                                div()
                                    .text_sm()
                                    .text_color(rgb(0x666666))
                                    .child(format!("Interactive buttons with multiple variants • Total clicks: {}", self.click_count))
                            )
                    )
                    // Button Variants
                    .child(self.render_section(
                        "Button Variants",
                        "Different visual styles for various use cases"
                    ))
                    // Button Sizes
                    .child(self.render_sizes_section(cx))
                    // Full Width
                    .child(self.render_full_width_section(cx))
            )
            )
            )
    }
}

impl ButtonDemo {
    fn render_section(&self, title: &str, description: &str) -> impl IntoElement {
        let title = title.to_string();
        let description = description.to_string();

        div()
            .flex()
            .flex_col()
            .w_full()
            .p_6()
            .bg(rgb(0xFFFFFF))
            .border_1()
            .border_color(rgb(0xE0E0E0))
            .rounded(px(8.))
            .gap_4()
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
                            .child(title)
                    )
                    .child(
                        div()
                            .text_sm()
                            .text_color(rgb(0x666666))
                            .child(description)
                    )
            )
            .child(
                div()
                    .flex()
                    .flex_row()
                    .flex_wrap()
                    .gap_3()
                    .child(self.primary_button.clone())
                    .child(self.secondary_button.clone())
                    .child(self.outline_button.clone())
                    .child(self.text_button.clone())
                    .child(self.danger_button.clone())
                    .child(self.disabled_button.clone())
                    .child(self.loading_button.clone())
            )
    }

    fn render_sizes_section(&self, cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .flex()
            .flex_col()
            .w_full()
            .p_6()
            .bg(rgb(0xFFFFFF))
            .border_1()
            .border_color(rgb(0xE0E0E0))
            .rounded(px(8.))
            .gap_4()
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
                            .child("Button Sizes")
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
                    .flex_row()
                    .items_center()
                    .flex_wrap()
                    .gap_3()
                    .child(
                        cx.new(|_cx| {
                            Button::new("XSmall")
                                .variant(ButtonVariant::Primary)
                                .size(ComponentSize::XSmall)
                        }).clone()
                    )
                    .child(
                        cx.new(|_cx| {
                            Button::new("Small")
                                .variant(ButtonVariant::Primary)
                                .size(ComponentSize::Small)
                        }).clone()
                    )
                    .child(
                        cx.new(|_cx| {
                            Button::new("Medium")
                                .variant(ButtonVariant::Primary)
                                .size(ComponentSize::Medium)
                        }).clone()
                    )
                    .child(
                        cx.new(|_cx| {
                            Button::new("Large")
                                .variant(ButtonVariant::Primary)
                                .size(ComponentSize::Large)
                        }).clone()
                    )
                    .child(
                        cx.new(|_cx| {
                            Button::new("XLarge")
                                .variant(ButtonVariant::Primary)
                                .size(ComponentSize::XLarge)
                        }).clone()
                    )
            )
    }

    fn render_full_width_section(&self, cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .flex()
            .flex_col()
            .w_full()
            .p_6()
            .bg(rgb(0xFFFFFF))
            .border_1()
            .border_color(rgb(0xE0E0E0))
            .rounded(px(8.))
            .gap_4()
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
                            .child("Full Width Buttons")
                    )
                    .child(
                        div()
                            .text_sm()
                            .text_color(rgb(0x666666))
                            .child("Buttons that span the full width of their container")
                    )
            )
            .child(
                div()
                    .flex()
                    .flex_col()
                    .gap_2()
                    .child(
                        cx.new(|_cx| {
                            Button::new("Full Width Primary")
                                .variant(ButtonVariant::Primary)
                                .size(ComponentSize::Medium)
                                .full_width(true)
                        }).clone()
                    )
                    .child(
                        cx.new(|_cx| {
                            Button::new("Full Width Secondary")
                                .variant(ButtonVariant::Secondary)
                                .size(ComponentSize::Medium)
                                .full_width(true)
                        }).clone()
                    )
            )
    }
}
