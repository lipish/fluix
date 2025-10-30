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
    /// Cursor position (character index)
    cursor_pos: usize,
    /// Selection start position (character index, None if no selection)
    selection_start: Option<usize>,
    /// Selection end position (character index, None if no selection)
    selection_end: Option<usize>,
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
    /// Custom background color
    bg_color: Option<Rgba>,
    /// Custom border color (None for default)
    custom_border_color: Option<Rgba>,
    /// Custom focus border color (None for default)
    focus_border_color: Option<Rgba>,
    /// Whether to show border
    show_border: bool,
}

impl TextArea {
    /// Create a new TextArea
    pub fn new(cx: &mut Context<Self>) -> Self {
        Self {
            value: String::new(),
            cursor_pos: 0,
            selection_start: None,
            selection_end: None,
            placeholder: String::new(),
            focus_handle: cx.focus_handle(),
            disabled: false,
            max_length: None,
            min_height: 100.0,
            max_height: None,
            bg_color: None,
            custom_border_color: None,
            focus_border_color: None,
            show_border: true,
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

    /// Set custom background color
    pub fn bg_color(mut self, color: Rgba) -> Self {
        self.bg_color = Some(color);
        self
    }

    /// Set custom border color
    pub fn border_color(mut self, color: Rgba) -> Self {
        self.custom_border_color = Some(color);
        self
    }

    /// Set custom focus border color
    pub fn focus_border_color(mut self, color: Rgba) -> Self {
        self.focus_border_color = Some(color);
        self
    }

    /// Remove border
    pub fn no_border(mut self) -> Self {
        self.show_border = false;
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
    
    /// Select all text
    pub fn select_all(&mut self, cx: &mut Context<Self>) {
        if !self.value.is_empty() {
            self.selection_start = Some(0);
            self.selection_end = Some(self.value.chars().count());
            cx.notify();
        }
    }
    
    /// Clear selection
    fn clear_selection(&mut self) {
        self.selection_start = None;
        self.selection_end = None;
    }
    
    /// Check if there is an active selection
    fn has_selection(&self) -> bool {
        self.selection_start.is_some() && self.selection_end.is_some()
    }
    
    /// Delete selected text and return the new cursor position
    fn delete_selection(&mut self) -> usize {
        if let (Some(start), Some(end)) = (self.selection_start, self.selection_end) {
            let (sel_start, sel_end) = if start <= end {
                (start, end)
            } else {
                (end, start)
            };
            
            let before = self.value.chars().take(sel_start).collect::<String>();
            let after = self.value.chars().skip(sel_end).collect::<String>();
            self.value = format!("{}{}", before, after);
            self.clear_selection();
            sel_start
        } else {
            self.cursor_pos
        }
    }

    fn handle_input(&mut self, text: &str, cx: &mut Context<Self>) {
        if self.disabled {
            return;
        }
        
        // If there's a selection, delete it first
        if self.has_selection() {
            self.cursor_pos = self.delete_selection();
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
        if self.disabled {
            return;
        }
        
        // If there's a selection, delete it
        if self.has_selection() {
            self.cursor_pos = self.delete_selection();
            cx.emit(TextAreaEvent::Change(self.value.clone()));
            cx.notify();
            return;
        }
        
        if self.cursor_pos == 0 {
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

    /// Extend or start selection to the cursor position
    fn extend_selection_to(&mut self, pos: usize) {
        // This is called when extending existing selection
        if let (Some(start), Some(end)) = (self.selection_start, self.selection_end) {
            let (sel_start, sel_end) = if start <= end {
                (start, end)
            } else {
                (end, start)
            };
            
            // Extend from the end that's closer to the new position
            if (pos as i32 - sel_start as i32).abs() < (pos as i32 - sel_end as i32).abs() {
                // Extend from start
                self.selection_start = Some(pos);
                self.selection_end = Some(sel_end);
            } else {
                // Extend from end
                self.selection_start = Some(sel_start);
                self.selection_end = Some(pos);
            }
        }
    }
    
    fn move_cursor_left(&mut self, extend_selection: bool, cx: &mut Context<Self>) {
        if self.cursor_pos > 0 {
            let old_pos = self.cursor_pos;
            self.cursor_pos -= 1;
            if extend_selection {
                if !self.has_selection() {
                    // Start new selection: old position is anchor, new position is end
                    self.selection_start = Some(old_pos);
                    self.selection_end = Some(self.cursor_pos);
                } else {
                    self.extend_selection_to(self.cursor_pos);
                }
            } else {
                self.clear_selection();
            }
            cx.notify();
        }
    }
    
    fn move_cursor_right(&mut self, extend_selection: bool, cx: &mut Context<Self>) {
        let char_count = self.value.chars().count();
        if self.cursor_pos < char_count {
            let old_pos = self.cursor_pos;
            self.cursor_pos += 1;
            if extend_selection {
                if !self.has_selection() {
                    // Start new selection: old position is anchor, new position is end
                    self.selection_start = Some(old_pos);
                    self.selection_end = Some(self.cursor_pos);
                } else {
                    self.extend_selection_to(self.cursor_pos);
                }
            } else {
                self.clear_selection();
            }
            cx.notify();
        }
    }
    
    fn move_cursor_home(&mut self, extend_selection: bool, cx: &mut Context<Self>) {
        // For now, move to start of text (could be enhanced to move to start of line)
        let old_pos = self.cursor_pos;
        self.cursor_pos = 0;
        if extend_selection {
            if !self.has_selection() {
                self.selection_start = Some(old_pos);
                self.selection_end = Some(0);
            } else {
                self.extend_selection_to(0);
            }
        } else {
            self.clear_selection();
        }
        cx.notify();
    }
    
    fn move_cursor_end(&mut self, extend_selection: bool, cx: &mut Context<Self>) {
        // For now, move to end of text (could be enhanced to move to end of line)
        let old_pos = self.cursor_pos;
        self.cursor_pos = self.value.chars().count();
        if extend_selection {
            if !self.has_selection() {
                self.selection_start = Some(old_pos);
                self.selection_end = Some(self.cursor_pos);
            } else {
                self.extend_selection_to(self.cursor_pos);
            }
        } else {
            self.clear_selection();
        }
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
        let selection_start = self.selection_start;
        let selection_end = self.selection_end;
        
        // Determine colors based on customization or defaults
        let bg_color = if disabled {
            self.bg_color.unwrap_or(rgb(0xF5F5F5))
        } else {
            self.bg_color.unwrap_or(rgb(0xFFFFFF))
        };
        
        let border_color = if is_focused {
            self.focus_border_color
                .or(self.custom_border_color)
                .unwrap_or(rgb(0x696FC7))
        } else {
            self.custom_border_color.unwrap_or(rgb(0xE0E0E0))
        };

        div()
            .id("text-area")
            .track_focus(&self.focus_handle)
            .on_key_down(cx.listener(|this, event: &KeyDownEvent, _, cx| {
                if this.disabled {
                    return;
                }

                // Handle keyboard shortcuts
                let modifiers = &event.keystroke.modifiers;
                // On macOS use platform (Cmd), on others use control (Ctrl)
                let is_cmd_or_ctrl = if cfg!(target_os = "macos") {
                    modifiers.platform
                } else {
                    modifiers.control
                };
                
                // Check for Cmd/Ctrl + A (Select All)
                if is_cmd_or_ctrl && event.keystroke.key.as_str().eq_ignore_ascii_case("a") {
                    this.select_all(cx);
                    return;
                }

                // Check for Shift key to extend selection
                let shift_pressed = modifiers.shift;

                // Handle special keys
                match event.keystroke.key.as_str() {
                    "backspace" => {
                        this.handle_backspace(cx);
                    }
                    "left" => {
                        this.move_cursor_left(shift_pressed, cx);
                    }
                    "right" => {
                        this.move_cursor_right(shift_pressed, cx);
                    }
                    "home" => {
                        this.move_cursor_home(shift_pressed, cx);
                    }
                    "end" => {
                        this.move_cursor_end(shift_pressed, cx);
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
            .on_mouse_down(MouseButton::Left, cx.listener(|this, event: &MouseDownEvent, window, cx| {
                cx.emit(TextAreaEvent::Focus);
                cx.focus_self(window);
                
                // Check for double-click to select all
                if event.click_count == 2 && !this.disabled {
                    this.select_all(cx);
                } else if !this.disabled {
                    // Single click: clear selection and position cursor at click
                    this.clear_selection();
                    // For now, just move cursor to end on click
                    // (Precise click positioning would require pixel-to-char calculation)
                    this.cursor_pos = this.value.chars().count();
                }
                
                cx.notify();
            }))
            .flex()
            .flex_col()
            .w_full()
            .min_h(px(height))
            .p_3()
            .bg(bg_color)
            .when(self.show_border, |this| {
                this.border_1().border_color(border_color)
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
                        // Calculate selection range if exists
                        let has_selection = selection_start.is_some() && selection_end.is_some();
                        let (sel_start, sel_end) = if let (Some(start), Some(end)) = (selection_start, selection_end) {
                            if start <= end {
                                (start, end)
                            } else {
                                (end, start)
                            }
                        } else {
                            (0, 0)
                        };
                        
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
                                        let mut line_chars_before = 0;
                                        
                                        // Calculate character count before this line
                                        for (idx, l) in lines.iter().enumerate() {
                                            if idx < line_idx {
                                                line_chars_before += l.chars().count() + 1; // +1 for newline
                                            }
                                        }
                                        
                                        let line_start = line_chars_before;
                                        let line_end = line_start + line_str.chars().count();
                                        
                                        // Check if this line has selection
                                        let line_has_selection = has_selection && 
                                            !(sel_end <= line_start || sel_start >= line_end);
                                        
                                        if line_has_selection {
                                            // This line has selection
                                            let sel_start_in_line = if sel_start > line_start {
                                                sel_start - line_start
                                            } else {
                                                0
                                            };
                                            let sel_end_in_line = if sel_end < line_end {
                                                sel_end - line_start
                                            } else {
                                                line_str.chars().count()
                                            };
                                            
                                            let before_sel = line_str.chars().take(sel_start_in_line).collect::<String>();
                                            let selected = line_str.chars().skip(sel_start_in_line).take(sel_end_in_line - sel_start_in_line).collect::<String>();
                                            let after_sel = line_str.chars().skip(sel_end_in_line).collect::<String>();
                                            
                                            div()
                                                .flex()
                                                .flex_row()
                                                .items_start()
                                                .when(!before_sel.is_empty(), |el| el.child(before_sel))
                                                .when(!selected.is_empty(), |el| {
                                                    el.child(
                                                        div()
                                                            .bg(rgb(0x4A90E2))
                                                            .text_color(rgb(0xFFFFFF))
                                                            .child(selected)
                                                    )
                                                })
                                                .when(!after_sel.is_empty(), |el| el.child(after_sel))
                                                // Don't show cursor when there's a selection
                                        } else if line_idx == cursor_line_idx {
                                            // This line contains the cursor but no selection
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
                                            // Regular line without cursor or selection
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
