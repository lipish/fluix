use fluix::*;
use gpui::*;

fn main() {
    env_logger::init();

    let app = Application::new().with_assets(fluix::Assets);

    app.run(move |cx| {
        let window_options = WindowOptions {
            window_bounds: Some(WindowBounds::Windowed(Bounds {
                origin: point(px(100.), px(100.)),
                size: size(px(900.), px(700.)),
            })),
            titlebar: Some(TitlebarOptions {
                title: Some("Select Compact Mode Demo".into()),
                appears_transparent: false,
                ..Default::default()
            }),
            ..Default::default()
        };

        cx.open_window(window_options, |window, cx| {
            cx.new(|cx| SelectCompactDemo::new(window, cx))
        })
        .unwrap();
    });
}

struct SelectCompactDemo {
    scroll_handle: ScrollHandle,
    normal_select: Entity<Select>,
    compact_select: Entity<Select>,
    normal_grouped: Entity<Select>,
    compact_grouped: Entity<Select>,
}

impl SelectCompactDemo {
    fn new(_window: &mut Window, cx: &mut Context<Self>) -> Self {
        let scroll_handle = ScrollHandle::new();

        // Options for simple select
        let options = vec![
            SelectOption::new("react", "React"),
            SelectOption::new("vue", "Vue"),
            SelectOption::new("angular", "Angular"),
            SelectOption::new("svelte", "Svelte"),
            SelectOption::new("solid", "Solid"),
            SelectOption::new("qwik", "Qwik"),
            SelectOption::new("preact", "Preact"),
            SelectOption::new("lit", "Lit"),
        ];

        // Normal spacing
        let normal_select = cx.new(|cx| {
            Select::new(cx)
                .placeholder("Normal spacing")
                .options(options.clone())
        });

        // Compact spacing
        let compact_select = cx.new(|cx| {
            Select::new(cx)
                .placeholder("Compact spacing")
                .compact()
                .options(options.clone())
        });

        // Grouped options
        let groups = vec![
            SelectOptionGroup {
                label: "Frontend Frameworks".to_string(),
                options: vec![
                    SelectOption::new("react", "React"),
                    SelectOption::new("vue", "Vue"),
                    SelectOption::new("angular", "Angular"),
                    SelectOption::new("svelte", "Svelte"),
                ],
            },
            SelectOptionGroup {
                label: "Backend Frameworks".to_string(),
                options: vec![
                    SelectOption::new("express", "Express"),
                    SelectOption::new("fastify", "Fastify"),
                    SelectOption::new("nestjs", "NestJS"),
                    SelectOption::new("koa", "Koa"),
                ],
            },
        ];

        // Normal grouped
        let normal_grouped = cx.new(|cx| {
            Select::new(cx)
                .placeholder("Normal grouped")
                .option_groups(groups.clone())
        });

        // Compact grouped
        let compact_grouped = cx.new(|cx| {
            Select::new(cx)
                .placeholder("Compact grouped")
                .compact()
                .option_groups(groups.clone())
        });

        Self {
            scroll_handle,
            normal_select,
            compact_select,
            normal_grouped,
            compact_grouped,
        }
    }
}

impl Render for SelectCompactDemo {
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
                                div()
                                    .flex()
                                    .flex_col()
                                    .gap_2()
                                    .child(
                                        div()
                                            .text_2xl()
                                            .font_weight(FontWeight::BOLD)
                                            .text_color(rgb(0x111827))
                                            .child("Select Compact Mode Demo")
                                    )
                                    .child(
                                        div()
                                            .text_sm()
                                            .text_color(rgb(0x6B7280))
                                            .child("Compare normal and compact spacing for dropdown items")
                                    )
                            )
                            .child(
                                div()
                                    .flex()
                                    .flex_col()
                                    .w_full()
                                    .max_w(px(700.))
                                    .p_6()
                                    .bg(rgb(0xFFFFFF))
                                    .border_1()
                                    .border_color(rgb(0xE5E7EB))
                                    .rounded(px(12.))
                                    .gap_6()
                                    .child(
                                        div()
                                            .text_lg()
                                            .font_weight(FontWeight::SEMIBOLD)
                                            .text_color(rgb(0x111827))
                                            .child("Simple Options")
                                    )
                                    .child(
                                        div()
                                            .flex()
                                            .gap_4()
                                            .child(
                                                div()
                                                    .flex()
                                                    .flex_col()
                                                    .flex_1()
                                                    .gap_2()
                                                    .child(
                                                        div()
                                                            .text_sm()
                                                            .font_weight(FontWeight::MEDIUM)
                                                            .text_color(rgb(0x374151))
                                                            .child("Normal Spacing")
                                                    )
                                                    .child(self.normal_select.clone())
                                                    .child(
                                                        div()
                                                            .text_xs()
                                                            .text_color(rgb(0x9CA3AF))
                                                            .child("Padding: 8px vertical, 12px horizontal")
                                                    )
                                            )
                                            .child(
                                                div()
                                                    .flex()
                                                    .flex_col()
                                                    .flex_1()
                                                    .gap_2()
                                                    .child(
                                                        div()
                                                            .text_sm()
                                                            .font_weight(FontWeight::MEDIUM)
                                                            .text_color(rgb(0x374151))
                                                            .child("Compact Spacing")
                                                    )
                                                    .child(self.compact_select.clone())
                                                    .child(
                                                        div()
                                                            .text_xs()
                                                            .text_color(rgb(0x9CA3AF))
                                                            .child("Padding: 4px vertical, 8px horizontal")
                                                    )
                                            )
                                    )
                            )
                            .child(
                                div()
                                    .flex()
                                    .flex_col()
                                    .w_full()
                                    .max_w(px(700.))
                                    .p_6()
                                    .bg(rgb(0xFFFFFF))
                                    .border_1()
                                    .border_color(rgb(0xE5E7EB))
                                    .rounded(px(12.))
                                    .gap_6()
                                    .child(
                                        div()
                                            .text_lg()
                                            .font_weight(FontWeight::SEMIBOLD)
                                            .text_color(rgb(0x111827))
                                            .child("Grouped Options")
                                    )
                                    .child(
                                        div()
                                            .flex()
                                            .gap_4()
                                            .child(
                                                div()
                                                    .flex()
                                                    .flex_col()
                                                    .flex_1()
                                                    .gap_2()
                                                    .child(
                                                        div()
                                                            .text_sm()
                                                            .font_weight(FontWeight::MEDIUM)
                                                            .text_color(rgb(0x374151))
                                                            .child("Normal Grouped")
                                                    )
                                                    .child(self.normal_grouped.clone())
                                                    .child(
                                                        div()
                                                            .text_xs()
                                                            .text_color(rgb(0x9CA3AF))
                                                            .child("Group label: 6px vertical, 12px horizontal")
                                                    )
                                            )
                                            .child(
                                                div()
                                                    .flex()
                                                    .flex_col()
                                                    .flex_1()
                                                    .gap_2()
                                                    .child(
                                                        div()
                                                            .text_sm()
                                                            .font_weight(FontWeight::MEDIUM)
                                                            .text_color(rgb(0x374151))
                                                            .child("Compact Grouped")
                                                    )
                                                    .child(self.compact_grouped.clone())
                                                    .child(
                                                        div()
                                                            .text_xs()
                                                            .text_color(rgb(0x9CA3AF))
                                                            .child("Group label: 3px vertical, 8px horizontal")
                                                    )
                                            )
                                    )
                            )
                            .child(
                                div()
                                    .p_4()
                                    .bg(rgb(0xDCFCE7))
                                    .border_1()
                                    .border_color(rgb(0x86EFAC))
                                    .rounded(px(8.))
                                    .child(
                                        div()
                                            .flex()
                                            .flex_col()
                                            .gap_2()
                                            .child(
                                                div()
                                                    .text_sm()
                                                    .font_weight(FontWeight::SEMIBOLD)
                                                    .text_color(rgb(0x166534))
                                                    .child("✨ Compact Mode Benefits")
                                            )
                                            .child(
                                                div()
                                                    .text_sm()
                                                    .text_color(rgb(0x15803D))
                                                    .child("• Fits more items in dropdown without scrolling")
                                            )
                                            .child(
                                                div()
                                                    .text_sm()
                                                    .text_color(rgb(0x15803D))
                                                    .child("• Better for long lists (50% less vertical space)")
                                            )
                                            .child(
                                                div()
                                                    .text_sm()
                                                    .text_color(rgb(0x15803D))
                                                    .child("• Use .compact() method to enable")
                                            )
                                    )
                            )
                    )
            )
    }
}

