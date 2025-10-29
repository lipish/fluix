use gpui::*;
use gpui::prelude::FluentBuilder;
use std::sync::Arc;

// ============================================================================
// Events
// ============================================================================

/// Events emitted by TextInput
#[derive(Clone, Debug)]
pub enum TextInputEvent {
    /// The input value has changed
    Change(String),
    /// Enter key was pressed
    Submit(String),
    /// Input gained focus
    Focus,
    /// Input lost focus
    Blur,
}

// ============================================================================
// TextInput Component
// ============================================================================

/// A simple, customizable text input component
pub struct TextInput {
    /// Current input value
    value: String,
    /// Cursor position (byte offset in the string)
    cursor_position: usize,
    /// Placeholder text when empty
    placeholder: String,
    /// Focus handle for keyboard input
    focus_handle: FocusHandle,
    /// Whether the input is disabled
    disabled: bool,
    /// Whether to show as password (mask characters)
    is_password: bool,
    /// Maximum length (None for unlimited)
    max_length: Option<usize>,
    /// Custom validation function
    validator: Option<Arc<dyn Fn(&str) -> bool>>,
}

impl TextInput {
    /// Create a new TextInput
    pub fn new(cx: &mut Context<Self>) -> Self {
        Self {
            value: String::new(),
            cursor_position: 0,
            placeholder: String::new(),
            focus_handle: cx.focus_handle(),
            disabled: false,
            is_password: false,
            max_length: None,
            validator: None,
        }
    }

    /// Set the placeholder text
    pub fn placeholder(mut self, placeholder: impl Into<String>) -> Self {
        self.placeholder = placeholder.into();
        self
    }

    /// Set the initial value
    pub fn value(mut self, value: impl Into<String>) -> Self {
        self.value = value.into();
        self.cursor_position = self.value.len();
        self
    }

    /// Set disabled state
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    /// Set password mode (mask characters)
    pub fn password(mut self, is_password: bool) -> Self {
        self.is_password = is_password;
        self
    }

    /// Set maximum length
    pub fn max_length(mut self, max_length: usize) -> Self {
        self.max_length = Some(max_length);
        self
    }

    /// Set a custom validator function
    pub fn validator<F>(mut self, validator: F) -> Self
    where
        F: Fn(&str) -> bool + 'static,
    {
        self.validator = Some(Arc::new(validator));
        self
    }

    /// Get the current value
    pub fn get_value(&self) -> &str {
        &self.value
    }

    /// Set the value programmatically
    pub fn set_value(&mut self, value: String, cx: &mut Context<Self>) {
        if let Some(ref validator) = self.validator {
            if !validator(&value) {
                return;
            }
        }

        if let Some(max_len) = self.max_length {
            if value.len() > max_len {
                return;
            }
        }

        self.value = value.clone();
        self.cursor_position = self.value.len();
        cx.emit(TextInputEvent::Change(value));
        cx.notify();
    }

    /// Clear the input
    pub fn clear(&mut self, cx: &mut Context<Self>) {
        self.value.clear();
        self.cursor_position = 0;
        cx.emit(TextInputEvent::Change(String::new()));
        cx.notify();
    }

    /// Focus the input
    pub fn focus(&self, window: &mut Window) {
        self.focus_handle.focus(window);
    }

    fn handle_input(&mut self, text: &str, cx: &mut Context<Self>) {
        if self.disabled {
            return;
        }

        // Insert text at cursor position
        let mut new_value = String::new();
        new_value.push_str(&self.value[..self.cursor_position]);
        new_value.push_str(text);
        new_value.push_str(&self.value[self.cursor_position..]);

        // Check max length
        if let Some(max_len) = self.max_length {
            if new_value.len() > max_len {
                return;
            }
        }

        // Check validator
        if let Some(ref validator) = self.validator {
            if !validator(&new_value) {
                return;
            }
        }

        self.value = new_value.clone();
        self.cursor_position += text.len();
        cx.emit(TextInputEvent::Change(new_value));
        cx.notify();
    }

    fn handle_backspace(&mut self, cx: &mut Context<Self>) {
        if self.disabled || self.cursor_position == 0 {
            return;
        }

        // Remove character before cursor
        let mut new_value = String::new();
        new_value.push_str(&self.value[..self.cursor_position - 1]);
        new_value.push_str(&self.value[self.cursor_position..]);

        self.value = new_value.clone();
        self.cursor_position -= 1;
        cx.emit(TextInputEvent::Change(new_value));
        cx.notify();
    }

    fn handle_delete(&mut self, cx: &mut Context<Self>) {
        if self.disabled || self.cursor_position >= self.value.len() {
            return;
        }

        // Remove character at cursor
        let mut new_value = String::new();
        new_value.push_str(&self.value[..self.cursor_position]);
        new_value.push_str(&self.value[self.cursor_position + 1..]);

        self.value = new_value.clone();
        cx.emit(TextInputEvent::Change(new_value));
        cx.notify();
    }

    fn move_cursor_left(&mut self, cx: &mut Context<Self>) {
        if self.cursor_position > 0 {
            self.cursor_position -= 1;
            cx.notify();
        }
    }

    fn move_cursor_right(&mut self, cx: &mut Context<Self>) {
        if self.cursor_position < self.value.len() {
            self.cursor_position += 1;
            cx.notify();
        }
    }

    fn move_cursor_home(&mut self, cx: &mut Context<Self>) {
        self.cursor_position = 0;
        cx.notify();
    }

    fn move_cursor_end(&mut self, cx: &mut Context<Self>) {
        self.cursor_position = self.value.len();
        cx.notify();
    }

    fn handle_submit(&mut self, cx: &mut Context<Self>) {
        if self.disabled {
            return;
        }

        cx.emit(TextInputEvent::Submit(self.value.clone()));
    }

    fn render_display_text(&self) -> String {
        if self.is_password && !self.value.is_empty() {
            "•".repeat(self.value.len())
        } else {
            self.value.clone()
        }
    }
}

impl EventEmitter<TextInputEvent> for TextInput {}

impl Focusable for TextInput {
    fn focus_handle(&self, _cx: &App) -> FocusHandle {
        self.focus_handle.clone()
    }
}

impl Render for TextInput {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let is_focused = self.focus_handle.is_focused(window);
        let display_text = self.render_display_text();
        let show_placeholder = self.value.is_empty();
        let disabled = self.disabled;
        let placeholder = self.placeholder.clone();

        div()
            .id("text-input")
            .track_focus(&self.focus_handle)
            .on_key_down(cx.listener(|this, event: &KeyDownEvent, _, cx| {
                if this.disabled {
                    return;
                }

                // Handle special keys
                match event.keystroke.key.as_str() {
                    "backspace" => {
                        this.handle_backspace(cx);
                    }
                    "delete" => {
                        this.handle_delete(cx);
                    }
                    "left" => {
                        this.move_cursor_left(cx);
                    }
                    "right" => {
                        this.move_cursor_right(cx);
                    }
                    "home" => {
                        this.move_cursor_home(cx);
                    }
                    "end" => {
                        this.move_cursor_end(cx);
                    }
                    "enter" => {
                        this.handle_submit(cx);
                    }
                    _ => {
                        // Handle regular character input
                        if let Some(ch) = &event.keystroke.key_char {
                            this.handle_input(ch, cx);
                        }
                    }
                }
            }))
            .on_mouse_down(MouseButton::Left, cx.listener(|_, _, window, cx| {
                cx.emit(TextInputEvent::Focus);
                cx.focus_self(window);
            }))
            .flex()
            .items_center()
            .w_full()
            .h(px(36.))
            .px_3()
            .bg(if disabled {
                rgb(0xF5F5F5)
            } else {
                rgb(0xFFFFFF)
            })
            .border_1()
            .border_color(if is_focused {
                rgb(0x696FC7)
            } else {
                rgb(0xE0E0E0)
            })
            .rounded(px(6.))
            .when(!disabled, |this| {
                this.cursor(CursorStyle::IBeam)
            })
            .child(
                div()
                    .flex()
                    .items_center()
                    .flex_1()
                    .text_sm()
                    .when(show_placeholder, |this| {
                        this.text_color(rgb(0x999999))
                            .child(placeholder)
                    })
                    .when(!show_placeholder && !is_focused, |this| {
                        // Not focused: show full text
                        this.text_color(if disabled {
                            rgb(0x999999)
                        } else {
                            rgb(0x333333)
                        })
                        .child(display_text.clone())
                    })
                    .when(!show_placeholder && is_focused && !disabled, |this| {
                        // Focused: show text with cursor
                        let cursor_pos = self.cursor_position;
                        let text_before = if cursor_pos > 0 {
                            if self.is_password {
                                "•".repeat(cursor_pos)
                            } else {
                                self.value[..cursor_pos].to_string()
                            }
                        } else {
                            String::new()
                        };

                        let text_after = if cursor_pos < self.value.len() {
                            if self.is_password {
                                "•".repeat(self.value.len() - cursor_pos)
                            } else {
                                self.value[cursor_pos..].to_string()
                            }
                        } else {
                            String::new()
                        };

                        this.text_color(if disabled {
                            rgb(0x999999)
                        } else {
                            rgb(0x333333)
                        })
                        .when(!text_before.is_empty(), |this| {
                            this.child(text_before)
                        })
                        .child(
                            // Cursor
                            div()
                                .w(px(1.))
                                .h(px(18.))
                                .bg(rgb(0x333333))
                        )
                        .when(!text_after.is_empty(), |this| {
                            this.child(text_after)
                        })
                    })
            )
    }
}
