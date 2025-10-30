use gpui::*;
use gpui::prelude::FluentBuilder;
use std::sync::{Arc, Mutex};

// Element state for storing layout information per line
#[derive(Clone)]
struct TextAreaLineLayout {
    bounds: Bounds<Pixels>,
    shaped_line: ShapedLine,
}

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
    /// Blink epoch - increments when cursor should reset to visible
    blink_epoch: usize,
    /// Whether cursor is currently visible (for blinking)
    cursor_visible: bool,
    /// Blink task handle
    _blink_task: Option<Task<()>>,
    /// Whether mouse is currently dragging for selection
    is_dragging: bool,
    /// Last layout info for mouse position calculation (per line)
    last_layout: Vec<TextAreaLineLayout>,
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
            blink_epoch: 0,
            cursor_visible: true,
            _blink_task: None,
            is_dragging: false,
            last_layout: Vec::new(),
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
        self.pause_blinking(cx);
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
            self.pause_blinking(cx);
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
        self.pause_blinking(cx);
        
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
            self.pause_blinking(cx);
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
            self.pause_blinking(cx);
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
        self.pause_blinking(cx);
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
        self.pause_blinking(cx);
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
    
    fn handle_delete(&mut self, cx: &mut Context<Self>) {
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
        
        let char_count = self.value.chars().count();
        if self.cursor_pos >= char_count {
            return;
        }

        // Remove character at cursor
        let before = self.value.chars().take(self.cursor_pos).collect::<String>();
        let after = self.value.chars().skip(self.cursor_pos + 1).collect::<String>();
        self.value = format!("{}{}", before, after);
        
        cx.emit(TextAreaEvent::Change(self.value.clone()));
        cx.notify();
    }
    
    /// Increment blink epoch (used to cancel old blink tasks)
    fn next_blink_epoch(&mut self) -> usize {
        self.blink_epoch += 1;
        self.blink_epoch
    }

    /// Start cursor blinking animation
    fn start_blinking(&mut self, epoch: usize, cx: &mut Context<Self>) {
        // Only blink if this is still the current epoch
        if epoch == self.blink_epoch {
            // Toggle visibility
            self.cursor_visible = !self.cursor_visible;
            cx.notify();

            // Schedule next blink
            let next_epoch = self.next_blink_epoch();
            let task = cx.spawn(async move |this, cx| {
                cx.background_spawn(async move {
                    std::thread::sleep(std::time::Duration::from_millis(530));
                }).await;
                _ = this.update(cx, |this, cx| {
                    this.start_blinking(next_epoch, cx);
                });
            });
            self._blink_task = Some(task);
        }
    }

    /// Pause blinking and show cursor (called on user input)
    fn pause_blinking(&mut self, cx: &mut Context<Self>) {
        // Show cursor immediately
        self.cursor_visible = true;
        cx.notify();

        // Cancel current blink and restart after a delay
        let epoch = self.next_blink_epoch();
        let task = cx.spawn(async move |this, cx| {
            cx.background_spawn(async move {
                std::thread::sleep(std::time::Duration::from_millis(530));
            }).await;
            _ = this.update(cx, |this, cx| {
                this.start_blinking(epoch, cx);
            });
        });
        self._blink_task = Some(task);
    }
    
    /// Calculate character index from mouse position (for multi-line text)
    fn index_for_mouse_position(&self, position: Point<Pixels>) -> usize {
        if self.value.is_empty() {
            return 0;
        }

        if self.last_layout.is_empty() {
            return self.value.chars().count();
        }

        let lines: Vec<&str> = self.value.split('\n').collect();
        
        // Find which line the click is on
        let mut target_line_idx = 0;
        
        for (idx, layout) in self.last_layout.iter().enumerate() {
            if position.y < layout.bounds.bottom() {
                target_line_idx = idx;
                break;
            }
        }
        
        // If click is below all lines, return end of text
        if target_line_idx >= lines.len() {
            return self.value.chars().count();
        }
        
        // Get the layout for the target line
        let Some(line_layout) = self.last_layout.get(target_line_idx) else {
            return self.value.chars().count();
        };
        
        // Calculate relative x position within the line
        let relative_x = position.x - line_layout.bounds.left();
        
        // Use ShapedLine to find closest character index
        let byte_index = line_layout.shaped_line.closest_index_for_x(relative_x);
        
        // Calculate character index before this line
        let mut chars_before_line = 0;
        for (idx, line) in lines.iter().enumerate() {
            if idx < target_line_idx {
                chars_before_line += line.chars().count() + 1; // +1 for newline
            }
        }
        
        // Convert byte index to character index and add offset
        let line = lines[target_line_idx];
        let chars_in_line = line.chars().count();
        let char_index_in_line = byte_index.min(chars_in_line);
        
        chars_before_line + char_index_in_line
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
        
        // Start blinking when focused
        if is_focused && self._blink_task.is_none() {
            let epoch = self.next_blink_epoch();
            self.start_blinking(epoch, cx);
        }
        
        let show_placeholder = self.value.is_empty() && !is_focused;
        let disabled = self.disabled;
        let placeholder = self.placeholder.clone();
        let value = self.value.clone();
        let cursor_pos = self.cursor_pos;
        let height = self.calculate_height();
        let selection_start = self.selection_start;
        let selection_end = self.selection_end;
        
        // Create a shared container for layout info that will be filled during paint
        let layout_container: Arc<Mutex<Vec<TextAreaLineLayout>>> = Arc::new(Mutex::new(Vec::new()));
        
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
                    "delete" => {
                        this.handle_delete(cx);
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
            .on_mouse_down(MouseButton::Left, {
                let layout_container = layout_container.clone();
                cx.listener(move |this, event: &MouseDownEvent, window, cx| {
                    cx.emit(TextAreaEvent::Focus);
                    cx.focus_self(window);
                    
                    // Update last_layout from the container
                    if let Ok(layout) = layout_container.lock() {
                        this.last_layout = layout.clone();
                    }
                    
                    // Check for double-click to select all
                    if event.click_count == 2 && !this.disabled {
                        this.select_all(cx);
                        this.pause_blinking(cx);
                    } else if !this.disabled {
                        // Start dragging
                        this.is_dragging = true;
                        
                        // Calculate click position
                        let index = this.index_for_mouse_position(event.position);
                        
                        if event.modifiers.shift {
                            // Shift+click extends selection
                            if !this.has_selection() {
                                this.selection_start = Some(this.cursor_pos);
                                this.selection_end = Some(index);
                            } else {
                                // Extend existing selection
                                this.selection_end = Some(index);
                            }
                        } else {
                            // Normal click - clear selection and set cursor
                            this.clear_selection();
                        }
                        
                        this.cursor_pos = index;
                        this.pause_blinking(cx);
                    }
                    
                    cx.notify();
                })
            })
            .on_mouse_up(MouseButton::Left, cx.listener(|this, _event: &MouseUpEvent, _window, cx| {
                this.is_dragging = false;
                cx.notify();
            }))
            .on_mouse_move(cx.listener(|this, event: &MouseMoveEvent, _window, cx| {
                if this.is_dragging {
                    // Update selection while dragging
                    let index = this.index_for_mouse_position(event.position);
                    
                    if !this.has_selection() {
                        // Start new selection from cursor position
                        this.selection_start = Some(this.cursor_pos);
                        this.selection_end = Some(index);
                    } else {
                        // Update selection end
                        this.selection_end = Some(index);
                    }
                    
                    this.cursor_pos = index;
                    cx.notify();
                }
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
                        // Empty and focused: show cursor if visible
                        let cursor_visible = self.cursor_visible;
                        if cursor_visible {
                            this.child(
                                div()
                                    .w(px(2.))
                                    .h(px(20.))
                                    .bg(rgb(0x333333))
                            )
                        } else {
                            this // Empty div when cursor hidden
                        }
                    })
                    .when(!show_placeholder && !value.is_empty(), |this| {
                        // Use TextRun API for rendering - prevents width jitter
                        let has_selection = selection_start.is_some() && selection_end.is_some();
                        let font = gpui::Font {
                            family: ".SystemUIFont".into(),
                            features: Default::default(),
                            weight: Default::default(),
                            style: Default::default(),
                            fallbacks: None,
                        };
                        let font_size = px(14.);
                        let line_height = px(20.);
                        let cursor_visible = self.cursor_visible;
                        
                        // Split text into lines
                        let lines: Vec<&str> = value.split('\n').collect();
                        
                        // Calculate character offsets for each line
                        let mut line_offsets = Vec::new();
                        let mut chars_counted = 0;
                        for line in &lines {
                            let line_start = chars_counted;
                            line_offsets.push(line_start);
                            chars_counted += line.chars().count() + 1; // +1 for newline
                        }
                        
                        // Find cursor line
                        let mut cursor_line_idx = 0;
                        let mut cursor_col = cursor_pos;
                        for (idx, offset) in line_offsets.iter().enumerate() {
                            let line_len = lines[idx].chars().count();
                            if cursor_pos <= *offset + line_len {
                                cursor_line_idx = idx;
                                cursor_col = cursor_pos - *offset;
                                break;
                            }
                        }
                        
                        // Clone layout container for each line
                        let layout_container_clone = layout_container.clone();
                        
                        this.child(
                            div()
                                .flex()
                                .flex_col()
                                .items_start()
                                .w_full()
                                .children(
                                    lines.iter().enumerate().map({
                                        let font_clone = font.clone();
                                        let font_size_clone = font_size;
                                        let line_height_clone = line_height;
                                        let cursor_visible_clone = cursor_visible;
                                        let cursor_line_idx_clone = cursor_line_idx;
                                        let cursor_col_clone = cursor_col;
                                        let layout_container_for_line = layout_container_clone.clone();
                                        let line_offsets_clone = line_offsets.clone();
                                        let has_selection_clone = has_selection;
                                        let selection_start_clone = selection_start;
                                        let selection_end_clone = selection_end;
                                        
                                        move |(line_idx, line)| {
                                            let line_start = line_offsets_clone[line_idx];
                                            let line_str = line.to_string();
                                            let line_end = line_start + line_str.chars().count();
                                            
                                            // Build TextRun for this line
                                            let (display_text, text_runs) = if !has_selection_clone {
                                                // No selection: single text run
                                                (line_str.clone(), vec![TextRun {
                                                    len: line_str.len(),
                                                    font: font_clone.clone(),
                                                    color: rgb(0x333333).into(),
                                                    background_color: None,
                                                    underline: None,
                                                    strikethrough: None,
                                                }])
                                            } else {
                                                // Check if this line has selection
                                                let (sel_start, sel_end) = if let (Some(start), Some(end)) = (selection_start_clone, selection_end_clone) {
                                                    if start <= end {
                                                        (start, end)
                                                    } else {
                                                        (end, start)
                                                    }
                                                } else {
                                                    (0, 0)
                                                };
                                                
                                                let line_has_selection = !(sel_end <= line_start || sel_start >= line_end);
                                                
                                                if !line_has_selection {
                                                    // No selection on this line
                                                    (line_str.clone(), vec![TextRun {
                                                        len: line_str.len(),
                                                        font: font_clone.clone(),
                                                        color: rgb(0x333333).into(),
                                                        background_color: None,
                                                        underline: None,
                                                        strikethrough: None,
                                                    }])
                                                } else {
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
                                                    
                                                    let mut runs = Vec::new();
                                                    
                                                    // Text before selection
                                                    if sel_start_in_line > 0 {
                                                        runs.push(TextRun {
                                                            len: sel_start_in_line,
                                                            font: font_clone.clone(),
                                                            color: rgb(0x333333).into(),
                                                            background_color: None,
                                                            underline: None,
                                                            strikethrough: None,
                                                        });
                                                    }
                                                    
                                                    // Selected text
                                                    if sel_end_in_line > sel_start_in_line {
                                                        runs.push(TextRun {
                                                            len: sel_end_in_line - sel_start_in_line,
                                                            font: font_clone.clone(),
                                                            color: rgb(0xFFFFFF).into(),
                                                            background_color: Some(rgb(0x4A90E2).into()),
                                                            underline: None,
                                                            strikethrough: None,
                                                        });
                                                    }
                                                    
                                                    // Text after selection
                                                    if sel_end_in_line < line_str.chars().count() {
                                                        runs.push(TextRun {
                                                            len: line_str.chars().count() - sel_end_in_line,
                                                            font: font_clone.clone(),
                                                            color: rgb(0x333333).into(),
                                                            background_color: None,
                                                            underline: None,
                                                            strikethrough: None,
                                                        });
                                                    }
                                                    
                                                    (line_str.clone(), runs)
                                                }
                                            };
                                            
                                            let is_cursor_line = line_idx == cursor_line_idx_clone && !has_selection_clone;
                                            
                                            div()
                                                .flex()
                                                .flex_row()
                                                .items_start()
                                                .w_full()
                                                .relative()
                                                .child(
                                                    // Text layer using canvas with TextRun API
                                                    canvas(
                                                        move |bounds, _, _cx| {
                                                            gpui::size(bounds.size.width, line_height_clone)
                                                        },
                                                        {
                                                            let display_text_clone = display_text.clone();
                                                            let text_runs_clone = text_runs.clone();
                                                            let line_idx_clone = line_idx;
                                                            let layout_container_for_paint = layout_container_for_line.clone();
                                                            
                                                            move |bounds, _, window, _cx| {
                                                                if !display_text_clone.is_empty() {
                                                                    // Shape the line using TextRun
                                                                    let shaped_line = window.text_system().shape_line(
                                                                        display_text_clone.clone().into(),
                                                                        font_size_clone,
                                                                        &text_runs_clone,
                                                                        None,
                                                                    );
                                                                    
                                                                    // Save layout info for mouse position calculation
                                                                    if let Ok(mut layouts) = layout_container_for_paint.lock() {
                                                                        // Ensure vector is large enough
                                                                        while layouts.len() <= line_idx_clone {
                                                                            layouts.push(TextAreaLineLayout {
                                                                                bounds: gpui::Bounds::default(),
                                                                                shaped_line: shaped_line.clone(),
                                                                            });
                                                                        }
                                                                        layouts[line_idx_clone] = TextAreaLineLayout {
                                                                            bounds,
                                                                            shaped_line: shaped_line.clone(),
                                                                        };
                                                                    }
                                                                    
                                                                    let origin = bounds.origin;
                                                                    
                                                                    // Paint background first (for selection)
                                                                    shaped_line.paint_background(origin, line_height_clone, window, _cx).ok();
                                                                    
                                                                    // Then paint the text
                                                                    shaped_line.paint(origin, line_height_clone, window, _cx).ok();
                                                                }
                                                            }
                                                        },
                                                    )
                                                    .w_full()
                                                    .h(line_height_clone)
                                                )
                                                .when(is_cursor_line && is_focused && !disabled && cursor_visible_clone, |el| {
                                                    // Cursor layer
                                                    let cursor_col_clone = cursor_col_clone;
                                                    let line_str_clone = line_str.clone();
                                                    let font_clone_for_cursor = font_clone.clone();
                                                    
                                                    el.child(
                                                        canvas(
                                                            move |_bounds, _, _cx| {
                                                                gpui::size(px(0.), px(0.))
                                                            },
                                                            move |bounds, _, window, _cx| {
                                                                // Calculate cursor position
                                                                let cursor_x = if cursor_col_clone == 0 || line_str_clone.is_empty() {
                                                                    px(0.)
                                                                } else {
                                                                    let text_before = line_str_clone.chars().take(cursor_col_clone).collect::<String>();
                                                                    if text_before.is_empty() {
                                                                        px(0.)
                                                                    } else {
                                                                        let temp_runs = vec![TextRun {
                                                                            len: text_before.len(),
                                                                            font: font_clone_for_cursor.clone(),
                                                                            color: rgb(0x333333).into(),
                                                                            background_color: None,
                                                                            underline: None,
                                                                            strikethrough: None,
                                                                        }];
                                                                        let temp_line = window.text_system().shape_line(
                                                                            text_before.into(),
                                                                            font_size_clone,
                                                                            &temp_runs,
                                                                            None,
                                                                        );
                                                                        temp_line.width
                                                                    }
                                                                };
                                                                
                                                                // Draw cursor
                                                                let cursor_bounds = gpui::Bounds {
                                                                    origin: bounds.origin + gpui::point(cursor_x, px(1.)),
                                                                    size: gpui::size(px(2.), px(18.)),
                                                                };
                                                                window.paint_quad(gpui::fill(cursor_bounds, rgb(0x000000)));
                                                            },
                                                        )
                                                        .absolute()
                                                        .top(px(0.))
                                                        .left(px(0.))
                                                        .w(px(0.))
                                                        .h(line_height_clone)
                                                    )
                                                })
                                        }
                                    })
                                )
                        )
                    })
            ])
    }
}
