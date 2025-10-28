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
                title: Some("Select Dropdown Width Demo".into()),
                appears_transparent: false,
                ..Default::default()
            }),
            ..Default::default()
        };

        cx.open_window(window_options, |window, cx| {
            cx.new(|cx| SelectWidthDemo::new(window, cx))
        })
        .unwrap();
    });
}

struct SelectWidthDemo {
    scroll_handle: ScrollHandle,
    match_trigger: Entity<Select>,
    fixed_narrow: Entity<Select>,
    fixed_wide: Entity<Select>,
    min_width: Entity<Select>,
    max_width: Entity<Select>,
    compact_narrow: Entity<Select>,
}

impl SelectWidthDemo {
    fn new(_window: &mut Window, cx: &mut Context<Self>) -> Self {
        let scroll_handle = ScrollHandle::new();

        let options = vec![
            SelectOption::new("react", "React"),
            SelectOption::new("vue", "Vue"),
            SelectOption::new("angular", "Angular"),
            SelectOption::new("svelte", "Svelte"),
        ];

        // Match trigger width (default)
        let match_trigger = cx.new(|cx| {
            Select::new(cx)
                .placeholder("Match trigger (default)")
                .options(options.clone())
        });

        // Fixed narrow width
        let fixed_narrow = cx.new(|cx| {
            Select::new(cx)
                .placeholder("Fixed 120px")
                .fixed_width(px(120.))
                .options(options.clone())
        });

        // Fixed wide width
        let fixed_wide = cx.new(|cx| {
            Select::new(cx)
                .placeholder("Fixed 400px")
                .fixed_width(px(400.))
                .options(options.clone())
        });

        // Minimum width
        let min_width = cx.new(|cx| {
            Select::new(cx)
                .placeholder("Min 300px")
                .min_width(px(300.))
                .options(options.clone())
        });

        // Maximum width
        let max_width = cx.new(|cx| {
            Select::new(cx)
                .placeholder("Max 200px")
                .max_width(px(200.))
                .options(options.clone())
        });

        // Compact + narrow width
        let compact_narrow = cx.new(|cx| {
            Select::new(cx)
                .placeholder("Compact + 100px")
                .compact()
                .fixed_width(px(100.))
                .align_right()
                .options(options.clone())
        });

        Self {
            scroll_handle,
            match_trigger,
            fixed_narrow,
            fixed_wide,
            min_width,
            max_width,
            compact_narrow,
        }
    }
}

impl Render for SelectWidthDemo {
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
                                            .child("Select Dropdown Width Demo")
                                    )
                                    .child(
                                        div()
                                            .text_sm()
                                            .text_color(rgb(0x6B7280))
                                            .child("Control dropdown menu width with fixed, min, or max constraints")
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
                                            .child("Width Options")
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
                                                            .text_color(rgb(0x374151))
                                                            .child("Match Trigger Width (Default)")
                                                    )
                                                    .child(self.match_trigger.clone())
                                                    .child(
                                                        div()
                                                            .text_xs()
                                                            .text_color(rgb(0x9CA3AF))
                                                            .child("Code: Select::new(cx) - Dropdown matches trigger width")
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
                                                            .text_color(rgb(0x374151))
                                                            .child("Fixed Width (Narrow)")
                                                    )
                                                    .child(self.fixed_narrow.clone())
                                                    .child(
                                                        div()
                                                            .text_xs()
                                                            .text_color(rgb(0x9CA3AF))
                                                            .child("Code: .fixed_width(px(120.)) or .dropdown_width(DropdownWidth::Fixed(px(120.)))")
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
                                                            .text_color(rgb(0x374151))
                                                            .child("Fixed Width (Wide)")
                                                    )
                                                    .child(self.fixed_wide.clone())
                                                    .child(
                                                        div()
                                                            .text_xs()
                                                            .text_color(rgb(0x9CA3AF))
                                                            .child("Code: .fixed_width(px(400.))")
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
                                                            .text_color(rgb(0x374151))
                                                            .child("Minimum Width")
                                                    )
                                                    .child(self.min_width.clone())
                                                    .child(
                                                        div()
                                                            .text_xs()
                                                            .text_color(rgb(0x9CA3AF))
                                                            .child("Code: .min_width(px(300.)) - At least 300px wide")
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
                                                            .text_color(rgb(0x374151))
                                                            .child("Maximum Width")
                                                    )
                                                    .child(self.max_width.clone())
                                                    .child(
                                                        div()
                                                            .text_xs()
                                                            .text_color(rgb(0x9CA3AF))
                                                            .child("Code: .max_width(px(200.)) - At most 200px wide")
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
                                            .child("Combined Features")
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
                                                    .text_color(rgb(0x374151))
                                                    .child("Compact + Narrow + Right Aligned")
                                            )
                                            .child(self.compact_narrow.clone())
                                            .child(
                                                div()
                                                    .text_xs()
                                                    .text_color(rgb(0x9CA3AF))
                                                    .child("Code: .compact().fixed_width(px(100.)).align_right()")
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
                                                    .child("✨ Dropdown Width Options")
                                            )
                                            .child(
                                                div()
                                                    .text_sm()
                                                    .text_color(rgb(0x15803D))
                                                    .child("• MatchTrigger: Dropdown matches trigger width (default)")
                                            )
                                            .child(
                                                div()
                                                    .text_sm()
                                                    .text_color(rgb(0x15803D))
                                                    .child("• Fixed: Set exact width with .fixed_width(px(n))")
                                            )
                                            .child(
                                                div()
                                                    .text_sm()
                                                    .text_color(rgb(0x15803D))
                                                    .child("• MinWidth: Set minimum width with .min_width(px(n))")
                                            )
                                            .child(
                                                div()
                                                    .text_sm()
                                                    .text_color(rgb(0x15803D))
                                                    .child("• MaxWidth: Set maximum width with .max_width(px(n))")
                                            )
                                    )
                            )
                    )
            )
    }
}

