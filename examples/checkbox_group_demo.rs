use gpui::prelude::FluentBuilder;
use gpui::*;
use fluix::{CheckboxGroup, CheckboxOption, CheckboxGroupEvent, CheckboxGroupDirection, ComponentSize};

fn main() {
    env_logger::init();

    let app = Application::new().with_assets(fluix::Assets);

    app.run(move |cx| {
        let window_options = WindowOptions {
            window_bounds: Some(WindowBounds::Windowed(Bounds {
                origin: point(px(100.), px(100.)),
                size: size(px(800.), px(900.)),
            })),
            titlebar: Some(TitlebarOptions {
                title: Some("Fluix CheckboxGroup Demo".into()),
                appears_transparent: false,
                ..Default::default()
            }),
            ..Default::default()
        };

        cx.open_window(window_options, |window, cx| {
            cx.new(|cx| CheckboxGroupDemo::new(window, cx))
        }).unwrap();
    });
}

struct CheckboxGroupDemo {
    scroll_handle: ScrollHandle,
    vertical_group: Entity<CheckboxGroup>,
    horizontal_group: Entity<CheckboxGroup>,
    disabled_group: Entity<CheckboxGroup>,
    preselected_group: Entity<CheckboxGroup>,
    selected_fruits: Vec<String>,
    selected_hobbies: Vec<String>,
    selected_options: Vec<String>,
}

impl CheckboxGroupDemo {
    fn new(_window: &mut Window, cx: &mut Context<Self>) -> Self {
        let scroll_handle = ScrollHandle::new();

        let vertical_group = cx.new(|cx| {
            CheckboxGroup::new(cx)
                .options(vec![
                    CheckboxOption::new("apple", "Apple"),
                    CheckboxOption::new("banana", "Banana"),
                    CheckboxOption::new("orange", "Orange"),
                    CheckboxOption::new("grape", "Grape"),
                ])
                .direction(CheckboxGroupDirection::Vertical)
        });

        let horizontal_group = cx.new(|cx| {
            CheckboxGroup::new(cx)
                .options(vec![
                    CheckboxOption::new("reading", "Reading"),
                    CheckboxOption::new("coding", "Coding"),
                    CheckboxOption::new("music", "Music"),
                    CheckboxOption::new("travel", "Travel"),
                ])
                .direction(CheckboxGroupDirection::Horizontal)
        });

        let disabled_group = cx.new(|cx| {
            CheckboxGroup::new(cx)
                .options(vec![
                    CheckboxOption::new("option1", "Option 1"),
                    CheckboxOption::new("option2", "Option 2"),
                    CheckboxOption::new("option3", "Option 3"),
                ])
                .disabled(true)
        });

        let preselected_group = cx.new(|cx| {
            CheckboxGroup::new(cx)
                .options(vec![
                    CheckboxOption::new("react", "React"),
                    CheckboxOption::new("vue", "Vue.js"),
                    CheckboxOption::new("angular", "Angular"),
                    CheckboxOption::new("svelte", "Svelte"),
                ])
                .values(vec!["react".to_string(), "vue".to_string()])
        });

        cx.subscribe_in(&vertical_group, _window, |this: &mut Self, _group, event: &CheckboxGroupEvent, _window, cx| {
            let CheckboxGroupEvent::Changed(values) = event;
            this.selected_fruits = values.clone();
            println!("Selected fruits: {:?}", values);
            cx.notify();
        }).detach();

        cx.subscribe_in(&horizontal_group, _window, |this: &mut Self, _group, event: &CheckboxGroupEvent, _window, cx| {
            let CheckboxGroupEvent::Changed(values) = event;
            this.selected_hobbies = values.clone();
            println!("Selected hobbies: {:?}", values);
            cx.notify();
        }).detach();

        cx.subscribe_in(&preselected_group, _window, |this: &mut Self, _group, event: &CheckboxGroupEvent, _window, cx| {
            let CheckboxGroupEvent::Changed(values) = event;
            this.selected_options = values.clone();
            println!("Selected frameworks: {:?}", values);
            cx.notify();
        }).detach();

        Self {
            scroll_handle,
            vertical_group,
            horizontal_group,
            disabled_group,
            preselected_group,
            selected_fruits: Vec::new(),
            selected_hobbies: Vec::new(),
            selected_options: vec!["react".to_string(), "vue".to_string()],
        }
    }
}

impl Render for CheckboxGroupDemo {
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
                                            .child("Fluix CheckboxGroup Component")
                                    )
                                    .child(
                                        div()
                                            .text_sm()
                                            .text_color(rgb(0x666666))
                                            .child("Group of checkboxes for multiple selections")
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
                                                    .child("Checkboxes arranged vertically")
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
                                                    .child("Select your favorite fruits:")
                                            )
                                            .child(self.vertical_group.clone())
                                            .when(!self.selected_fruits.is_empty(), |this| {
                                                this.child(
                                                    div()
                                                        .text_xs()
                                                        .text_color(rgb(0x666666))
                                                        .child(format!("Selected: {}", self.selected_fruits.join(", ")))
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
                                                    .child("Checkboxes arranged horizontally")
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
                                                    .child("Select your hobbies:")
                                            )
                                            .child(self.horizontal_group.clone())
                                            .when(!self.selected_hobbies.is_empty(), |this| {
                                                this.child(
                                                    div()
                                                        .text_xs()
                                                        .text_color(rgb(0x666666))
                                                        .child(format!("Selected: {}", self.selected_hobbies.join(", ")))
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
                                                    .child("CheckboxGroup with disabled state")
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
                                                    .child("Pre-selected Values")
                                            )
                                            .child(
                                                div()
                                                    .text_sm()
                                                    .text_color(rgb(0x666666))
                                                    .child("CheckboxGroup with pre-selected options")
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
                                                    .child("Select your favorite frameworks:")
                                            )
                                            .child(self.preselected_group.clone())
                                            .when(!self.selected_options.is_empty(), |this| {
                                                this.child(
                                                    div()
                                                        .text_xs()
                                                        .text_color(rgb(0x666666))
                                                        .child(format!("Selected: {}", self.selected_options.join(", ")))
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
                                                    .child("CheckboxGroup with different sizes")
                                            )
                                    )
                                    .child(
                                        div()
                                            .flex()
                                            .flex_col()
                                            .gap_6()
                                            .child(
                                                cx.new(|cx| {
                                                    CheckboxGroup::new(cx)
                                                        .options(vec![
                                                            CheckboxOption::new("small1", "Small 1"),
                                                            CheckboxOption::new("small2", "Small 2"),
                                                        ])
                                                        .size(ComponentSize::Small)
                                                }).clone()
                                            )
                                            .child(
                                                cx.new(|cx| {
                                                    CheckboxGroup::new(cx)
                                                        .options(vec![
                                                            CheckboxOption::new("medium1", "Medium 1"),
                                                            CheckboxOption::new("medium2", "Medium 2"),
                                                        ])
                                                        .size(ComponentSize::Medium)
                                                }).clone()
                                            )
                                            .child(
                                                cx.new(|cx| {
                                                    CheckboxGroup::new(cx)
                                                        .options(vec![
                                                            CheckboxOption::new("large1", "Large 1"),
                                                            CheckboxOption::new("large2", "Large 2"),
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

