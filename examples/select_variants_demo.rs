use fluix::*;
use gpui::*;

fn main() {
    env_logger::init();

    let app = Application::new().with_assets(fluix::Assets);

    app.run(move |cx| {
        let window_options = WindowOptions {
            window_bounds: Some(WindowBounds::Windowed(Bounds {
                origin: point(px(100.), px(100.)),
                size: size(px(900.), px(800.)),
            })),
            titlebar: Some(TitlebarOptions {
                title: Some("Select Variants & Directions Demo".into()),
                appears_transparent: false,
                ..Default::default()
            }),
            ..Default::default()
        };

        cx.open_window(window_options, |window, cx| {
            cx.new(|cx| SelectVariantsDemo::new(window, cx))
        })
        .unwrap();
    });
}

struct SelectVariantsDemo {
    scroll_handle: ScrollHandle,
    default_select: Entity<Select>,
    ghost_select: Entity<Select>,
    outline_select: Entity<Select>,
    no_border_select: Entity<Select>,
    no_shadow_select: Entity<Select>,
    transparent_select: Entity<Select>,
    clean_select: Entity<Select>,
    down_select: Entity<Select>,
    up_select: Entity<Select>,
    custom_ghost: Entity<Select>,
}

impl SelectVariantsDemo {
    fn new(_window: &mut Window, cx: &mut Context<Self>) -> Self {
        let scroll_handle = ScrollHandle::new();
        let options = vec![
            SelectOption::new("react", "React"),
            SelectOption::new("vue", "Vue"),
            SelectOption::new("angular", "Angular"),
            SelectOption::new("svelte", "Svelte"),
        ];

        // Default variant
        let default_select = cx.new(|cx| {
            Select::new(cx)
                .placeholder("Default variant")
                .options(options.clone())
        });

        // Ghost variant
        let ghost_select = cx.new(|cx| {
            Select::new(cx)
                .placeholder("Ghost variant (no border, transparent)")
                .variant(SelectVariant::Ghost)
                .options(options.clone())
        });

        // Outline variant
        let outline_select = cx.new(|cx| {
            Select::new(cx)
                .placeholder("Outline variant (border only)")
                .variant(SelectVariant::Outline)
                .options(options.clone())
        });

        // No border (convenience method)
        let no_border_select = cx.new(|cx| {
            Select::new(cx)
                .placeholder("No border (convenience method)")
                .no_border()
                .options(options.clone())
        });

        // No shadow (convenience method)
        let no_shadow_select = cx.new(|cx| {
            Select::new(cx)
                .placeholder("No shadow (convenience method)")
                .no_shadow()
                .options(options.clone())
        });

        // Transparent (convenience method)
        let transparent_select = cx.new(|cx| {
            Select::new(cx)
                .placeholder("Transparent background")
                .transparent()
                .options(options.clone())
        });

        // Clean style (no border, no shadow, transparent)
        let clean_select = cx.new(|cx| {
            Select::new(cx)
                .placeholder("Clean style (all-in-one)")
                .clean()
                .options(options.clone())
        });

        // Dropdown direction: Down (default)
        let down_select = cx.new(|cx| {
            Select::new(cx)
                .placeholder("Dropdown expands DOWN")
                .dropdown_direction(DropdownDirection::Down)
                .options(options.clone())
        });

        // Dropdown direction: Up
        let up_select = cx.new(|cx| {
            Select::new(cx)
                .placeholder("Dropdown expands UP")
                .dropdown_direction(DropdownDirection::Up)
                .options(options.clone())
        });

        // Custom ghost with colors
        let custom_ghost = cx.new(|cx| {
            Select::new(cx)
                .placeholder("Custom ghost with colors")
                .variant(SelectVariant::Ghost)
                .text_color(rgb(0x999999))
                .size(ComponentSize::Small)
                .options(options.clone())
        });

        Self {
            scroll_handle,
            default_select,
            ghost_select,
            outline_select,
            no_border_select,
            no_shadow_select,
            transparent_select,
            clean_select,
            down_select,
            up_select,
            custom_ghost,
        }
    }
}

impl Render for SelectVariantsDemo {
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
                                            .text_color(rgb(0x111827))
                                            .child("Select Variants & Directions")
                                    )
                                    .child(
                                        div()
                                            .text_sm()
                                            .text_color(rgb(0x6B7280))
                                            .child("Different visual styles and dropdown directions")
                                    )
                            )
                            .child(
                                // Variants section
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
                                            .child("Visual Variants")
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
                                                            .child("Default Variant")
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
                                                            .child("Ghost Variant (No border, transparent)")
                                                    )
                                                    .child(self.ghost_select.clone())
                                                    .child(
                                                        div()
                                                            .text_xs()
                                                            .text_color(rgb(0x9CA3AF))
                                                            .child("Code: .variant(SelectVariant::Ghost)")
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
                                                            .child("Outline Variant (Border only, transparent)")
                                                    )
                                                    .child(self.outline_select.clone())
                                                    .child(
                                                        div()
                                                            .text_xs()
                                                            .text_color(rgb(0x9CA3AF))
                                                            .child("Code: .variant(SelectVariant::Outline)")
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
                                                            .child("No Border (Convenience method)")
                                                    )
                                                    .child(self.no_border_select.clone())
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
                                                            .child("No Shadow (Convenience method)")
                                                    )
                                                    .child(self.no_shadow_select.clone())
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
                                                            .child("Transparent Background (Convenience method)")
                                                    )
                                                    .child(self.transparent_select.clone())
                                                    .child(
                                                        div()
                                                            .text_xs()
                                                            .text_color(rgb(0x9CA3AF))
                                                            .child("Code: .transparent()")
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
                                                            .child("Clean Style (No border, no shadow, transparent)")
                                                    )
                                                    .child(self.clean_select.clone())
                                                    .child(
                                                        div()
                                                            .text_xs()
                                                            .text_color(rgb(0x9CA3AF))
                                                            .child("Code: .clean() - All-in-one convenience method!")
                                                    )
                                            )
                                    )
                            )
                            .child(
                                // Dropdown directions section
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
                                            .child("Dropdown Directions")
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
                                                            .child("Expand Down (Default)")
                                                    )
                                                    .child(self.down_select.clone())
                                                    .child(
                                                        div()
                                                            .text_xs()
                                                            .text_color(rgb(0x9CA3AF))
                                                            .child("Code: .dropdown_direction(DropdownDirection::Down)")
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
                                                            .child("Expand Up (Scroll down to see this)")
                                                    )
                                                    .child(self.up_select.clone())
                                                    .child(
                                                        div()
                                                            .text_xs()
                                                            .text_color(rgb(0x9CA3AF))
                                                            .child("Code: .dropdown_direction(DropdownDirection::Up)")
                                                    )
                                            )
                                    )
                            )
                            .child(
                                // Custom combination
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
                                            .child("Custom Combinations")
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
                                                    .child("Ghost + Small + Custom Text Color")
                                            )
                                            .child(self.custom_ghost.clone())
                                            .child(
                                                div()
                                                    .text_xs()
                                                    .text_color(rgb(0x9CA3AF))
                                                    .child("Code: .variant(SelectVariant::Ghost).text_color(rgb(0x999999)).size(ComponentSize::Small)")
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
                                                    .child("✨ New Features!")
                                            )
                                            .child(
                                                div()
                                                    .text_sm()
                                                    .text_color(rgb(0x15803D))
                                                    .child("• Visual variants: Default, Ghost, Outline")
                                            )
                                            .child(
                                                div()
                                                    .text_sm()
                                                    .text_color(rgb(0x15803D))
                                                    .child("• Dropdown directions: Down, Up, Auto")
                                            )
                                            .child(
                                                div()
                                                    .text_sm()
                                                    .text_color(rgb(0x15803D))
                                                    .child("• Convenience methods: .no_border(), .no_shadow(), .transparent(), .clean()")
                                            )
                                    )
                            )
                    )
            )
    }
}

