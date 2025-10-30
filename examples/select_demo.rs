use gpui::prelude::FluentBuilder;
use gpui::*;
use fluix::{Select, SelectOption, SelectOptionGroup, SelectEvent};

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
    multi_select: Entity<Select>,
    grouped_select: Entity<Select>,
    compact_grouped_select: Entity<Select>,
    selected_framework: String,
    selected_size: String,
    selected_languages: Vec<String>,
    selected_country: String,
    selected_country_compact: String,
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

        // Multi-select
        let multi_select = cx.new(|cx| {
            Select::new(cx)
                .placeholder("Select languages...")
                .multiple(true)
                .options(vec![
                    SelectOption::new("rust", "Rust"),
                    SelectOption::new("typescript", "TypeScript"),
                    SelectOption::new("python", "Python"),
                    SelectOption::new("go", "Go"),
                    SelectOption::new("java", "Java"),
                    SelectOption::new("cpp", "C++"),
                ])
        });

        // Grouped select
        let grouped_select = cx.new(|cx| {
            Select::new(cx)
                .placeholder("Select country...")
                .option_groups(vec![
                    SelectOptionGroup::new("North America", vec![
                        SelectOption::new("us", "United States"),
                        SelectOption::new("ca", "Canada"),
                        SelectOption::new("mx", "Mexico"),
                    ]),
                    SelectOptionGroup::new("Europe", vec![
                        SelectOption::new("uk", "United Kingdom"),
                        SelectOption::new("de", "Germany"),
                        SelectOption::new("fr", "France"),
                        SelectOption::new("es", "Spain"),
                    ]),
                    SelectOptionGroup::new("Asia", vec![
                        SelectOption::new("cn", "China"),
                        SelectOption::new("jp", "Japan"),
                        SelectOption::new("kr", "South Korea"),
                        SelectOption::new("in", "India"),
                    ]),
                ])
        });
        
        // Compact grouped select for comparison
        let compact_grouped_select = cx.new(|cx| {
            Select::new(cx)
                .placeholder("Select country (compact)...")
                .compact()
                .option_groups(vec![
                    SelectOptionGroup::new("North America", vec![
                        SelectOption::new("us", "United States"),
                        SelectOption::new("ca", "Canada"),
                        SelectOption::new("mx", "Mexico"),
                    ]),
                    SelectOptionGroup::new("Europe", vec![
                        SelectOption::new("uk", "United Kingdom"),
                        SelectOption::new("de", "Germany"),
                        SelectOption::new("fr", "France"),
                        SelectOption::new("es", "Spain"),
                    ]),
                    SelectOptionGroup::new("Asia", vec![
                        SelectOption::new("cn", "China"),
                        SelectOption::new("jp", "Japan"),
                        SelectOption::new("kr", "South Korea"),
                        SelectOption::new("in", "India"),
                    ]),
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

        cx.subscribe_in(&multi_select, _window, |this: &mut Self, _select, event: &SelectEvent, _window, cx| {
            if let SelectEvent::MultiChanged(values) = event {
                this.selected_languages = values.clone();
                println!("Languages selected: {:?}", values);
                cx.notify();
            }
        }).detach();

        cx.subscribe_in(&grouped_select, _window, |this: &mut Self, _select, event: &SelectEvent, _window, cx| {
            if let SelectEvent::Changed(value) = event {
                this.selected_country = value.clone();
                println!("Country selected: {}", value);
                cx.notify();
            }
        }).detach();

        cx.subscribe_in(&compact_grouped_select, _window, |this: &mut Self, _select, event: &SelectEvent, _window, cx| {
            if let SelectEvent::Changed(value) = event {
                this.selected_country_compact = value.clone();
                println!("Country selected (compact): {}", value);
                cx.notify();
            }
        }).detach();

        Self {
            scroll_handle,
            framework_select,
            size_select,
            disabled_select,
            multi_select,
            grouped_select,
            compact_grouped_select,
            selected_framework: String::new(),
            selected_size: "medium".to_string(),
            selected_languages: Vec::new(),
            selected_country: String::new(),
            selected_country_compact: String::new(),
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
                            .pb(px(250.))
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
            .child(
                // Grouped select section
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
                                    .child("Grouped Options")
                            )
                            .child(
                                div()
                                    .text_sm()
                                    .text_color(rgb(0x666666))
                                    .child("Options organized into groups with headers")
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
                                    .child("Country Selection")
                            )
                            .child(self.grouped_select.clone())
                            .when(!self.selected_country.is_empty(), |this| {
                                this.child(
                                    div()
                                        .text_xs()
                                        .text_color(rgb(0x666666))
                                        .child(format!("Selected: {}", self.selected_country))
                                )
                            })
                            .child(
                                div()
                                    .mt(px(4.))
                                    .flex()
                                    .flex_col()
                                    .gap_2()
                                    .child(
                                        div()
                                            .text_sm()
                                            .font_weight(FontWeight::MEDIUM)
                                            .text_color(rgb(0x333333))
                                            .child("Country Selection (Compact)")
                                    )
                                    .child(self.compact_grouped_select.clone())
                                    .when(!self.selected_country_compact.is_empty(), |this| {
                                        this.child(
                                            div()
                                                .text_xs()
                                                .text_color(rgb(0x666666))
                                                .child(format!("Selected: {}", self.selected_country_compact))
                                        )
                                    })
                            )
                    )
            )
            .child(
                // Multi-select section
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
                                    .child("Multi-Select")
                            )
                            .child(
                                div()
                                    .text_sm()
                                    .text_color(rgb(0x666666))
                                    .child("Select multiple options with checkboxes")
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
                                    .child("Programming Languages")
                            )
                            .child(self.multi_select.clone())
                            .when(!self.selected_languages.is_empty(), |this| {
                                this.child(
                                    div()
                                        .text_xs()
                                        .text_color(rgb(0x666666))
                                        .child(format!("Selected: {}", self.selected_languages.join(", ")))
                                )
                            })
                    )
            )
        )
        )
    }
}
