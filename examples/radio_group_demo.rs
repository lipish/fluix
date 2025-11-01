use gpui::prelude::FluentBuilder;
use gpui::*;
use fluix::{RadioGroup, RadioOption, RadioGroupEvent, RadioGroupDirection, ComponentSize};

fn main() {
    env_logger::init();

    let app = Application::new().with_assets(fluix::Assets);

    app.run(move |cx| {
        let window_options = WindowOptions {
            window_bounds: Some(WindowBounds::Windowed(Bounds {
                origin: point(px(100.), px(100.)),
                size: size(px(800.), px(800.)),
            })),
            titlebar: Some(TitlebarOptions {
                title: Some("Fluix RadioGroup Demo".into()),
                appears_transparent: false,
                ..Default::default()
            }),
            ..Default::default()
        };

        cx.open_window(window_options, |window, cx| {
            cx.new(|cx| RadioGroupDemo::new(window, cx))
        }).unwrap();
    });
}

struct RadioGroupDemo {
    scroll_handle: ScrollHandle,
    vertical_group: Entity<RadioGroup>,
    horizontal_group: Entity<RadioGroup>,
    disabled_group: Entity<RadioGroup>,
    preselected_group: Entity<RadioGroup>,
    selected_color: String,
    selected_size: String,
    selected_plan: String,
}

impl RadioGroupDemo {
    fn new(_window: &mut Window, cx: &mut Context<Self>) -> Self {
        let scroll_handle = ScrollHandle::new();

        let vertical_group = cx.new(|cx| {
            RadioGroup::new(cx)
                .options(vec![
                    RadioOption::new("red", "Red"),
                    RadioOption::new("blue", "Blue"),
                    RadioOption::new("green", "Green"),
                    RadioOption::new("yellow", "Yellow"),
                ])
                .direction(RadioGroupDirection::Vertical)
        });

        let horizontal_group = cx.new(|cx| {
            RadioGroup::new(cx)
                .options(vec![
                    RadioOption::new("small", "Small"),
                    RadioOption::new("medium", "Medium"),
                    RadioOption::new("large", "Large"),
                    RadioOption::new("xlarge", "Extra Large"),
                ])
                .direction(RadioGroupDirection::Horizontal)
        });

        let disabled_group = cx.new(|cx| {
            RadioGroup::new(cx)
                .options(vec![
                    RadioOption::new("option1", "Option 1"),
                    RadioOption::new("option2", "Option 2"),
                    RadioOption::new("option3", "Option 3"),
                ])
                .disabled(true)
        });

        let preselected_group = cx.new(|cx| {
            RadioGroup::new(cx)
                .options(vec![
                    RadioOption::new("free", "Free Plan"),
                    RadioOption::new("basic", "Basic Plan"),
                    RadioOption::new("pro", "Pro Plan"),
                    RadioOption::new("enterprise", "Enterprise Plan"),
                ])
                .value("pro")
        });

        cx.subscribe_in(&vertical_group, _window, |this: &mut Self, _group, event: &RadioGroupEvent, _window, cx| {
            let RadioGroupEvent::Changed(value) = event;
            this.selected_color = value.clone();
            println!("Selected color: {}", value);
            cx.notify();
        }).detach();

        cx.subscribe_in(&horizontal_group, _window, |this: &mut Self, _group, event: &RadioGroupEvent, _window, cx| {
            let RadioGroupEvent::Changed(value) = event;
            this.selected_size = value.clone();
            println!("Selected size: {}", value);
            cx.notify();
        }).detach();

        cx.subscribe_in(&preselected_group, _window, |this: &mut Self, _group, event: &RadioGroupEvent, _window, cx| {
            let RadioGroupEvent::Changed(value) = event;
            this.selected_plan = value.clone();
            println!("Selected plan: {}", value);
            cx.notify();
        }).detach();

        Self {
            scroll_handle,
            vertical_group,
            horizontal_group,
            disabled_group,
            preselected_group,
            selected_color: String::new(),
            selected_size: String::new(),
            selected_plan: "pro".to_string(),
        }
    }
}

impl Render for RadioGroupDemo {
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
                                            .child("Fluix RadioGroup Component")
                                    )
                                    .child(
                                        div()
                                            .text_sm()
                                            .text_color(rgb(0x666666))
                                            .child("Radio button group for single selection")
                                    )
                            )
                            .child(
                                // Vertical layout section
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
                                                    .child("Vertical Layout")
                                            )
                                            .child(
                                                div()
                                                    .text_sm()
                                                    .text_color(rgb(0x666666))
                                                    .child("Radio buttons arranged vertically")
                                            )
                                    )
                                    .child(
                                        div()
                                            .flex()
                                            .flex_col()
                                            .gap_4()
                                            .child(
                                                div()
                                                    .text_sm()
                                                    .font_weight(FontWeight::MEDIUM)
                                                    .text_color(rgb(0x333333))
                                                    .child("Select your favorite color:")
                                            )
                                            .child(self.vertical_group.clone())
                                            .when(!self.selected_color.is_empty(), |this| {
                                                this.child(
                                                    div()
                                                        .text_xs()
                                                        .text_color(rgb(0x666666))
                                                        .child(format!("Selected: {}", self.selected_color))
                                                )
                                            })
                                    )
                            )
                            .child(
                                // Horizontal layout section
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
                                                    .child("Horizontal Layout")
                                            )
                                            .child(
                                                div()
                                                    .text_sm()
                                                    .text_color(rgb(0x666666))
                                                    .child("Radio buttons arranged horizontally")
                                            )
                                    )
                                    .child(
                                        div()
                                            .flex()
                                            .flex_col()
                                            .gap_4()
                                            .child(
                                                div()
                                                    .text_sm()
                                                    .font_weight(FontWeight::MEDIUM)
                                                    .text_color(rgb(0x333333))
                                                    .child("Select size:")
                                            )
                                            .child(self.horizontal_group.clone())
                                            .when(!self.selected_size.is_empty(), |this| {
                                                this.child(
                                                    div()
                                                        .text_xs()
                                                        .text_color(rgb(0x666666))
                                                        .child(format!("Selected: {}", self.selected_size))
                                                )
                                            })
                                    )
                            )
                            .child(
                                // Disabled section
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
                                                    .child("Disabled State")
                                            )
                                            .child(
                                                div()
                                                    .text_sm()
                                                    .text_color(rgb(0x666666))
                                                    .child("RadioGroup with disabled state")
                                            )
                                    )
                                    .child(self.disabled_group.clone())
                            )
                            .child(
                                // Pre-selected section
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
                                                    .child("Pre-selected Value")
                                            )
                                            .child(
                                                div()
                                                    .text_sm()
                                                    .text_color(rgb(0x666666))
                                                    .child("RadioGroup with pre-selected option")
                                            )
                                    )
                                    .child(
                                        div()
                                            .flex()
                                            .flex_col()
                                            .gap_4()
                                            .child(
                                                div()
                                                    .text_sm()
                                                    .font_weight(FontWeight::MEDIUM)
                                                    .text_color(rgb(0x333333))
                                                    .child("Select your plan:")
                                            )
                                            .child(self.preselected_group.clone())
                                            .when(!self.selected_plan.is_empty(), |this| {
                                                this.child(
                                                    div()
                                                        .text_xs()
                                                        .text_color(rgb(0x666666))
                                                        .child(format!("Selected: {}", self.selected_plan))
                                                )
                                            })
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
                                                    .child("Different Sizes")
                                            )
                                            .child(
                                                div()
                                                    .text_sm()
                                                    .text_color(rgb(0x666666))
                                                    .child("RadioGroup with different sizes")
                                            )
                                    )
                                    .child(
                                        div()
                                            .flex()
                                            .flex_col()
                                            .gap_6()
                                            .child(
                                                cx.new(|cx| {
                                                    RadioGroup::new(cx)
                                                        .options(vec![
                                                            RadioOption::new("small1", "Small 1"),
                                                            RadioOption::new("small2", "Small 2"),
                                                        ])
                                                        .size(ComponentSize::Small)
                                                }).clone()
                                            )
                                            .child(
                                                cx.new(|cx| {
                                                    RadioGroup::new(cx)
                                                        .options(vec![
                                                            RadioOption::new("medium1", "Medium 1"),
                                                            RadioOption::new("medium2", "Medium 2"),
                                                        ])
                                                        .size(ComponentSize::Medium)
                                                }).clone()
                                            )
                                            .child(
                                                cx.new(|cx| {
                                                    RadioGroup::new(cx)
                                                        .options(vec![
                                                            RadioOption::new("large1", "Large 1"),
                                                            RadioOption::new("large2", "Large 2"),
                                                        ])
                                                        .size(ComponentSize::Large)
                                                }).clone()
                                            )
                                    )
                            )
                    )
            )
    }
}

