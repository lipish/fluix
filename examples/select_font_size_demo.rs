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
                title: Some("Select Font Size Demo".into()),
                appears_transparent: false,
                ..Default::default()
            }),
            ..Default::default()
        };

        cx.open_window(window_options, |window, cx| {
            cx.new(|cx| SelectFontSizeDemo::new(window, cx))
        })
        .unwrap();
    });
}

struct SelectFontSizeDemo {
    scroll_handle: ScrollHandle,
    default_select: Entity<Select>,
    small_font_select: Entity<Select>,
    large_font_select: Entity<Select>,
    custom_font_select: Entity<Select>,
}

impl SelectFontSizeDemo {
    fn new(_window: &mut Window, cx: &mut Context<Self>) -> Self {
        let scroll_handle = ScrollHandle::new();
        let options = vec![
            SelectOption::new("react", "React"),
            SelectOption::new("vue", "Vue"),
            SelectOption::new("angular", "Angular"),
            SelectOption::new("svelte", "Svelte"),
        ];

        // Default select (14px font, 36px height)
        let default_select = cx.new(|cx| {
            Select::new(cx)
                .placeholder("Default (14px font)")
                .options(options.clone())
        });

        // Small font (11px) but keep Medium component size (36px height)
        let small_font_select = cx.new(|cx| {
            Select::new(cx)
                .placeholder("Small font (11px) with Medium height")
                .font_size(px(11.))
                .options(options.clone())
        });

        // Large font (18px) but keep Medium component size (36px height)
        let large_font_select = cx.new(|cx| {
            Select::new(cx)
                .placeholder("Large font (18px) with Medium height")
                .font_size(px(18.))
                .options(options.clone())
        });

        // Custom font size (12px) - perfect for matching TextInput
        let custom_font_select = cx.new(|cx| {
            Select::new(cx)
                .placeholder("Custom font (12px) - matches TextInput")
                .font_size(px(12.))
                .options(options.clone())
        });

        Self {
            scroll_handle,
            default_select,
            small_font_select,
            large_font_select,
            custom_font_select,
        }
    }
}

impl Render for SelectFontSizeDemo {
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
                    .gap_8()
                    .p_8()
                    .max_w(px(600.))
                    .w_full()
                    .bg(rgb(0xFFFFFF))
                    .rounded(px(12.))
                    .shadow(vec![BoxShadow {
                        color: rgba(0x0000000D).into(),
                        offset: point(px(0.), px(2.)),
                        blur_radius: px(8.),
                        spread_radius: px(0.),
                    }])
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
                                    .child("Select Font Size Demo")
                            )
                            .child(
                                div()
                                    .text_sm()
                                    .text_color(rgb(0x6B7280))
                                    .child("Change font size without changing component height")
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
                                            .text_color(rgb(0x374151))
                                            .child("Default Select (14px font, 36px height)")
                                    )
                                    .child(self.default_select.clone())
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
                                            .child("Small Font (11px font, 36px height)")
                                    )
                                    .child(self.small_font_select.clone())
                                    .child(
                                        div()
                                            .text_xs()
                                            .text_color(rgb(0x9CA3AF))
                                            .child("Code: .font_size(px(11.))")
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
                                            .child("Large Font (18px font, 36px height)")
                                    )
                                    .child(self.large_font_select.clone())
                                    .child(
                                        div()
                                            .text_xs()
                                            .text_color(rgb(0x9CA3AF))
                                            .child("Code: .font_size(px(18.))")
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
                                            .child("Custom Font (12px font, 36px height)")
                                    )
                                    .child(self.custom_font_select.clone())
                                    .child(
                                        div()
                                            .text_xs()
                                            .text_color(rgb(0x9CA3AF))
                                            .child("Code: .font_size(px(12.)) - Perfect for matching TextInput!")
                                    )
                            )
                    )
                    .child(
                        div()
                            .p_4()
                            .bg(rgb(0xFEF3C7))
                            .border_1()
                            .border_color(rgb(0xFCD34D))
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
                                            .text_color(rgb(0x92400E))
                                            .child("💡 New Feature!")
                                    )
                                    .child(
                                        div()
                                            .text_sm()
                                            .text_color(rgb(0x78350F))
                                            .child("You can now change the font size independently from the component size!")
                                    )
                                    .child(
                                        div()
                                            .text_xs()
                                            .text_color(rgb(0x92400E))
                                            .child("Use .font_size(px(12.)) to match TextInput font size while keeping the same component height.")
                                    )
                            )
                    )
                )
            )
        )
    }
}

