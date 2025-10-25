use gpui::*;
use gpui::prelude::FluentBuilder;

// ============================================================================
// Events
// ============================================================================

/// Events emitted by TextArea
#[derive(Clone, Debug)]
pub enum TextAreaEvent {
    /// The textarea value has changed
    Change(String),
    /// Submit event (when Enter is pressed without Shift)
    Submit(String),
    /// Focus event
    Focus,
    /// Blur event
    Blur,
}

// ============================================================================
// TextArea Component
// ============================================================================

/// A multi-line text area component
pub struct TextArea {
    /// Current text value
    value: String,
    /// Cursor position (byte index)
    cursor_pos: usize,
    /// Placeholder text when empty
    placeholder: String,
    /// Focus handle for keyboard input
    focus_handle: FocusHandle,
    /// Whether the textarea is disabled
    disabled: bool,
    /// Maximum length (None for unlimited)
    max_length: Option<usize>,
    /// Minimum height in pixels
    min_height: f32,
    /// Maximum height in pixels (None for unlimited)
    max_height: Option<f32>,
}

impl TextArea {
    /// Create a new TextArea
    pub fn new(cx: &mut Context<Self>) -> Self {
        Self {
            value: String::new(),
            cursor_pos: 0,
            placeholder: String::new(),
            focus_handle: cx.focus_handle(),
            disabled: false,
            max_length: None,
            min_height: 100.0,
            max_height: None,
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
        self
    }

    /// Set disabled state
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    /// Set maximum length
    pub fn max_length(mut self, max_length: usize) -> Self {
        self.max_length = Some(max_length);
        self
    }

    /// Set minimum height
    pub fn min_height(mut self, height: f32) -> Self {
        self.min_height = height;
        self
    }

    /// Set maximum height
    pub fn max_height(mut self, height: f32) -> Self {
        self.max_height = Some(height);
        self
    }

    /// Get the current value
    pub fn get_value(&self) -> &str {
        &self.value
    }

    /// Set the value programmatically
    pub fn set_value(&mut self, value: String, cx: &mut Context<Self>) {
        if let Some(max_len) = self.max_length {
            if value.len() > max_len {
                return;
            }
        }

        self.cursor_pos = value.len();
        self.value = value.clone();
        cx.emit(TextAreaEvent::Change(value));
        cx.notify();
    }

    /// Clear the textarea
    pub fn clear(&mut self, cx: &mut Context<Self>) {
        self.value.clear();
        self.cursor_pos = 0;
        cx.emit(TextAreaEvent::Change(String::new()));
        cx.notify();
    }

    /// Focus the textarea
    pub fn focus(&self, window: &mut Window) {
        self.focus_handle.focus(window);
    }

    fn handle_input(&mut self, text: &str, cx: &mut Context<Self>) {
        if self.disabled {
            return;
        }

        // Insert text at cursor position
        let before = self.value.chars().take(self.cursor_pos).collect::<String>();
        let after = self.value.chars().skip(self.cursor_pos).collect::<String>();
        let new_value = format!("{}{}{}", before, text, after);

        // Check max length
        if let Some(max_len) = self.max_length {
            if new_value.chars().count() > max_len {
                return;
            }
        }

        self.value = new_value.clone();
        self.cursor_pos += text.chars().count();
        cx.emit(TextAreaEvent::Change(new_value));
        cx.notify();
    }

    fn handle_backspace(&mut self, cx: &mut Context<Self>) {
        if self.disabled || self.cursor_pos == 0 {
            return;
        }

        // Remove character before cursor
        let before = self.value.chars().take(self.cursor_pos - 1).collect::<String>();
        let after = self.value.chars().skip(self.cursor_pos).collect::<String>();
        self.value = format!("{}{}", before, after);
        self.cursor_pos -= 1;
        
        cx.emit(TextAreaEvent::Change(self.value.clone()));
        cx.notify();
    }

    fn handle_enter(&mut self, shift_pressed: bool, cx: &mut Context<Self>) {
        if self.disabled {
            return;
        }

        if shift_pressed {
            // Shift+Enter: insert newline
            self.handle_input("\n", cx);
        } else {
            // Enter: submit
            cx.emit(TextAreaEvent::Submit(self.value.clone()));
        }
    }

    fn count_lines(&self) -> usize {
        if self.value.is_empty() {
            1
        } else {
            self.value.lines().count().max(1)
        }
    }

    fn calculate_height(&self) -> f32 {
        let line_count = self.count_lines();
        let line_height = 20.0; // Approximate line height
        let padding = 12.0; // Vertical padding
        let calculated = (line_count as f32 * line_height) + padding;

        // Apply min and max constraints
        let height = calculated.max(self.min_height);
        if let Some(max_h) = self.max_height {
            height.min(max_h)
        } else {
            height
        }
    }
}

impl EventEmitter<TextAreaEvent> for TextArea {}

impl Focusable for TextArea {
    fn focus_handle(&self, _cx: &App) -> FocusHandle {
        self.focus_handle.clone()
    }
}

impl Render for TextArea {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let is_focused = self.focus_handle.is_focused(window);
        let show_placeholder = self.value.is_empty() && !is_focused;
        let disabled = self.disabled;
        let placeholder = self.placeholder.clone();
        let value = self.value.clone();
        let cursor_pos = self.cursor_pos;
        let height = self.calculate_height();

        div()
            .id("text-area")
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
                    "enter" => {
                        let shift_pressed = event.keystroke.modifiers.shift;
                        this.handle_enter(shift_pressed, cx);
                    }
                    _ => {
                        // Handle regular character input
                        if let Some(ch) = &event.keystroke.key_char {
                            this.handle_input(ch, cx);
                        }
                    }
                }
            }))
            .on_mouse_down(MouseButton::Left, cx.listener(|_this, _, window, cx| {
                cx.emit(TextAreaEvent::Focus);
                cx.focus_self(window);
                // Ensure cursor is visible on focus
                cx.notify();
            }))
            .flex()
            .flex_col()
            .w_full()
            .min_h(px(height))
            .p_3()
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
            .children([
                // Render text with cursor
                div()
                    .flex()
                    .flex_row()
                    .items_start()
                    .w_full()
                    .text_sm()
                    .when(show_placeholder, |this| {
                        this.text_color(rgb(0x999999))
                            .child(placeholder)
                    })
                    .when(!show_placeholder && is_focused && value.is_empty(), |this| {
                        // Empty and focused: show just cursor
                        this.child(
                            div()
                                .w(px(2.))
                                .h(px(20.))
                                .bg(rgb(0x333333))
                        )
                    })
                    .when(!show_placeholder && !value.is_empty(), |this| {
                        // Split text into lines and find cursor position
                        let lines: Vec<&str> = value.split('\n').collect();
                        let mut chars_counted = 0;
                        let mut cursor_line_idx = 0;
                        let mut cursor_col = cursor_pos;
                        
                        // Find which line the cursor is on
                        for (idx, line) in lines.iter().enumerate() {
                            let line_len = line.chars().count();
                            // Check if cursor is within this line (including at the end)
                            if cursor_pos <= chars_counted + line_len {
                                cursor_line_idx = idx;
                                cursor_col = cursor_pos - chars_counted;
                                break;
                            }
                            // +1 for the newline character
                            chars_counted += line_len + 1;
                            
                            // If this is the last line and we haven't found the cursor yet
                            if idx == lines.len() - 1 {
                                cursor_line_idx = idx;
                                cursor_col = line_len;
                            }
                        }
                        
                        this.text_color(if disabled {
                            rgb(0x999999)
                        } else {
                            rgb(0x333333)
                        })
                        .child(
                            div()
                                .flex()
                                .flex_col()
                                .items_start()
                                .w_full()
                                .children(
                                    lines.iter().enumerate().map(|(line_idx, line)| {
                                        let line_str = line.to_string();
                                        
                                        if line_idx == cursor_line_idx {
                                            // This line contains the cursor
                                            let before = line_str.chars().take(cursor_col).collect::<String>();
                                            let after = line_str.chars().skip(cursor_col).collect::<String>();
                                            
                                            div()
                                                .flex()
                                                .flex_row()
                                                .items_start()
                                                .child(before)
                                                .when(is_focused && !disabled, |el| {
                                                    el.child(
                                                        div()
                                                            .w(px(2.))
                                                            .h(px(20.))
                                                            .bg(rgb(0x333333))
                                                            .flex_shrink_0()
                                                    )
                                                })
                                                .child(after)
                                        } else {
                                            // Regular line without cursor
                                            div()
                                                .flex()
                                                .flex_row()
                                                .child(line_str)
                                        }
                                    })
                                )
                        )
                    })
            ])
    }
}
