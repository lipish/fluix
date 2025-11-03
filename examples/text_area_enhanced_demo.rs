//! Enhanced TextArea Demo
//!
//! This example demonstrates the enhanced TextArea component with Chinese input support.

use gpui::*;
use fluix::{TextArea, TextAreaEvent};

struct EnhancedTextAreaDemo {
    text_area: Entity<TextArea>,
    ime_status: String,
}

impl EnhancedTextAreaDemo {
    fn new(_window: &mut Window, cx: &mut Context<Self>) -> Self {
        let text_area = cx.new(|cx| {
            TextArea::new(cx)
                .placeholder("请输入中文或英文文本...")
                .min_height(120.0)
                .max_height(300.0)
        });

        // Subscribe to text area events
        cx.subscribe(&text_area, |this, _, event, cx| {
            match event {
                TextAreaEvent::Change(text) => {
                    println!("Text changed: {}", text);
                }
                TextAreaEvent::Submit(text) => {
                    println!("Text submitted: {}", text);
                }
                TextAreaEvent::Ime(text) => {
                    this.ime_status = format!("检测到IME输入: {}", text);
                    cx.notify();
                    println!("IME detected: {}", text);
                }
                TextAreaEvent::Focus => {
                    println!("TextArea focused");
                }
                TextAreaEvent::Blur => {
                    println!("TextArea blurred");
                }
            }
        }).detach();

        Self {
            text_area,
            ime_status: "等待输入...".to_string(),
        }
    }
}

impl Render for EnhancedTextAreaDemo {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .flex()
            .flex_col()
            .gap_4()
            .p_6()
            .size_full()
            .bg(rgb(0xffffff))  // 白色背景
            .child(
                div()
                    .text_lg()
                    .child("增强版 TextArea 演示")
            )
            .child(
                div()
                    .text_sm()
                    .text_color(rgb(0x6c757d))
                    .child("支持中文输入法检测的多行文本输入框")
            )
            .child(self.text_area.clone())
            .child(
                div()
                    .text_sm()
                    .text_color(rgb(0x28a745))
                    .child(self.ime_status.clone())
            )
            .child(
                div()
                    .text_xs()
                    .text_color(rgb(0x6c757d))
                    .child("提示：输入中文时会触发 IME 事件")
            )
    }
}

fn main() {
    Application::new()
        .with_assets(fluix::Assets)
        .run(|cx| {
            cx.activate(true);

            let bounds = Bounds::centered(None, size(px(500.0), px(400.0)), cx);

            let _ = cx.open_window(
                WindowOptions {
                    window_bounds: Some(WindowBounds::Windowed(bounds)),
                    titlebar: Some(TitlebarOptions {
                        title: Some("Enhanced TextArea Demo".into()),
                        appears_transparent: true,
                        ..Default::default()
                    }),
                    window_background: WindowBackgroundAppearance::Opaque,
                    focus: true,
                    show: true,
                    kind: WindowKind::Normal,
                    is_movable: true,
                    display_id: None,
                    window_min_size: Some(size(px(400.0), px(300.0))),
                    app_id: Some("enhanced_text_area_demo".into()),
                    is_minimizable: true,
                    is_resizable: true,
                    window_decorations: Some(WindowDecorations::Server),
                    tabbing_identifier: None,
                },
                |window, cx| {
                    cx.new(|cx| EnhancedTextAreaDemo::new(window, cx))
                },
            );
        });
}
