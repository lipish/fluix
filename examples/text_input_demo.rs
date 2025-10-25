use gpui::*;
use rui::{TextInput, TextInputEvent, TextArea, TextAreaEvent};

// ============================================================================
// Demo App
// ============================================================================

struct TextInputDemo {
    // Different types of inputs
    basic_input: Entity<TextInput>,
    password_input: Entity<TextInput>,
    limited_input: Entity<TextInput>,
    validated_input: Entity<TextInput>,
    disabled_input: Entity<TextInput>,
    
    // TextArea
    text_area: Entity<TextArea>,

    // Display results
    basic_value: String,
    password_value: String,
    limited_value: String,
    validated_value: String,
    textarea_value: String,
}

impl TextInputDemo {
    fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        // Basic text input
        let basic_input = cx.new(|cx| {
            TextInput::new(cx)
                .placeholder("Enter your name...")
        });

        // Password input
        let password_input = cx.new(|cx| {
            TextInput::new(cx)
                .placeholder("Enter password...")
                .password(true)
        });

        // Limited length input
        let limited_input = cx.new(|cx| {
            TextInput::new(cx)
                .placeholder("Max 10 characters")
                .max_length(10)
        });

        // Validated input (only numbers)
        let validated_input = cx.new(|cx| {
            TextInput::new(cx)
                .placeholder("Numbers only...")
                .validator(|text| text.chars().all(|c| c.is_numeric()))
        });

        // Disabled input
        let disabled_input = cx.new(|cx| {
            TextInput::new(cx)
                .value("This is disabled")
                .disabled(true)
        });
        
        // Multi-line text area
        let text_area = cx.new(|cx| {
            TextArea::new(cx)
                .placeholder("Type your message...\nPress Shift+Enter for new line\nPress Enter to submit")
                .min_height(80.0)
                .max_height(200.0)
        });

        // Subscribe to events
        cx.subscribe_in(&basic_input, window, Self::on_basic_change).detach();
        cx.subscribe_in(&password_input, window, Self::on_password_change).detach();
        cx.subscribe_in(&limited_input, window, Self::on_limited_change).detach();
        cx.subscribe_in(&validated_input, window, Self::on_validated_change).detach();
        cx.subscribe_in(&text_area, window, Self::on_textarea_change).detach();

        Self {
            basic_input,
            password_input,
            limited_input,
            validated_input,
            disabled_input,
            text_area,
            basic_value: String::new(),
            password_value: String::new(),
            limited_value: String::new(),
            validated_value: String::new(),
            textarea_value: String::new(),
        }
    }

    fn on_basic_change(
        &mut self,
        _input: &Entity<TextInput>,
        event: &TextInputEvent,
        _: &mut Window,
        cx: &mut Context<Self>,
    ) {
        match event {
            TextInputEvent::Change(value) => {
                self.basic_value = value.clone();
                cx.notify();
            }
            TextInputEvent::Submit(value) => {
                println!("Basic input submitted: {}", value);
            }
            _ => {}
        }
    }

    fn on_password_change(
        &mut self,
        _input: &Entity<TextInput>,
        event: &TextInputEvent,
        _: &mut Window,
        cx: &mut Context<Self>,
    ) {
        match event {
            TextInputEvent::Change(value) => {
                self.password_value = value.clone();
                cx.notify();
            }
            TextInputEvent::Submit(value) => {
                println!("Password submitted: {}", value);
            }
            _ => {}
        }
    }

    fn on_limited_change(
        &mut self,
        _input: &Entity<TextInput>,
        event: &TextInputEvent,
        _: &mut Window,
        cx: &mut Context<Self>,
    ) {
        match event {
            TextInputEvent::Change(value) => {
                self.limited_value = value.clone();
                cx.notify();
            }
            _ => {}
        }
    }

    fn on_validated_change(
        &mut self,
        _input: &Entity<TextInput>,
        event: &TextInputEvent,
        _: &mut Window,
        cx: &mut Context<Self>,
    ) {
        match event {
            TextInputEvent::Change(value) => {
                self.validated_value = value.clone();
                cx.notify();
            }
            _ => {}
        }
    }
    
    fn on_textarea_change(
        &mut self,
        _textarea: &Entity<TextArea>,
        event: &TextAreaEvent,
        _: &mut Window,
        cx: &mut Context<Self>,
    ) {
        match event {
            TextAreaEvent::Change(value) => {
                self.textarea_value = value.clone();
                cx.notify();
            }
            TextAreaEvent::Submit(value) => {
                println!("📤 Submitted: {}", value);
                // Clear after submit
                self.textarea_value.clear();
                cx.notify();
            }
            _ => {}
        }
    }

    fn render_section(&self, title: &str, input: Entity<TextInput>, value: &str) -> impl IntoElement {
        let title = title.to_string();
        let value = value.to_string();
        div()
            .flex()
            .flex_col()
            .gap_2()
            .w_full()
            .child(
                div()
                    .text_sm()
                    .font_weight(FontWeight::SEMIBOLD)
                    .text_color(rgb(0x333333))
                    .child(title)
            )
            .child(
                div()
                    .w_full()
                    .child(input)
            )
            .child(
                div()
                    .text_xs()
                    .text_color(rgb(0x666666))
                    .child(format!("Value: '{}'", value))
            )
    }
}

impl Render for TextInputDemo {
    fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
        div()
            .flex()
            .flex_col()
            .size_full()
            .bg(rgb(0xF5F5F5))
            .p_8()
            .gap_8()
            .child(
                div()
                    .flex()
                    .flex_col()
                    .w_full()
                    .max_w(px(600.))
                    .mx_auto()
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
                                    .text_color(rgb(0x333333))
                                    .child("RUI TextArea Demo")
                            )
                            .child(
                                div()
                                    .text_sm()
                                    .text_color(rgb(0x666666))
                                    .child("Multi-line text editor component")
                            )
                    )
                    .child(
                        // TextArea only
                        div()
                            .flex()
                            .flex_col()
                            .gap_6()
                            .bg(rgb(0xFFFFFF))
                            .border_1()
                            .border_color(rgb(0xE0E0E0))
                            .rounded(px(8.))
                            .p_6()
                            .child(
                                div()
                                    .flex()
                                    .flex_col()
                                    .gap_2()
                                    .w_full()
                                    .child(
                                        div()
                                            .text_sm()
                                            .font_weight(FontWeight::SEMIBOLD)
                                            .text_color(rgb(0x333333))
                                            .child("Multi-line Text Input")
                                    )
                                    .child(
                                        div()
                                            .w_full()
                                            .child(self.text_area.clone())
                                    )
                                    .child(
                                        div()
                                            .text_xs()
                                            .text_color(rgb(0x666666))
                                            .child(format!("Lines: {} | Length: {}", 
                                                self.textarea_value.lines().count().max(1),
                                                self.textarea_value.len()
                                            ))
                                    )
                            )
                    )
                    .child(
                        // Instructions
                        div()
                            .flex()
                            .flex_col()
                            .gap_2()
                            .bg(rgb(0xFFF9E6))
                            .border_1()
                            .border_color(rgb(0xFFE066))
                            .rounded(px(8.))
                            .p_4()
                            .child(
                                div()
                                    .text_sm()
                                    .font_weight(FontWeight::SEMIBOLD)
                                    .text_color(rgb(0x664D00))
                                    .child("💡 Instructions")
                            )
                            .child(
                                div()
                                    .text_xs()
                                    .text_color(rgb(0x8C6600))
                                    .child("• Click on the text area to focus and start typing")
                            )
                            .child(
                                div()
                                    .text_xs()
                                    .text_color(rgb(0x8C6600))
                                    .child("• Press Shift+Enter to create new lines")
                            )
                            .child(
                                div()
                                    .text_xs()
                                    .text_color(rgb(0x8C6600))
                                    .child("• Press Enter to submit (check console)")
                            )
                    )
            )
    }
}

// ============================================================================
// Main
// ============================================================================

fn main() {
    env_logger::init();

    let app = Application::new();

    app.run(move |cx| {
        let window_options = WindowOptions {
            window_bounds: Some(WindowBounds::Windowed(Bounds {
                origin: point(px(100.), px(100.)),
                size: size(px(700.), px(600.)),
            })),
            titlebar: Some(TitlebarOptions {
                title: Some("RUI TextArea Demo".into()),
                appears_transparent: false,
                ..Default::default()
            }),
            ..Default::default()
        };

        cx.spawn(async move |cx| {
            cx.open_window(window_options, |window, cx| {
                cx.new(|cx| TextInputDemo::new(window, cx))
            })?;

            Ok::<_, anyhow::Error>(())
        })
        .detach();
    });
}
