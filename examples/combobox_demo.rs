use gpui::prelude::FluentBuilder;
use gpui::*;
use fluix::{Combobox, SelectOption, ComboboxEvent, ComponentSize};

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
                title: Some("Fluix Combobox Demo".into()),
                appears_transparent: false,
                ..Default::default()
            }),
            ..Default::default()
        };

        cx.open_window(window_options, |window, cx| {
            cx.new(|cx| ComboboxDemo::new(window, cx))
        }).unwrap();
    });
}

struct ComboboxDemo {
    scroll_handle: ScrollHandle,
    basic_combobox: Entity<Combobox>,
    pre_selected_combobox: Entity<Combobox>,
    disabled_combobox: Entity<Combobox>,
    selected_value: String,
    input_value: String,
}

impl ComboboxDemo {
    fn new(_window: &mut Window, cx: &mut Context<Self>) -> Self {
        let scroll_handle = ScrollHandle::new();

        let basic_combobox = cx.new(|cx| {
            Combobox::new(cx)
                .placeholder("Search or select a framework...")
                .options(vec![
                    SelectOption::new("react", "React"),
                    SelectOption::new("vue", "Vue.js"),
                    SelectOption::new("angular", "Angular"),
                    SelectOption::new("svelte", "Svelte"),
                    SelectOption::new("solid", "SolidJS"),
                    SelectOption::new("ember", "Ember.js"),
                    SelectOption::new("preact", "Preact"),
                ])
        });

        let pre_selected_combobox = cx.new(|cx| {
            Combobox::new(cx)
                .placeholder("Search or select a language...")
                .value("rust")
                .input_value("Rust")
                .options(vec![
                    SelectOption::new("rust", "Rust"),
                    SelectOption::new("typescript", "TypeScript"),
                    SelectOption::new("python", "Python"),
                    SelectOption::new("go", "Go"),
                    SelectOption::new("java", "Java"),
                    SelectOption::new("javascript", "JavaScript"),
                ])
        });

        let disabled_combobox = cx.new(|cx| {
            Combobox::new(cx)
                .placeholder("Disabled combobox...")
                .disabled(true)
                .options(vec![
                    SelectOption::new("option1", "Option 1"),
                    SelectOption::new("option2", "Option 2"),
                ])
        });

        cx.subscribe_in(&basic_combobox, _window, |this: &mut Self, _combobox, event: &ComboboxEvent, _window, cx| {
            match event {
                ComboboxEvent::Changed(value) => {
                    this.selected_value = value.clone();
                    println!("Selected: {}", value);
                    cx.notify();
                }
                ComboboxEvent::InputChanged(value) => {
                    this.input_value = value.clone();
                    println!("Input changed: {}", value);
                    cx.notify();
                }
            }
        }).detach();

        Self {
            scroll_handle,
            basic_combobox,
            pre_selected_combobox,
            disabled_combobox,
            selected_value: String::new(),
            input_value: String::new(),
        }
    }
}

impl Render for ComboboxDemo {
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
                                            .child("Fluix Combobox Component")
                                    )
                                    .child(
                                        div()
                                            .text_sm()
                                            .text_color(rgb(0x666666))
                                            .child("Combobox with searchable dropdown selection")
                                    )
                            )
                            .child(
                                // Basic combobox section
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
                                                    .child("Basic Combobox")
                                            )
                                            .child(
                                                div()
                                                    .text_sm()
                                                    .text_color(rgb(0x666666))
                                                    .child("Type to search or click to select")
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
                                                    .child("Select a framework:")
                                            )
                                            .child(self.basic_combobox.clone())
                                            .when(!self.selected_value.is_empty(), |this| {
                                                this.child(
                                                    div()
                                                        .text_xs()
                                                        .text_color(rgb(0x666666))
                                                        .child(format!("Selected: {}", self.selected_value))
                                                )
                                            })
                                            .when(!self.input_value.is_empty(), |this| {
                                                this.child(
                                                    div()
                                                        .text_xs()
                                                        .text_color(rgb(0x666666))
                                                        .child(format!("Input: {}", self.input_value))
                                                )
                                            })
                                    )
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
                                                    .child("Pre-selected Combobox")
                                            )
                                            .child(
                                                div()
                                                    .text_sm()
                                                    .text_color(rgb(0x666666))
                                                    .child("Combobox with pre-selected value")
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
                                                    .child("Select a programming language:")
                                            )
                                            .child(self.pre_selected_combobox.clone())
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
                                                    .child("Combobox with disabled state")
                                            )
                                    )
                                    .child(self.disabled_combobox.clone())
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
                                                    .child("Combobox with different sizes")
                                            )
                                    )
                                    .child(
                                        div()
                                            .flex()
                                            .flex_col()
                                            .gap_4()
                                            .child(
                                                cx.new(|cx| {
                                                    Combobox::new(cx)
                                                        .placeholder("Small size...")
                                                        .size(ComponentSize::Small)
                                                        .options(vec![
                                                            SelectOption::new("opt1", "Option 1"),
                                                            SelectOption::new("opt2", "Option 2"),
                                                        ])
                                                }).clone()
                                            )
                                            .child(
                                                cx.new(|cx| {
                                                    Combobox::new(cx)
                                                        .placeholder("Medium size...")
                                                        .size(ComponentSize::Medium)
                                                        .options(vec![
                                                            SelectOption::new("opt1", "Option 1"),
                                                            SelectOption::new("opt2", "Option 2"),
                                                        ])
                                                }).clone()
                                            )
                                            .child(
                                                cx.new(|cx| {
                                                    Combobox::new(cx)
                                                        .placeholder("Large size...")
                                                        .size(ComponentSize::Large)
                                                        .options(vec![
                                                            SelectOption::new("opt1", "Option 1"),
                                                            SelectOption::new("opt2", "Option 2"),
                                                        ])
                                                }).clone()
                                            )
                                    )
                            )
                    )
            )
    }
}

