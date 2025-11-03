//! TextInput Chinese Input Demo
//!
//! This example demonstrates the TextInput component with Chinese input testing.

use gpui::*;
use fluix::{TextInput, TextInputEvent};

struct TextInputChineseDemo {
    text_input: Entity<TextInput>,
    current_text: String,
}

impl TextInputChineseDemo {
    fn new(_window: &mut Window, cx: &mut Context<Self>) -> Self {
        let text_input = cx.new(|cx| {
            TextInput::new(cx)
                .placeholder("请输入中文文本...")
        });

        // Subscribe to text input events
        cx.subscribe(&text_input, |this, _, event, cx| {
            match event {
                TextInputEvent::Change(text) => {
                    this.current_text = text.clone();
                    cx.notify();
                    println!("Text changed: {}", text);
                }
                TextInputEvent::Submit(text) => {
                    println!("Text submitted: {}", text);
                }
                TextInputEvent::Focus => {
                    println!("TextInput focused");
                }
                TextInputEvent::Blur => {
                    println!("TextInput blurred");
                }
            }
        }).detach();

        Self {
            text_input,
            current_text: String::new(),
        }
    }
}

impl Render for TextInputChineseDemo {
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
                    .child("TextInput 中文输入测试")
            )
            .child(
                div()
                    .text_sm()
                    .text_color(rgb(0x6c757d))
                    .child("测试单行输入框的中文输入功能")
            )
            .child(
                div()
                    .flex()
                    .items_center()
                    .gap_2()
                    .child(
                        div()
                            .text_sm()
                            .text_color(rgb(0x495057))
                            .child("输入:")
                    )
                    .child(self.text_input.clone())
            )
            .child(
                div()
                    .flex()
                    .flex_col()
                    .gap_2()
                    .child(
                        div()
                            .text_sm()
                            .text_color(rgb(0x495057))
                            .child("当前内容:")
                    )
                    .child(
                        div()
                            .p_3()
                            .bg(rgb(0xf8f9fa))
                            .border_1()
                            .border_color(rgb(0xe9ecef))
                            .rounded_md()
                            .text_sm()
                            .child(if self.current_text.is_empty() {
                                "（空）".to_string()
                            } else {
                                self.current_text.clone()
                            })
                    )
            )
            .child(
                div()
                    .text_xs()
                    .text_color(rgb(0x6c757d))
                    .child("提示：尝试输入中文字符，观察是否能正常显示")
            )
    }
}

fn main() {
    Application::new()
        .with_assets(fluix::Assets)
        .run(|cx| {
            cx.activate(true);

            let bounds = Bounds::centered(None, size(px(600.0), px(400.0)), cx);

            let _ = cx.open_window(
                WindowOptions {
                    window_bounds: Some(WindowBounds::Windowed(bounds)),
                    titlebar: Some(TitlebarOptions {
                        title: Some("TextInput Chinese Demo".into()),
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
                    app_id: Some("text_input_chinese_demo".into()),
                    is_minimizable: true,
                    is_resizable: true,
                    window_decorations: Some(WindowDecorations::Server),
                    tabbing_identifier: None,
                },
                |window, cx| {
                    cx.new(|cx| TextInputChineseDemo::new(window, cx))
                },
            );
        });
}
