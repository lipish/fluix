use fluix::*;
use gpui::*;

fn main() {
    env_logger::init();

    let app = Application::new().with_assets(fluix::Assets);

    app.run(move |cx| {
        let window_options = WindowOptions {
            window_bounds: Some(WindowBounds::Windowed(Bounds {
                origin: point(px(100.), px(100.)),
                size: size(px(800.), px(700.)),
            })),
            titlebar: Some(TitlebarOptions {
                title: Some("Select Background Color Demo".into()),
                appears_transparent: false,
                ..Default::default()
            }),
            ..Default::default()
        };

        cx.open_window(window_options, |window, cx| {
            cx.new(|cx| SelectBgColorDemo::new(window, cx))
        })
        .unwrap();
    });
}

struct SelectBgColorDemo {
    scroll_handle: ScrollHandle,
    default_select: Entity<Select>,
    light_blue_select: Entity<Select>,
    light_green_select: Entity<Select>,
    light_yellow_select: Entity<Select>,
    light_pink_select: Entity<Select>,
    dark_select: Entity<Select>,
}

impl SelectBgColorDemo {
    fn new(_window: &mut Window, cx: &mut Context<Self>) -> Self {
        let scroll_handle = ScrollHandle::new();
        let options = vec![
            SelectOption::new("react", "React"),
            SelectOption::new("vue", "Vue"),
            SelectOption::new("angular", "Angular"),
            SelectOption::new("svelte", "Svelte"),
        ];

        // Default (white background)
        let default_select = cx.new(|cx| {
            Select::new(cx)
                .placeholder("Default (White)")
                .options(options.clone())
        });

        // Light blue background
        let light_blue_select = cx.new(|cx| {
            Select::new(cx)
                .placeholder("Light Blue Background")
                .bg_color(rgb(0xEFF6FF))  // Light blue
                .options(options.clone())
        });

        // Light green background
        let light_green_select = cx.new(|cx| {
            Select::new(cx)
                .placeholder("Light Green Background")
                .bg_color(rgb(0xF0FDF4))  // Light green
                .options(options.clone())
        });

        // Light yellow background
        let light_yellow_select = cx.new(|cx| {
            Select::new(cx)
                .placeholder("Light Yellow Background")
                .bg_color(rgb(0xFEFCE8))  // Light yellow
                .options(options.clone())
        });

        // Light pink background
        let light_pink_select = cx.new(|cx| {
            Select::new(cx)
                .placeholder("Light Pink Background")
                .bg_color(rgb(0xFDF2F8))  // Light pink
                .options(options.clone())
        });

        // Dark background with white text
        let dark_select = cx.new(|cx| {
            Select::new(cx)
                .placeholder("Dark Background with White Text")
                .bg_color(rgb(0x1F2937))  // Dark gray
                .text_color(rgb(0xFFFFFF))  // White text
                .options(options.clone())
        });

        Self {
            scroll_handle,
            default_select,
            light_blue_select,
            light_green_select,
            light_yellow_select,
            light_pink_select,
            dark_select,
        }
    }
}

impl Render for SelectBgColorDemo {
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
                                    .child("Select Background Color Demo")
                            )
                            .child(
                                div()
                                    .text_sm()
                                    .text_color(rgb(0x6B7280))
                                    .child("Customize the background color of Select components")
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
                                            .child("Default (White Background)")
                                    )
                                    .child(self.default_select.clone())
                                    .child(
                                        div()
                                            .text_xs()
                                            .text_color(rgb(0x9CA3AF))
                                            .child("No custom background color")
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
                                            .child("Light Blue Background")
                                    )
                                    .child(self.light_blue_select.clone())
                                    .child(
                                        div()
                                            .text_xs()
                                            .text_color(rgb(0x9CA3AF))
                                            .child("Code: .bg_color(rgb(0xEFF6FF))")
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
                                            .child("Light Green Background")
                                    )
                                    .child(self.light_green_select.clone())
                                    .child(
                                        div()
                                            .text_xs()
                                            .text_color(rgb(0x9CA3AF))
                                            .child("Code: .bg_color(rgb(0xF0FDF4))")
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
                                            .child("Light Yellow Background")
                                    )
                                    .child(self.light_yellow_select.clone())
                                    .child(
                                        div()
                                            .text_xs()
                                            .text_color(rgb(0x9CA3AF))
                                            .child("Code: .bg_color(rgb(0xFEFCE8))")
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
                                            .child("Light Pink Background")
                                    )
                                    .child(self.light_pink_select.clone())
                                    .child(
                                        div()
                                            .text_xs()
                                            .text_color(rgb(0x9CA3AF))
                                            .child("Code: .bg_color(rgb(0xFDF2F8))")
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
                                            .child("Dark Background with White Text")
                                    )
                                    .child(self.dark_select.clone())
                                    .child(
                                        div()
                                            .text_xs()
                                            .text_color(rgb(0x9CA3AF))
                                            .child("Code: .bg_color(rgb(0x1F2937)).text_color(rgb(0xFFFFFF))")
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
                                            .child("✨ New Feature!")
                                    )
                                    .child(
                                        div()
                                            .text_sm()
                                            .text_color(rgb(0x15803D))
                                            .child("You can now customize both background and text colors!")
                                    )
                                    .child(
                                        div()
                                            .text_xs()
                                            .text_color(rgb(0x166534))
                                            .child("Use .bg_color() for background and .text_color() for text.")
                                    )
                            )
                    )
                )
            )
        )
    }
}

