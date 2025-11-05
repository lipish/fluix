use fluix::prelude::*;
use fluix::ai::display::{MessageBubble, MessageBubbleVariant};
use fluix::ai::{Message, MessageContent, MessageRole};
use chrono::Utc;

struct MessageBubbleDemo {
    scroll_handle: ScrollHandle,
    messages: Vec<Message>,
    current_variant: MessageBubbleVariant,
}

impl MessageBubbleDemo {
    fn new(_window: &mut Window, _cx: &mut Context<Self>) -> Self {
        let messages = vec![
            Message::new_user("Hello! Can you help me with a Rust programming question?"),
            Message::new_assistant(vec![
                MessageContent::Text("Of course! I'd be happy to help you with Rust programming. What specific question do you have?".to_string()),
            ]),
            Message::new_user("How do I create a vector in Rust?"),
            Message::new_assistant(vec![
                MessageContent::Text("There are several ways to create a vector in Rust. Here are the most common methods:".to_string()),
                MessageContent::Code {
                    language: "rust".to_string(),
                    code: r#"// Method 1: Using vec! macro
let mut v1 = vec![1, 2, 3, 4, 5];

// Method 2: Using Vec::new() and push
let mut v2 = Vec::new();
v2.push(1);
v2.push(2);

// Method 3: Using Vec::with_capacity for performance
let mut v3 = Vec::with_capacity(10);

// Method 4: From an array
let v4: Vec<i32> = [1, 2, 3, 4, 5].to_vec();"#.to_string(),
                },
                MessageContent::Text("The `vec!` macro is usually the most convenient for creating vectors with initial values.".to_string()),
            ]),
            Message::new_system("Code execution completed successfully."),
            Message::new_assistant(vec![
                MessageContent::Thinking("The user is asking about vectors, which is a fundamental concept in Rust. I should provide clear examples and explain the differences between the methods.".to_string()),
                MessageContent::Text("Would you like me to explain any of these methods in more detail?".to_string()),
            ]),
            Message::new_user("Can you show me an example with error handling?"),
            Message::new_assistant(vec![
                MessageContent::Text("Absolutely! Here's an example that demonstrates error handling with vectors:".to_string()),
                MessageContent::Code {
                    language: "rust".to_string(),
                    code: r#"fn safe_vector_access(v: &Vec<i32>, index: usize) -> Result<i32, String> {
    match v.get(index) {
        Some(value) => Ok(*value),
        None => Err(format!("Index {} is out of bounds", index)),
    }
}

fn main() {
    let numbers = vec![10, 20, 30, 40, 50];
    
    // Safe access
    match safe_vector_access(&numbers, 2) {
        Ok(value) => println!("Value at index 2: {}", value),
        Err(e) => println!("Error: {}", e),
    }
    
    // This will return an error
    match safe_vector_access(&numbers, 10) {
        Ok(value) => println!("Value at index 10: {}", value),
        Err(e) => println!("Error: {}", e),
    }
}"#.to_string(),
                },
                MessageContent::Terminal {
                    command: "cargo run".to_string(),
                    output: "Value at index 2: 30\nError: Index 10 is out of bounds".to_string(),
                },
            ]),
            Message {
                id: uuid::Uuid::new_v4(),
                role: MessageRole::Assistant,
                content: vec![
                    MessageContent::Error("Failed to compile: missing semicolon on line 15".to_string()),
                ],
                timestamp: Utc::now(),
                is_streaming: false,
                metadata: None,
            },
            Message {
                id: uuid::Uuid::new_v4(),
                role: MessageRole::Tool,
                content: vec![
                    MessageContent::Tool {
                        name: "rust_analyzer".to_string(),
                        input: "check_syntax".to_string(),
                        output: "Syntax check completed. Found 1 error.".to_string(),
                    },
                ],
                timestamp: Utc::now(),
                is_streaming: false,
                metadata: None,
            },
        ];

        Self {
            scroll_handle: ScrollHandle::new(),
            messages,
            current_variant: MessageBubbleVariant::Standard,
        }
    }

    fn cycle_variant(&mut self, cx: &mut Context<Self>) {
        self.current_variant = match self.current_variant {
            MessageBubbleVariant::Standard => MessageBubbleVariant::Minimal,
            MessageBubbleVariant::Minimal => MessageBubbleVariant::Card,
            MessageBubbleVariant::Card => MessageBubbleVariant::Compact,
            MessageBubbleVariant::Compact => MessageBubbleVariant::Standard,
        };
        cx.notify();
    }
}

impl Render for MessageBubbleDemo {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
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
                            .size_full()
                            .bg(rgb(0xF5F5F5))
                            .child(
                                // Header with variant selector
                                div()
                                    .flex()
                                    .items_center()
                                    .justify_between()
                                    .h(px(60.))
                                    .bg(rgb(0xFFFFFF))
                                    .border_b_1()
                                    .border_color(rgb(0xE0E0E0))
                                    .px_6()
                                    .child(
                                        div()
                                            .text_xl()
                                            .font_weight(FontWeight::BOLD)
                                            .text_color(rgb(0x1A1A1A))
                                            .child("AI Message Bubble Demo")
                                    )
                                    .child(
                                        div()
                                            .flex()
                                            .items_center()
                                            .gap_3()
                                            .child(
                                                div()
                                                    .text_sm()
                                                    .text_color(rgb(0x666666))
                                                    .child(format!("Variant: {:?}", self.current_variant))
                                            )
                                            .child(
                                                div()
                                                    .px_4()
                                                    .py_2()
                                                    .bg(rgb(0x696FC7))
                                                    .text_color(rgb(0xFFFFFF))
                                                    .rounded_lg()
                                                    .cursor_pointer()
                                                    .hover(|this| this.bg(rgb(0xA7AAE1)))
                                                    .on_mouse_down(MouseButton::Left, cx.listener(|this, _event: &MouseDownEvent, _window, cx| {
                                                        this.cycle_variant(cx);
                                                    }))
                                                    .child("Change Variant")
                                            )
                                    )
                            )
                            .child(
                                // Messages container
                                div()
                                    .flex()
                                    .flex_col()
                                    .flex_1()
                                    .p_6()
                                    .gap_4()
                                    .children(
                                        self.messages.iter().map(|message| {
                                            MessageBubble::new(message.clone())
                                                .variant(self.current_variant.clone())
                                                .show_timestamp(true)
                                                .show_avatar(true)
                                                .show_actions(true)
                                                .max_width_percent(75.0)
                                                .syntax_highlighting(true)
                                        })
                                    )
                            )
                            .child(
                                // Footer with instructions
                                div()
                                    .h(px(50.))
                                    .bg(rgb(0xFFFFFF))
                                    .border_t_1()
                                    .border_color(rgb(0xE0E0E0))
                                    .flex()
                                    .items_center()
                                    .justify_center()
                                    .child(
                                        div()
                                            .text_sm()
                                            .text_color(rgb(0x666666))
                                            .child("Click 'Change Variant' to see different message bubble styles")
                                    )
                            )
                    )
            )
    }
}

fn main() {
    Application::new()
        .with_assets(fluix::Assets)
        .run(|cx| {
            cx.activate(true);
            
            let bounds = Bounds::centered(None, size(px(900.0), px(700.0)), cx);
            let _ = cx.open_window(
                WindowOptions {
                    window_bounds: Some(WindowBounds::Windowed(bounds)),
                    titlebar: Some(TitlebarOptions {
                        title: Some("AI Message Bubble Demo".into()),
                        appears_transparent: true,
                        ..Default::default()
                    }),
                    window_background: WindowBackgroundAppearance::Opaque,
                    focus: true,
                    show: true,
                    kind: WindowKind::Normal,
                    is_movable: true,
                    display_id: None,
                    window_min_size: Some(size(px(700.0), px(500.0))),
                    app_id: Some("ai_message_bubble_demo".into()),
                    is_minimizable: true,
                    is_resizable: true,
                    window_decorations: Some(WindowDecorations::Server),
                    tabbing_identifier: None,
                },
                |window, cx| cx.new(|cx| MessageBubbleDemo::new(window, cx)),
            );
        });
}
