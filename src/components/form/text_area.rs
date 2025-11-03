use gpui::*;
use gpui::prelude::FluentBuilder;
use std::sync::{Arc, Mutex};

// Element state for storing layout information per line
#[derive(Clone)]
struct TextAreaLineLayout {
    bounds: Bounds<Pixels>,
    shaped_line: ShapedLine,
}

// Custom element wrapper to register input handler
struct TextAreaElement {
    entity: Entity<TextArea>,
}

impl IntoElement for TextAreaElement {
    type Element = Self;

    fn into_element(self) -> Self::Element {
        self
    }
}

impl Element for TextAreaElement {
    type RequestLayoutState = ();
    type PrepaintState = ();

    fn id(&self) -> Option<ElementId> {
        None
    }

    fn source_location(&self) -> Option<&'static core::panic::Location<'static>> {
        None
    }

    fn request_layout(
        &mut self,
        _id: Option<&GlobalElementId>,
        _inspector_id: Option<&InspectorElementId>,
        window: &mut Window,
        cx: &mut App,
    ) -> (LayoutId, Self::RequestLayoutState) {
        let mut style = Style::default();
        style.size.width = px(0.0).into();
        style.size.height = px(0.0).into();
        (window.request_layout(style, [], cx), ())
    }

    fn prepaint(
        &mut self,
        _id: Option<&GlobalElementId>,
        _inspector_id: Option<&InspectorElementId>,
        _bounds: Bounds<Pixels>,
        _request_layout: &mut Self::RequestLayoutState,
        _window: &mut Window,
        _cx: &mut App,
    ) -> Self::PrepaintState {
    }

    fn paint(
        &mut self,
        _id: Option<&GlobalElementId>,
        _inspector_id: Option<&InspectorElementId>,
        bounds: Bounds<Pixels>,
        _request_layout: &mut Self::RequestLayoutState,
        _prepaint: &mut Self::PrepaintState,
        window: &mut Window,
        cx: &mut App,
    ) {
        // Register input handler for IME support
        let focus_handle = self.entity.read(cx).focus_handle.clone();
        window.handle_input(
            &focus_handle,
            ElementInputHandler::new(bounds, self.entity.clone()),
            cx,
        );
    }
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
    /// IME input detected (for Chinese/Japanese/Korean input)
    Ime(String),
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
    /// Marked text range for IME (Input Method Editor) composition
    marked_range: Option<std::ops::Range<usize>>,
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
            marked_range: None,
        }
    }

    /// Check if text contains Chinese/Japanese/Korean characters
    fn contains_cjk(text: &str) -> bool {
        text.chars().any(|c| {
            let cp = c as u32;
            (0x4E00..=0x9FFF).contains(&cp) || // CJK统一汉字
            (0x3400..=0x4DBF).contains(&cp) || // CJK扩展A
            (0x20000..=0x2A6DF).contains(&cp) || // CJK扩展B
            (0x2A700..=0x2B73F).contains(&cp) || // CJK扩展C
            (0x2B740..=0x2B81F).contains(&cp) || // CJK扩展D
            (0x2B820..=0x2CEAF).contains(&cp) || // CJK扩展E
            (0x2CEB0..=0x2EBEF).contains(&cp) || // CJK扩展F
            (0x3000..=0x303F).contains(&cp) || // CJK符号和标点
            (0xFF00..=0xFFEF).contains(&cp)    // 全角ASCII、全角标点
        })
    }

    /// Ensure cursor position is within valid bounds
    fn validate_cursor_position(&mut self) {
        let char_count = self.value.chars().count();
        if self.cursor_pos > char_count {
            self.cursor_pos = char_count;
        }
    }

    /// Safely remove character at the given position
    fn remove_char_at(&mut self, pos: usize) -> bool {
        let chars: Vec<char> = self.value.chars().collect();
        if pos < chars.len() {
            let mut new_chars = Vec::new();
            new_chars.extend_from_slice(&chars[..pos]);
            new_chars.extend_from_slice(&chars[pos + 1..]);
            self.value = new_chars.into_iter().collect();
            true
        } else {
            false
        }
    }

    /// Convert character range to byte range
    fn char_range_to_byte_range(&self, char_range: &std::ops::Range<usize>) -> std::ops::Range<usize> {
        // Handle empty string case
        if self.value.is_empty() {
            return 0..0;
        }

        // Handle cursor position (empty range)
        if char_range.start == char_range.end {
            // Find the byte position for the character index
            for (char_count, (byte_idx, _)) in self.value.char_indices().enumerate() {
                if char_count == char_range.start {
                    return byte_idx..byte_idx;
                }
            }
            // If we reach here, the cursor is at the end
            return self.value.len()..self.value.len();
        }

        let mut byte_start = 0;
        let mut byte_end = self.value.len();
        let mut found_start = false;

        for (char_count, (byte_idx, _)) in self.value.char_indices().enumerate() {
            if char_count == char_range.start && !found_start {
                byte_start = byte_idx;
                found_start = true;
            }
            if char_count == char_range.end {
                byte_end = byte_idx;
                break;
            }
        }

        // Handle case where range.start is at the very beginning
        if char_range.start == 0 {
            byte_start = 0;
        }

        // Handle case where range.end is at the very end
        if char_range.end >= self.value.chars().count() {
            byte_end = self.value.len();
        }

        byte_start..byte_end
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

    /// Get the current selection range
    fn selected_range(&self) -> std::ops::Range<usize> {
        if let (Some(start), Some(end)) = (self.selection_start, self.selection_end) {
            if start <= end {
                start..end
            } else {
                end..start
            }
        } else {
            self.cursor_pos..self.cursor_pos
        }
    }

    /// Convert UTF-16 range to UTF-8 range
    fn range_from_utf16(&self, range_utf16: &std::ops::Range<usize>) -> std::ops::Range<usize> {
        let mut utf16_count = 0;
        let mut start = None;
        let mut end = None;

        for (byte_idx, ch) in self.value.char_indices() {
            if start.is_none() && utf16_count >= range_utf16.start {
                start = Some(byte_idx);
            }
            
            utf16_count += ch.len_utf16();
            
            if end.is_none() && utf16_count >= range_utf16.end {
                end = Some(byte_idx + ch.len_utf8());
                break;
            }
        }

        let start = start.unwrap_or(self.value.len());
        let end = end.unwrap_or(self.value.len());
        
        start..end
    }

    /// Convert character range to UTF-16 range
    fn char_range_to_utf16(&self, char_range: &std::ops::Range<usize>) -> std::ops::Range<usize> {
        let mut utf16_start = 0;
        let mut utf16_end = 0;

        for (char_count, ch) in self.value.chars().enumerate() {
            if char_count == char_range.start {
                utf16_start = utf16_end;
            }
            utf16_end += ch.len_utf16();
            if char_count == char_range.end {
                break;
            }
        }

        // Handle case where range.end is at the very end
        if char_range.end >= self.value.chars().count() {
            utf16_end = self.value.encode_utf16().count();
        }

        utf16_start..utf16_end
    }

    /// Convert byte range to UTF-16 range
    fn byte_range_to_utf16(&self, range: &std::ops::Range<usize>) -> std::ops::Range<usize> {
        let start_idx = range.start.min(self.value.len());
        let end_idx = range.end.min(self.value.len());

        let start = self.value[..start_idx].encode_utf16().count();
        let end = self.value[..end_idx].encode_utf16().count();
        start..end
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
        
        // Check for CJK characters and emit IME event if detected
        if Self::contains_cjk(text) {
            cx.emit(TextAreaEvent::Ime(self.value.clone()));
        }
        
        cx.emit(TextAreaEvent::Change(new_value));
        cx.notify();
    }

    fn handle_backspace(&mut self, cx: &mut Context<Self>) {
        if self.disabled {
            return;
        }

        // If there's marked text (IME composition), clear it
        if self.marked_range.is_some() {
            self.marked_range = None;
            cx.notify();
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

        // Ensure cursor position is valid
        self.validate_cursor_position();

        if self.cursor_pos == 0 {
            return;
        }

        // Safely remove character before cursor
        if self.remove_char_at(self.cursor_pos - 1) {
            self.cursor_pos -= 1;
        }

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

        // If there's marked text (IME composition), clear it
        if self.marked_range.is_some() {
            self.marked_range = None;
            cx.notify();
            return;
        }

        // If there's a selection, delete it
        if self.has_selection() {
            self.cursor_pos = self.delete_selection();
            cx.emit(TextAreaEvent::Change(self.value.clone()));
            cx.notify();
            return;
        }

        // Ensure cursor position is valid
        self.validate_cursor_position();

        let char_count = self.value.chars().count();
        if self.cursor_pos >= char_count {
            return;
        }

        // Safely remove character at cursor
        self.remove_char_at(self.cursor_pos);

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

        let content = div()
            .id("text-area")
            .track_focus(&self.focus_handle)
            .child(TextAreaElement {
                entity: cx.entity().clone(),
            })
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
                        // Don't handle regular character input here
                        // EntityInputHandler will handle all text input (including IME)
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
                                                    let sel_start_in_line = sel_start.saturating_sub(line_start);
                                                    let sel_end_in_line = if sel_end < line_end {
                                                        sel_end - line_start
                                                    } else {
                                                        line_str.chars().count()
                                                    };
                                                    
                                                    let mut runs = Vec::new();

                                                    // Convert character indices to byte indices for TextRun
                                                    let line_chars: Vec<char> = line_str.chars().collect();

                                                    // Text before selection
                                                    if sel_start_in_line > 0 {
                                                        let before_text: String = line_chars[..sel_start_in_line].iter().collect();
                                                        runs.push(TextRun {
                                                            len: before_text.len(),
                                                            font: font_clone.clone(),
                                                            color: rgb(0x333333).into(),
                                                            background_color: None,
                                                            underline: None,
                                                            strikethrough: None,
                                                        });
                                                    }

                                                    // Selected text
                                                    if sel_end_in_line > sel_start_in_line {
                                                        let selected_text: String = line_chars[sel_start_in_line..sel_end_in_line].iter().collect();
                                                        runs.push(TextRun {
                                                            len: selected_text.len(),
                                                            font: font_clone.clone(),
                                                            color: rgb(0xFFFFFF).into(),
                                                            background_color: Some(rgb(0x4A90E2).into()),
                                                            underline: None,
                                                            strikethrough: None,
                                                        });
                                                    }

                                                    // Text after selection
                                                    if sel_end_in_line < line_chars.len() {
                                                        let after_text: String = line_chars[sel_end_in_line..].iter().collect();
                                                        runs.push(TextRun {
                                                            len: after_text.len(),
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
            ]);

        content
    }
}

// Implement EntityInputHandler for IME (Input Method Editor) support
impl EntityInputHandler for TextArea {
    fn text_for_range(
        &mut self,
        range_utf16: std::ops::Range<usize>,
        actual_range: &mut Option<std::ops::Range<usize>>,
        _window: &mut Window,
        _cx: &mut Context<Self>,
    ) -> Option<String> {
        let range = self.range_from_utf16(&range_utf16);
        actual_range.replace(self.byte_range_to_utf16(&range));
        Some(self.value[range].to_string())
    }

    fn selected_text_range(
        &mut self,
        _ignore_disabled_input: bool,
        _window: &mut Window,
        _cx: &mut Context<Self>,
    ) -> Option<UTF16Selection> {
        let range = self.selected_range();
        Some(UTF16Selection {
            range: self.char_range_to_utf16(&range),
            reversed: false,
        })
    }

    fn marked_text_range(
        &self,
        _window: &mut Window,
        _cx: &mut Context<Self>,
    ) -> Option<std::ops::Range<usize>> {
        self.marked_range
            .as_ref()
            .map(|range| self.byte_range_to_utf16(range))
    }

    fn unmark_text(&mut self, _window: &mut Window, _cx: &mut Context<Self>) {
        self.marked_range = None;
    }

    fn replace_text_in_range(
        &mut self,
        range_utf16: Option<std::ops::Range<usize>>,
        new_text: &str,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if self.disabled {
            return;
        }

        let mut range = range_utf16
            .as_ref()
            .map(|range_utf16| self.range_from_utf16(range_utf16))
            .or(self.marked_range.clone())
            .unwrap_or_else(|| {
                // Convert character selection to byte range
                let char_range = self.selected_range();
                self.char_range_to_byte_range(&char_range)
            });

        // Ensure range is within bounds and on character boundaries
        range.start = range.start.min(self.value.len());
        range.end = range.end.min(self.value.len());

        while range.start > 0 && !self.value.is_char_boundary(range.start) {
            range.start -= 1;
        }
        while range.end > 0 && range.end < self.value.len() && !self.value.is_char_boundary(range.end) {
            range.end += 1;
        }
        if range.end > self.value.len() {
            range.end = self.value.len();
        }

        // Replace text in range
        let mut new_value = String::new();
        new_value.push_str(&self.value[..range.start]);
        new_value.push_str(new_text);
        new_value.push_str(&self.value[range.end..]);

        // Check max length
        if let Some(max_len) = self.max_length {
            if new_value.chars().count() > max_len {
                return;
            }
        }

        // Convert byte position to character position for cursor
        let char_pos_before = self.value[..range.start].chars().count();

        // Apply the change
        self.value = new_value;
        self.cursor_pos = char_pos_before + new_text.chars().count();
        self.selection_start = None;
        self.selection_end = None;
        self.marked_range = None;



        self.pause_blinking(cx);

        // Check for CJK characters and emit IME event if detected
        if Self::contains_cjk(new_text) {
            cx.emit(TextAreaEvent::Ime(self.value.clone()));
        }

        cx.emit(TextAreaEvent::Change(self.value.clone()));
        cx.notify();
    }

    fn replace_and_mark_text_in_range(
        &mut self,
        range_utf16: Option<std::ops::Range<usize>>,
        new_text: &str,
        _new_selected_range_utf16: Option<std::ops::Range<usize>>,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if self.disabled {
            return;
        }
        
        // Get the range to replace
        let byte_range = if let Some(range_utf16) = range_utf16 {
            // Convert UTF-16 range to byte range
            self.range_from_utf16(&range_utf16)
        } else if let Some(marked_range) = &self.marked_range {
            // Use marked range (already in bytes)
            marked_range.clone()
        } else {
            // Use current selection/cursor position
            let selected_range = self.selected_range(); // This is in character indices
            // Convert character range to byte range
            self.char_range_to_byte_range(&selected_range)
        };

        // Ensure range is within bounds and on character boundaries
        let mut range = byte_range;
        range.start = range.start.min(self.value.len());
        range.end = range.end.min(self.value.len());

        while range.start > 0 && !self.value.is_char_boundary(range.start) {
            range.start -= 1;
        }
        while range.end > 0 && range.end < self.value.len() && !self.value.is_char_boundary(range.end) {
            range.end += 1;
        }
        if range.end > self.value.len() {
            range.end = self.value.len();
        }

        // Replace text in range
        let mut new_value = String::new();
        new_value.push_str(&self.value[..range.start]);
        new_value.push_str(new_text);
        new_value.push_str(&self.value[range.end..]);

        // Check max length
        if let Some(max_len) = self.max_length {
            if new_value.chars().count() > max_len {
                return;
            }
        }

        // Convert byte position back to character position for cursor
        let char_pos_before = self.value[..range.start].chars().count();

        // Apply the change
        self.value = new_value;
        self.cursor_pos = char_pos_before + new_text.chars().count();
        self.selection_start = None;
        self.selection_end = None;

        // Set marked range for IME composition (in bytes)
        if !new_text.is_empty() {
            self.marked_range = Some(range.start..(range.start + new_text.len()));
        } else {
            self.marked_range = None;
        }
        
        self.pause_blinking(cx);
        
        // Check for CJK characters and emit IME event if detected
        if Self::contains_cjk(new_text) {
            cx.emit(TextAreaEvent::Ime(self.value.clone()));
        }
        
        cx.emit(TextAreaEvent::Change(self.value.clone()));
        cx.notify();
    }

    fn bounds_for_range(
        &mut self,
        _range_utf16: std::ops::Range<usize>,
        bounds: Bounds<Pixels>,
        _window: &mut Window,
        _cx: &mut Context<Self>,
    ) -> Option<Bounds<Pixels>> {
        // Return the input bounds for IME candidate window positioning
        Some(bounds)
    }

    fn character_index_for_point(
        &mut self,
        point: Point<Pixels>,
        _window: &mut Window,
        _cx: &mut Context<Self>,
    ) -> Option<usize> {
        // Convert pixel position to character index
        let byte_index = self.index_for_mouse_position(point);
        // Convert byte index to UTF-16 code units
        Some(self.value[..byte_index].encode_utf16().count())
    }
}


