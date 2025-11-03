//! TextArea Chinese Input Demo
//!
//! This example demonstrates the TextArea component with Chinese input testing,
//! specifically focusing on backspace and delete key functionality.

use gpui::*;
use fluix::{TextArea, TextAreaEvent};

struct TextAreaChineseDemo {
    text_area: Entity<TextArea>,
    current_text: String,
    event_log: Vec<String>,
}

impl TextAreaChineseDemo {
    fn new(_window: &mut Window, cx: &mut Context<Self>) -> Self {
        let text_area = cx.new(|cx| {
            TextArea::new(cx)
                .placeholder("请输入中文文本，测试退格键功能...")
                .min_height(120.0)
        });

        // Subscribe to text area events
        cx.subscribe(&text_area, |this, _, event, cx| {
            match event {
                TextAreaEvent::Change(text) => {
                    this.current_text = text.clone();
                    this.add_event_log(format!("文本变更: {}", text));
                    cx.notify();
                }
                TextAreaEvent::Submit(text) => {
                    this.add_event_log(format!("文本提交: {}", text));
                    cx.notify();
                }
                TextAreaEvent::Focus => {
                    this.add_event_log("获得焦点".to_string());
                    cx.notify();
                }
                TextAreaEvent::Blur => {
                    this.add_event_log("失去焦点".to_string());
                    cx.notify();
                }
                TextAreaEvent::Ime(text) => {
                    this.add_event_log(format!("IME输入: {}", text));
                    cx.notify();
                }
            }
        }).detach();

        Self {
            text_area,
            current_text: String::new(),
            event_log: Vec::new(),
        }
    }
    
    fn add_event_log(&mut self, event: String) {
        self.event_log.push(event);
        // Keep only the last 10 events
        if self.event_log.len() > 10 {
            self.event_log.remove(0);
        }
    }
}

impl Render for TextAreaChineseDemo {
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
                    .font_weight(FontWeight::BOLD)
                    .child("TextArea 中文输入退格键测试")
            )
            .child(
                div()
                    .text_sm()
                    .text_color(rgb(0x6c757d))
                    .child("测试多行输入框的中文输入功能，特别是退格键和删除键")
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
                            .child("输入区域:")
                    )
                    .child(self.text_area.clone())
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
                            .min_h_16()
                            .child(if self.current_text.is_empty() {
                                "（空）".to_string()
                            } else {
                                self.current_text.clone()
                            })
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
                            .text_color(rgb(0x495057))
                            .child("事件日志:")
                    )
                    .child(
                        div()
                            .p_3()
                            .bg(rgb(0xf8f9fa))
                            .border_1()
                            .border_color(rgb(0xe9ecef))
                            .rounded_md()
                            .text_xs()
                            .max_h_32()
                            .overflow_y_hidden()
                            .child(
                                div()
                                    .flex()
                                    .flex_col()
                                    .gap_1()
                                    .children(
                                        self.event_log.iter().map(|event| {
                                            div().child(event.clone())
                                        })
                                    )
                            )
                    )
            )
            .child(
                div()
                    .flex()
                    .flex_col()
                    .gap_2()
                    .p_3()
                    .bg(rgb(0xe3f2fd))
                    .border_1()
                    .border_color(rgb(0x2196f3))
                    .rounded_md()
                    .child(
                        div()
                            .text_sm()
                            .font_weight(FontWeight::BOLD)
                            .text_color(rgb(0x1976d2))
                            .child("测试说明:")
                    )
                    .child(
                        div()
                            .text_xs()
                            .text_color(rgb(0x1976d2))
                            .child("1. 输入中文字符（如：你好世界）")
                    )
                    .child(
                        div()
                            .text_xs()
                            .text_color(rgb(0x1976d2))
                            .child("2. 使用退格键删除字符")
                    )
                    .child(
                        div()
                            .text_xs()
                            .text_color(rgb(0x1976d2))
                            .child("3. 使用Delete键删除字符")
                    )
                    .child(
                        div()
                            .text_xs()
                            .text_color(rgb(0x1976d2))
                            .child("4. 混合输入中英文字符并测试删除")
                    )
                    .child(
                        div()
                            .text_xs()
                            .text_color(rgb(0x1976d2))
                            .child("5. 观察是否有崩溃或异常行为")
                    )
            )
    }
}

fn main() {
    Application::new()
        .with_assets(fluix::Assets)
        .run(|cx| {
            cx.activate(true);

            let bounds = Bounds::centered(None, size(px(700.0), px(600.0)), cx);

            let _ = cx.open_window(
                WindowOptions {
                    window_bounds: Some(WindowBounds::Windowed(bounds)),
                    titlebar: Some(TitlebarOptions {
                        title: Some("TextArea Chinese Demo".into()),
                        appears_transparent: true,
                        ..Default::default()
                    }),
                    window_background: WindowBackgroundAppearance::Opaque,
                    focus: true,
                    show: true,
                    kind: WindowKind::Normal,
                    is_movable: true,
                    display_id: None,
                    window_min_size: Some(size(px(500.0), px(400.0))),
                    app_id: Some("text_area_chinese_demo".into()),
                    is_minimizable: true,
                    is_resizable: true,
                    window_decorations: Some(WindowDecorations::Server),
                    tabbing_identifier: None,
                },
                |window, cx| {
                    cx.new(|cx| TextAreaChineseDemo::new(window, cx))
                },
            );
        });
}
