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
                title: Some("TextInput Cursor Demo".into()),
                appears_transparent: false,
                ..Default::default()
            }),
            ..Default::default()
        };

        cx.open_window(window_options, |window, cx| {
            cx.new(|cx| TextInputCursorDemo::new(window, cx))
        })
        .unwrap();
    });
}

struct TextInputCursorDemo {
    scroll_handle: ScrollHandle,
    normal_input: Entity<TextInput>,
    password_input: Entity<TextInput>,
    prefilled_input: Entity<TextInput>,
}

impl TextInputCursorDemo {
    fn new(_window: &mut Window, cx: &mut Context<Self>) -> Self {
        let scroll_handle = ScrollHandle::new();

        // Normal input
        let normal_input = cx.new(|cx| {
            TextInput::new(cx)
                .placeholder("Type here and test cursor movement...")
        });

        // Password input
        let password_input = cx.new(|cx| {
            TextInput::new(cx)
                .placeholder("Password with cursor support")
                .password(true)
        });

        // Prefilled input
        let prefilled_input = cx.new(|cx| {
            TextInput::new(cx)
                .value("Edit this text")
        });

        Self {
            scroll_handle,
            normal_input,
            password_input,
            prefilled_input,
        }
    }
}

impl Render for TextInputCursorDemo {
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
                                            .child("TextInput Cursor Demo")
                                    )
                                    .child(
                                        div()
                                            .text_sm()
                                            .text_color(rgb(0x6B7280))
                                            .child("Test cursor positioning and movement")
                                    )
                            )
                            .child(
                                div()
                                    .flex()
                                    .flex_col()
                                    .w_full()
                                    .max_w(px(600.))
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
                                            .child("Cursor Features")
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
                                                            .child("Normal Input")
                                                    )
                                                    .child(self.normal_input.clone())
                                                    .child(
                                                        div()
                                                            .text_xs()
                                                            .text_color(rgb(0x9CA3AF))
                                                            .child("✓ Cursor follows typing position")
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
                                                            .child("Password Input")
                                                    )
                                                    .child(self.password_input.clone())
                                                    .child(
                                                        div()
                                                            .text_xs()
                                                            .text_color(rgb(0x9CA3AF))
                                                            .child("✓ Cursor works with masked text")
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
                                                            .child("Prefilled Input")
                                                    )
                                                    .child(self.prefilled_input.clone())
                                            .child(
                                                div()
                                                    .text_xs()
                                                    .text_color(rgb(0x9CA3AF))
                                                    .child("✓ Cursor blinks automatically when focused")
                                            )
                                            .child(
                                                div()
                                                    .text_xs()
                                                    .text_color(rgb(0x9CA3AF))
                                                    .child("✓ Mouse selection fully supported")
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
                                                    .child("✨ Keyboard Shortcuts")
                                            )
                                            .child(
                                                div()
                                                    .text_sm()
                                                    .text_color(rgb(0x15803D))
                                                    .child("• Left Arrow: Move cursor left")
                                            )
                                            .child(
                                                div()
                                                    .text_sm()
                                                    .text_color(rgb(0x15803D))
                                                    .child("• Right Arrow: Move cursor right")
                                            )
                                            .child(
                                                div()
                                                    .text_sm()
                                                    .text_color(rgb(0x15803D))
                                                    .child("• Home: Move cursor to start")
                                            )
                                            .child(
                                                div()
                                                    .text_sm()
                                                    .text_color(rgb(0x15803D))
                                                    .child("• End: Move cursor to end")
                                            )
                                            .child(
                                                div()
                                                    .text_sm()
                                                    .text_color(rgb(0x15803D))
                                                    .child("• Backspace: Delete before cursor")
                                            )
                                            .child(
                                                div()
                                                    .text_sm()
                                                    .text_color(rgb(0x15803D))
                                                    .child("• Delete: Delete at cursor")
                                            )
                                            .child(
                                                div()
                                                    .text_sm()
                                                    .text_color(rgb(0x15803D))
                                                    .child("• Shift + Arrow: Extend selection")
                                            )
                                            .child(
                                                div()
                                                    .text_sm()
                                                    .text_color(rgb(0x15803D))
                                                    .child("• Cmd/Ctrl + A: Select all")
                                            )
                                    )
                            )
                            .child(
                                div()
                                    .p_4()
                                    .bg(rgb(0xFFF0E6))
                                    .border_1()
                                    .border_color(rgb(0xFFB366))
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
                                                    .text_color(rgb(0x7C2D12))
                                                    .child("🖱️ Mouse Selection")
                                            )
                                            .child(
                                                div()
                                                    .text_sm()
                                                    .text_color(rgb(0x9A3412))
                                                    .child("• Click to position cursor")
                                            )
                                            .child(
                                                div()
                                                    .text_sm()
                                                    .text_color(rgb(0x9A3412))
                                                    .child("• Drag to select text")
                                            )
                                            .child(
                                                div()
                                                    .text_sm()
                                                    .text_color(rgb(0x9A3412))
                                                    .child("• Shift+Click to extend selection")
                                            )
                                    )
                            )
                            .child(
                                div()
                                    .p_4()
                                    .bg(rgb(0xDEF7FF))
                                    .border_1()
                                    .border_color(rgb(0x7DD3FC))
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
                                                    .text_color(rgb(0x075985))
                                                    .child("🔧 Fixed Issues")
                                            )
                                            .child(
                                                div()
                                                    .text_sm()
                                                    .text_color(rgb(0x0C4A6E))
                                                    .child("✓ Cursor now follows typing position")
                                            )
                                            .child(
                                                div()
                                                    .text_sm()
                                                    .text_color(rgb(0x0C4A6E))
                                                    .child("✓ Text inserts at cursor position")
                                            )
                                            .child(
                                                div()
                                                    .text_sm()
                                                    .text_color(rgb(0x0C4A6E))
                                                    .child("✓ Arrow keys move cursor")
                                            )
                                            .child(
                                                div()
                                                    .text_sm()
                                                    .text_color(rgb(0x0C4A6E))
                                                    .child("✓ Backspace/Delete work correctly")
                                            )
                                    )
                            )
                    )
            )
    }
}

