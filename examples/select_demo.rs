use gpui::prelude::FluentBuilder;
use gpui::*;
use fluix::{Select, SelectOption, SelectEvent};

fn main() {
    env_logger::init();

    let app = Application::new();

    app.run(move |cx| {
        let window_options = WindowOptions {
            window_bounds: Some(WindowBounds::Windowed(Bounds {
                origin: point(px(100.), px(100.)),
                size: size(px(900.), px(700.)),
            })),
            titlebar: Some(TitlebarOptions {
                title: Some("Fluix Select Demo".into()),
                appears_transparent: false,
                ..Default::default()
            }),
            ..Default::default()
        };

        cx.open_window(window_options, |window, cx| {
            cx.new(|cx| SelectDemo::new(window, cx))
        }).unwrap();
    });
}

struct SelectDemo {
    scroll_handle: ScrollHandle,
    framework_select: Entity<Select>,
    size_select: Entity<Select>,
    disabled_select: Entity<Select>,
    selected_framework: String,
    selected_size: String,
}

impl SelectDemo {
    fn new(_window: &mut Window, cx: &mut Context<Self>) -> Self {
        let scroll_handle = ScrollHandle::new();
        // Framework select
        let framework_select = cx.new(|cx| {
            Select::new(cx)
                .placeholder("Select framework...")
                .options(vec![
                    SelectOption::new("react", "React"),
                    SelectOption::new("vue", "Vue.js"),
                    SelectOption::new("angular", "Angular"),
                    SelectOption::new("svelte", "Svelte"),
                    SelectOption::new("solid", "SolidJS"),
                ])
        });

        // Size select
        let size_select = cx.new(|cx| {
            Select::new(cx)
                .placeholder("Select size...")
                .value("medium")
                .options(vec![
                    SelectOption::new("small", "Small"),
                    SelectOption::new("medium", "Medium"),
                    SelectOption::new("large", "Large"),
                    SelectOption::new("xlarge", "Extra Large"),
                ])
        });

        // Disabled select
        let disabled_select = cx.new(|cx| {
            Select::new(cx)
                .placeholder("Disabled select...")
                .disabled(true)
                .options(vec![
                    SelectOption::new("option1", "Option 1"),
                    SelectOption::new("option2", "Option 2"),
                ])
        });

        // Subscribe to events
        cx.subscribe_in(&framework_select, _window, |this: &mut Self, _select, event: &SelectEvent, _window, cx| {
            if let SelectEvent::Changed(value) = event {
                this.selected_framework = value.clone();
                println!("Framework selected: {}", value);
                cx.notify();
            }
        }).detach();

        cx.subscribe_in(&size_select, _window, |this: &mut Self, _select, event: &SelectEvent, _window, cx| {
            if let SelectEvent::Changed(value) = event {
                this.selected_size = value.clone();
                println!("Size selected: {}", value);
                cx.notify();
            }
        }).detach();

        Self {
            scroll_handle,
            framework_select,
            size_select,
            disabled_select,
            selected_framework: String::new(),
            selected_size: "medium".to_string(),
        }
    }
}

impl Render for SelectDemo {
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
                            .child("Fluix Select Component")
                    )
                    .child(
                        div()
                            .text_sm()
                            .text_color(rgb(0x666666))
                            .child("Dropdown selection component with multiple options")
                    )
            )
            .child(
                // Select variants section
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
                                    .child("Select Variants")
                            )
                            .child(
                                div()
                                    .text_sm()
                                    .text_color(rgb(0x666666))
                                    .child("Different select configurations")
                            )
                    )
                    .child(
                        div()
                            .flex()
                            .flex_col()
                            .gap_4()
                            .child(
                                div()
                                    .flex()
                                    .flex_col()
                                    .gap_2()
                                    .child(
                                        div()
                                            .text_sm()
                                            .font_weight(FontWeight::MEDIUM)
                                            .text_color(rgb(0x333333))
                                            .child("Framework Selection")
                                    )
                                    .child(self.framework_select.clone())
                                    .when(!self.selected_framework.is_empty(), |this| {
                                        this.child(
                                            div()
                                                .text_xs()
                                                .text_color(rgb(0x666666))
                                                .child(format!("Selected: {}", self.selected_framework))
                                        )
                                    })
                            )
                            .child(
                                div()
                                    .flex()
                                    .flex_col()
                                    .gap_2()
                                    .child(
                                        div()
                                            .text_sm()
                                            .font_weight(FontWeight::MEDIUM)
                                            .text_color(rgb(0x333333))
                                            .child("Size Selection (Pre-selected)")
                                    )
                                    .child(self.size_select.clone())
                                    .child(
                                        div()
                                            .text_xs()
                                            .text_color(rgb(0x666666))
                                            .child(format!("Selected: {}", self.selected_size))
                                    )
                            )
                            .child(
                                div()
                                    .flex()
                                    .flex_col()
                                    .gap_2()
                                    .child(
                                        div()
                                            .text_sm()
                                            .font_weight(FontWeight::MEDIUM)
                                            .text_color(rgb(0x333333))
                                            .child("Disabled Select")
                                    )
                                    .child(self.disabled_select.clone())
                            )
                    )
            )
        )
        )
    }
}
