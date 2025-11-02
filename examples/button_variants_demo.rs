use gpui::*;
use fluix::{Button, ButtonVariant, ComponentSize, ButtonEvent, Icon, IconName};

fn main() {
    env_logger::init();

    let app = Application::new().with_assets(fluix::Assets);

    app.run(move |cx| {
        let window_options = WindowOptions {
            window_bounds: Some(WindowBounds::Windowed(Bounds {
                origin: point(px(100.), px(100.)),
                size: size(px(900.), px(1000.)),
            })),
            titlebar: Some(TitlebarOptions {
                title: Some("Fluix Button Variants Demo".into()),
                appears_transparent: false,
                ..Default::default()
            }),
            ..Default::default()
        };

        cx.open_window(window_options, |window, cx| {
            cx.new(|cx| ButtonVariantsDemo::new(window, cx))
        }).unwrap();
    });
}

struct ButtonVariantsDemo {
    scroll_handle: ScrollHandle,
    // Variants
    default_button: Entity<Button>,
    outline_button: Entity<Button>,
    secondary_button: Entity<Button>,
    danger_button: Entity<Button>,
    text_button: Entity<Button>,
    // Sizes
    small_button: Entity<Button>,
    large_button: Entity<Button>,
    // States
    disabled_button: Entity<Button>,
}

impl ButtonVariantsDemo {
    fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        let scroll_handle = ScrollHandle::new();

        // Variants
        let default_button = cx.new(|_cx| {
            Button::new("Button")
                .variant(ButtonVariant::Primary)
                .size(ComponentSize::Medium)
        });

        let outline_button = cx.new(|_cx| {
            Button::new("Button")
                .variant(ButtonVariant::Outline)
                .size(ComponentSize::Medium)
        });

        let secondary_button = cx.new(|_cx| {
            Button::new("Button")
                .variant(ButtonVariant::Secondary)
                .size(ComponentSize::Medium)
        });

        let danger_button = cx.new(|_cx| {
            Button::new("Button")
                .variant(ButtonVariant::Danger)
                .size(ComponentSize::Medium)
        });

        let text_button = cx.new(|_cx| {
            Button::new("Button")
                .variant(ButtonVariant::Text)
                .size(ComponentSize::Medium)
        });

        // Sizes
        let small_button = cx.new(|_cx| {
            Button::new("Button")
                .variant(ButtonVariant::Primary)
                .size(ComponentSize::Small)
        });

        let large_button = cx.new(|_cx| {
            Button::new("Button")
                .variant(ButtonVariant::Primary)
                .size(ComponentSize::Large)
        });

        // States
        let disabled_button = cx.new(|_cx| {
            Button::new("Button")
                .variant(ButtonVariant::Primary)
                .size(ComponentSize::Medium)
                .disabled(true)
        });

        // Subscribe to events
        cx.subscribe_in(&default_button, window, Self::on_button_click).detach();
        cx.subscribe_in(&outline_button, window, Self::on_button_click).detach();
        cx.subscribe_in(&secondary_button, window, Self::on_button_click).detach();
        cx.subscribe_in(&danger_button, window, Self::on_button_click).detach();
        cx.subscribe_in(&text_button, window, Self::on_button_click).detach();
        cx.subscribe_in(&small_button, window, Self::on_button_click).detach();
        cx.subscribe_in(&large_button, window, Self::on_button_click).detach();

        Self {
            scroll_handle,
            default_button,
            outline_button,
            secondary_button,
            danger_button,
            text_button,
            small_button,
            large_button,
            disabled_button,
        }
    }

    fn on_button_click(
        &mut self,
        _button: &Entity<Button>,
        _event: &ButtonEvent,
        _window: &mut Window,
        _cx: &mut Context<Self>,
    ) {
        println!("Button clicked!");
    }
}

impl Render for ButtonVariantsDemo {
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
                            .bg(rgb(0xFFFFFF))
                            .p_8()
                            .gap_6()
                            .child(
                                div()
                                    .flex()
                                    .flex_col()
                                    .w_full()
                                    .max_w(px(800.))
                                    .mx_auto()
                                    .gap_6()
                                    // Header
                                    .child(
                                        div()
                                            .flex()
                                            .flex_col()
                                            .gap_2()
                                            .child(
                                                div()
                                                    .text_3xl()
                                                    .font_weight(FontWeight::BOLD)
                                                    .text_color(rgb(0x111827))
                                                    .child("Button Variants")
                                            )
                                            .child(
                                                div()
                                                    .text_sm()
                                                    .text_color(rgb(0x6B7280))
                                                    .child("Different button styles and states")
                                            )
                                    )
                                    // Default
                                    .child(
                                        div()
                                            .flex()
                                            .flex_col()
                                            .gap_3()
                                            .child(
                                                div()
                                                    .text_lg()
                                                    .font_weight(FontWeight::SEMIBOLD)
                                                    .text_color(rgb(0x111827))
                                                    .child("Default")
                                            )
                                            .child(
                                                div()
                                                    .flex()
                                                    .w_auto()
                                                    .child(self.default_button.clone())
                                            )
                                    )
                                    // Outline
                                    .child(
                                        div()
                                            .flex()
                                            .flex_col()
                                            .gap_3()
                                            .child(
                                                div()
                                                    .text_lg()
                                                    .font_weight(FontWeight::SEMIBOLD)
                                                    .text_color(rgb(0x111827))
                                                    .child("Outline")
                                            )
                                            .child(
                                                div()
                                                    .flex()
                                                    .w_auto()
                                                    .child(self.outline_button.clone())
                                            )
                                    )
                                    // Secondary
                                    .child(
                                        div()
                                            .flex()
                                            .flex_col()
                                            .gap_3()
                                            .child(
                                                div()
                                                    .text_lg()
                                                    .font_weight(FontWeight::SEMIBOLD)
                                                    .text_color(rgb(0x111827))
                                                    .child("Secondary")
                                            )
                                            .child(
                                                div()
                                                    .flex()
                                                    .w_auto()
                                                    .child(self.secondary_button.clone())
                                            )
                                    )
                                    // Danger
                                    .child(
                                        div()
                                            .flex()
                                            .flex_col()
                                            .gap_3()
                                            .child(
                                                div()
                                                    .text_lg()
                                                    .font_weight(FontWeight::SEMIBOLD)
                                                    .text_color(rgb(0x111827))
                                                    .child("Danger")
                                            )
                                            .child(
                                                div()
                                                    .flex()
                                                    .w_auto()
                                                    .child(self.danger_button.clone())
                                            )
                                    )
                                    // Text
                                    .child(
                                        div()
                                            .flex()
                                            .flex_col()
                                            .gap_3()
                                            .child(
                                                div()
                                                    .text_lg()
                                                    .font_weight(FontWeight::SEMIBOLD)
                                                    .text_color(rgb(0x111827))
                                                    .child("Text")
                                            )
                                            .child(
                                                div()
                                                    .flex()
                                                    .w_auto()
                                                    .child(self.text_button.clone())
                                            )
                                    )
                                    // Small Size
                                    .child(
                                        div()
                                            .flex()
                                            .flex_col()
                                            .gap_3()
                                            .child(
                                                div()
                                                    .text_lg()
                                                    .font_weight(FontWeight::SEMIBOLD)
                                                    .text_color(rgb(0x111827))
                                                    .child("Small Size")
                                            )
                                            .child(
                                                div()
                                                    .flex()
                                                    .w_auto()
                                                    .child(self.small_button.clone())
                                            )
                                    )
                                    // Large Size
                                    .child(
                                        div()
                                            .flex()
                                            .flex_col()
                                            .gap_3()
                                            .child(
                                                div()
                                                    .text_lg()
                                                    .font_weight(FontWeight::SEMIBOLD)
                                                    .text_color(rgb(0x111827))
                                                    .child("Large Size")
                                            )
                                            .child(
                                                div()
                                                    .flex()
                                                    .w_auto()
                                                    .child(self.large_button.clone())
                                            )
                                    )
                                    // Disabled
                                    .child(
                                        div()
                                            .flex()
                                            .flex_col()
                                            .gap_3()
                                            .child(
                                                div()
                                                    .text_lg()
                                                    .font_weight(FontWeight::SEMIBOLD)
                                                    .text_color(rgb(0x111827))
                                                    .child("Disabled")
                                            )
                                            .child(
                                                div()
                                                    .flex()
                                                    .w_auto()
                                                    .child(self.disabled_button.clone())
                                            )
                                    )
                                    // With Icon - button with icon and text
                                    .child(
                                        div()
                                            .flex()
                                            .flex_col()
                                            .gap_3()
                                            .child(
                                                div()
                                                    .text_lg()
                                                    .font_weight(FontWeight::SEMIBOLD)
                                                    .text_color(rgb(0x111827))
                                                    .child("With Icon")
                                            )
                                            .child(
                                                div()
                                                    .flex()
                                                    .w_auto()
                                                    .gap_2()
                                                    .child(
                                                        div()
                                                            .flex()
                                                            .items_center()
                                                            .justify_center()
                                                            .gap_2()
                                                            .py(px(8.))  // Match ComponentSize::Medium padding_y
                                                            .px(px(16.))  // Match ComponentSize::Medium padding_x
                                                            .min_w(px(88.))  // Match ComponentSize::Medium min_width
                                                            .text_size(px(14.))  // Match ComponentSize::Medium font_size
                                                            .rounded(px(8.))
                                                            .bg(rgb(0x696FC7))
                                                            .text_color(rgb(0xFFFFFF))
                                                            .font_weight(FontWeight::MEDIUM)
                                                            .cursor(CursorStyle::PointingHand)
                                                            .shadow(vec![BoxShadow {
                                                                color: rgba(0x00000018).into(),
                                                                offset: point(px(0.), px(1.)),
                                                                blur_radius: px(2.),
                                                                spread_radius: px(0.),
                                                            }])
                                                            .on_mouse_down(MouseButton::Left, cx.listener(|_this, _event: &MouseDownEvent, _window, _cx| {
                                                                println!("Button with Send icon clicked!");
                                                            }))
                                                            .child(
                                                                Icon::new(IconName::Send)
                                                                    .size(fluix::IconSize::Small)
                                                                    .color(rgb(0xFFFFFF))
                                                            )
                                                            .child("Send")
                                                    )
                                                    .child(
                                                        div()
                                                            .flex()
                                                            .items_center()
                                                            .justify_center()
                                                            .gap_2()
                                                            .py(px(8.))  // Match ComponentSize::Medium padding_y
                                                            .px(px(16.))  // Match ComponentSize::Medium padding_x
                                                            .min_w(px(88.))  // Match ComponentSize::Medium min_width
                                                            .text_size(px(14.))  // Match ComponentSize::Medium font_size
                                                            .rounded(px(8.))
                                                            .bg(rgb(0x10B981))
                                                            .text_color(rgb(0xFFFFFF))
                                                            .font_weight(FontWeight::MEDIUM)
                                                            .cursor(CursorStyle::PointingHand)
                                                            .shadow(vec![BoxShadow {
                                                                color: rgba(0x00000018).into(),
                                                                offset: point(px(0.), px(1.)),
                                                                blur_radius: px(2.),
                                                                spread_radius: px(0.),
                                                            }])
                                                            .on_mouse_down(MouseButton::Left, cx.listener(|_this, _event: &MouseDownEvent, _window, _cx| {
                                                                println!("Button with Check icon clicked!");
                                                            }))
                                                            .child(
                                                                Icon::new(IconName::Check)
                                                                    .size(fluix::IconSize::Small)
                                                                    .color(rgb(0xFFFFFF))
                                                            )
                                                            .child("Save")
                                                    )
                                                    .child(
                                                        div()
                                                            .flex()
                                                            .items_center()
                                                            .justify_center()
                                                            .gap_2()
                                                            .py(px(8.))  // Match ComponentSize::Medium padding_y
                                                            .px(px(16.))  // Match ComponentSize::Medium padding_x
                                                            .min_w(px(88.))  // Match ComponentSize::Medium min_width
                                                            .text_size(px(14.))  // Match ComponentSize::Medium font_size
                                                            .rounded(px(8.))
                                                            .bg(rgb(0xF3F4F6))
                                                            .text_color(rgb(0x374151))
                                                            .font_weight(FontWeight::MEDIUM)
                                                            .cursor(CursorStyle::PointingHand)
                                                            .shadow(vec![BoxShadow {
                                                                color: rgba(0x00000018).into(),
                                                                offset: point(px(0.), px(1.)),
                                                                blur_radius: px(2.),
                                                                spread_radius: px(0.),
                                                            }])
                                                            .on_mouse_down(MouseButton::Left, cx.listener(|_this, _event: &MouseDownEvent, _window, _cx| {
                                                                println!("Button with Plus icon clicked!");
                                                            }))
                                                            .child(
                                                                Icon::new(IconName::Plus)
                                                                    .size(fluix::IconSize::Small)
                                                                    .color(rgb(0x374151))
                                                            )
                                                            .child("Add")
                                                    )
                                            )
                                    )
                                    // Icon Button - only icon, no text
                                    .child(
                                        div()
                                            .flex()
                                            .flex_col()
                                            .gap_3()
                                            .child(
                                                div()
                                                    .text_lg()
                                                    .font_weight(FontWeight::SEMIBOLD)
                                                    .text_color(rgb(0x111827))
                                                    .child("Icon Button")
                                            )
                                            .child(
                                                div()
                                                    .flex()
                                                    .w_auto()
                                                    .gap_2()
                                                    .child(
                                                        div()
                                                            .flex()
                                                            .items_center()
                                                            .justify_center()
                                                            .size(px(36.))
                                                            .rounded(px(8.))
                                                            .bg(rgb(0x696FC7))
                                                            .cursor(CursorStyle::PointingHand)
                                                            .shadow(vec![BoxShadow {
                                                                color: rgba(0x00000018).into(),
                                                                offset: point(px(0.), px(1.)),
                                                                blur_radius: px(2.),
                                                                spread_radius: px(0.),
                                                            }])
                                                            .on_mouse_down(MouseButton::Left, cx.listener(|_this, _event: &MouseDownEvent, _window, _cx| {
                                                                println!("Icon button clicked!");
                                                            }))
                                                            .child(
                                                                Icon::new(IconName::Attachment)
                                                                    .size(fluix::IconSize::Small)
                                                                    .color(rgb(0xFFFFFF))
                                                            )
                                                    )
                                                    .child(
                                                        div()
                                                            .flex()
                                                            .items_center()
                                                            .justify_center()
                                                            .size(px(36.))
                                                            .rounded(px(8.))
                                                            .bg(rgb(0xF3F4F6))
                                                            .cursor(CursorStyle::PointingHand)
                                                            .shadow(vec![BoxShadow {
                                                                color: rgba(0x00000018).into(),
                                                                offset: point(px(0.), px(1.)),
                                                                blur_radius: px(2.),
                                                                spread_radius: px(0.),
                                                            }])
                                                            .on_mouse_down(MouseButton::Left, cx.listener(|_this, _event: &MouseDownEvent, _window, _cx| {
                                                                println!("Icon button (outline) clicked!");
                                                            }))
                                                            .child(
                                                                Icon::new(IconName::Search)
                                                                    .size(fluix::IconSize::Small)
                                                                    .color(rgb(0x374151))
                                                            )
                                                    )
                                                    .child(
                                                        div()
                                                            .flex()
                                                            .items_center()
                                                            .justify_center()
                                                            .size(px(36.))
                                                            .rounded(px(8.))
                                                            .bg(rgb(0xDC2626))
                                                            .cursor(CursorStyle::PointingHand)
                                                            .shadow(vec![BoxShadow {
                                                                color: rgba(0x00000018).into(),
                                                                offset: point(px(0.), px(1.)),
                                                                blur_radius: px(2.),
                                                                spread_radius: px(0.),
                                                            }])
                                                            .on_mouse_down(MouseButton::Left, cx.listener(|_this, _event: &MouseDownEvent, _window, _cx| {
                                                                println!("Icon button (danger) clicked!");
                                                            }))
                                                            .child(
                                                                Icon::new(IconName::Close)
                                                                    .size(fluix::IconSize::Small)
                                                                    .color(rgb(0xFFFFFF))
                                                            )
                                                    )
                                            )
                                    )
                            )
                    )
            )
    }
}

