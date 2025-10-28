use fluix::*;
use gpui::*;

fn main() {
    env_logger::init();

    let app = Application::new().with_assets(fluix::Assets);

    app.run(move |cx| {
        let window_options = WindowOptions {
            window_bounds: Some(WindowBounds::Windowed(Bounds {
                origin: point(px(100.), px(100.)),
                size: size(px(800.), px(600.)),
            })),
            titlebar: Some(TitlebarOptions {
                title: Some("Select Clean Styles Demo".into()),
                appears_transparent: false,
                ..Default::default()
            }),
            ..Default::default()
        };

        cx.open_window(window_options, |window, cx| {
            cx.new(|cx| SelectCleanDemo::new(window, cx))
        })
        .unwrap();
    });
}

struct SelectCleanDemo {
    scroll_handle: ScrollHandle,
    default_select: Entity<Select>,
    no_border_only: Entity<Select>,
    no_shadow_only: Entity<Select>,
    both_no_border_shadow: Entity<Select>,
    clean_method: Entity<Select>,
    custom_clean: Entity<Select>,
}

impl SelectCleanDemo {
    fn new(_window: &mut Window, cx: &mut Context<Self>) -> Self {
        let scroll_handle = ScrollHandle::new();
        let options = vec![
            SelectOption::new("react", "React"),
            SelectOption::new("vue", "Vue"),
            SelectOption::new("angular", "Angular"),
        ];

        // Default (with border and shadow)
        let default_select = cx.new(|cx| {
            Select::new(cx)
                .placeholder("Default (border + shadow)")
                .options(options.clone())
        });

        // Only no border
        let no_border_only = cx.new(|cx| {
            Select::new(cx)
                .placeholder("Only no border (still has shadow)")
                .no_border()
                .options(options.clone())
        });

        // Only no shadow
        let no_shadow_only = cx.new(|cx| {
            Select::new(cx)
                .placeholder("Only no shadow (still has border)")
                .no_shadow()
                .options(options.clone())
        });

        // Both no border and no shadow
        let both_no_border_shadow = cx.new(|cx| {
            Select::new(cx)
                .placeholder("No border + No shadow (combined)")
                .no_border()
                .no_shadow()
                .options(options.clone())
        });

        // Clean method (all-in-one)
        let clean_method = cx.new(|cx| {
            Select::new(cx)
                .placeholder("Clean method (no border + no shadow + transparent)")
                .clean()
                .options(options.clone())
        });

        // Custom clean with colors
        let custom_clean = cx.new(|cx| {
            Select::new(cx)
                .placeholder("Custom clean with colors")
                .no_border()
                .no_shadow()
                .bg_color(rgb(0xF9FAFB))
                .text_color(rgb(0x666666))
                .size(ComponentSize::Small)
                .options(options.clone())
        });

        Self {
            scroll_handle,
            default_select,
            no_border_only,
            no_shadow_only,
            both_no_border_shadow,
            clean_method,
            custom_clean,
        }
    }
}

impl Render for SelectCleanDemo {
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
                                            .child("Select Clean Styles Demo")
                                    )
                                    .child(
                                        div()
                                            .text_sm()
                                            .text_color(rgb(0x6B7280))
                                            .child("Demonstrating no_border() and no_shadow() combinations")
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
                                            .child("Comparison")
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
                                                            .child("1. Default (border ✓ + shadow ✓)")
                                                    )
                                                    .child(self.default_select.clone())
                                                    .child(
                                                        div()
                                                            .text_xs()
                                                            .text_color(rgb(0x9CA3AF))
                                                            .child("Code: Select::new(cx)")
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
                                                            .child("2. Only no border (border ✗ + shadow ✓)")
                                                    )
                                                    .child(self.no_border_only.clone())
                                                    .child(
                                                        div()
                                                            .text_xs()
                                                            .text_color(rgb(0x9CA3AF))
                                                            .child("Code: .no_border()")
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
                                                            .child("3. Only no shadow (border ✓ + shadow ✗)")
                                                    )
                                                    .child(self.no_shadow_only.clone())
                                                    .child(
                                                        div()
                                                            .text_xs()
                                                            .text_color(rgb(0x9CA3AF))
                                                            .child("Code: .no_shadow()")
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
                                                            .child("4. Both no border and no shadow (border ✗ + shadow ✗)")
                                                    )
                                                    .child(self.both_no_border_shadow.clone())
                                                    .child(
                                                        div()
                                                            .text_xs()
                                                            .text_color(rgb(0x9CA3AF))
                                                            .child("Code: .no_border().no_shadow() ← YES, you can combine them!")
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
                                                            .child("5. Clean method (border ✗ + shadow ✗ + transparent ✓)")
                                                    )
                                                    .child(self.clean_method.clone())
                                                    .child(
                                                        div()
                                                            .text_xs()
                                                            .text_color(rgb(0x9CA3AF))
                                                            .child("Code: .clean() ← Shortcut for all three!")
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
                                                            .child("6. Custom clean with colors")
                                                    )
                                                    .child(self.custom_clean.clone())
                                                    .child(
                                                        div()
                                                            .text_xs()
                                                            .text_color(rgb(0x9CA3AF))
                                                            .child("Code: .no_border().no_shadow().bg_color(...).text_color(...)")
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
                                                    .child("✅ Yes! You can combine no_border() and no_shadow()")
                                            )
                                            .child(
                                                div()
                                                    .text_sm()
                                                    .text_color(rgb(0x15803D))
                                                    .child("• Use .no_border().no_shadow() to remove both")
                                            )
                                            .child(
                                                div()
                                                    .text_sm()
                                                    .text_color(rgb(0x15803D))
                                                    .child("• Or use .clean() as a shortcut (includes transparent background)")
                                            )
                                            .child(
                                                div()
                                                    .text_sm()
                                                    .text_color(rgb(0x15803D))
                                                    .child("• Perfect for embedded selects in Settings views!")
                                            )
                                    )
                            )
                    )
            )
    }
}

