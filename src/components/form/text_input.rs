use gpui::*;
use gpui::prelude::FluentBuilder;
use std::sync::{Arc, Mutex};

// Element state for storing layout information
#[derive(Clone)]
struct TextInputLayout {
    bounds: Bounds<Pixels>,
    shaped_line: ShapedLine,
}

// Custom element wrapper to register input handler
struct TextInputElement {
    entity: Entity<TextInput>,
}

impl IntoElement for TextInputElement {
    type Element = Self;

    fn into_element(self) -> Self::Element {
        self
    }
}

impl Element for TextInputElement {
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
    /// Selection start position (byte offset, None if no selection)
    selection_start: Option<usize>,
    /// Selection end position (byte offset, None if no selection)
    selection_end: Option<usize>,
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
    /// Blink epoch - increments when cursor should reset to visible
    blink_epoch: usize,
    /// Whether cursor is currently visible (for blinking)
    cursor_visible: bool,
    /// Blink task handle
    _blink_task: Option<Task<()>>,
    /// Whether mouse is currently dragging for selection
    is_dragging: bool,
    /// Last layout info for mouse position calculation
    last_layout: Option<TextInputLayout>,
    /// Marked text range for IME (Input Method Editor) composition
    marked_range: Option<std::ops::Range<usize>>,
    /// Whether to show border (for embedded use cases like combobox)
    show_border: bool,
    /// Custom background color (None uses default)
    custom_bg_color: Option<Rgba>,
    /// Custom border color (None uses default)
    custom_border_color: Option<Rgba>,
}

impl TextInput {
    /// Create a new TextInput
    pub fn new(cx: &mut Context<Self>) -> Self {
        Self {
            value: String::new(),
            cursor_position: 0,
            selection_start: None,
            selection_end: None,
            placeholder: String::new(),
            focus_handle: cx.focus_handle(),
            disabled: false,
            is_password: false,
            max_length: None,
            validator: None,
            blink_epoch: 0,
            cursor_visible: true,
            _blink_task: None,
            is_dragging: false,
            last_layout: None,
            marked_range: None,
            show_border: true,
            custom_bg_color: None,
            custom_border_color: None,
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
        self.selection_start = None;
        self.selection_end = None;
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

    /// Hide border (useful for embedded use cases like combobox)
    pub fn no_border(mut self) -> Self {
        self.show_border = false;
        self
    }

    /// Set custom background color
    pub fn bg_color(mut self, color: Rgba) -> Self {
        self.custom_bg_color = Some(color);
        self
    }

    /// Set custom border color
    pub fn border_color(mut self, color: Rgba) -> Self {
        self.custom_border_color = Some(color);
        self
    }

    /// Set transparent background
    pub fn transparent(mut self) -> Self {
        self.custom_bg_color = Some(rgba(0x00000000));
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
        self.selection_start = None;
        self.selection_end = None;
        cx.emit(TextInputEvent::Change(value));
        cx.notify();
    }

    /// Clear the input
    pub fn clear(&mut self, cx: &mut Context<Self>) {
        self.value.clear();
        self.cursor_position = 0;
        self.selection_start = None;
        self.selection_end = None;
        cx.emit(TextInputEvent::Change(String::new()));
        cx.notify();
    }

    /// Focus the input
    pub fn focus(&self, window: &mut Window) {
        self.focus_handle.focus(window);
    }

    /// Select all text
    pub fn select_all(&mut self, cx: &mut Context<Self>) {
        if !self.value.is_empty() {
            self.selection_start = Some(0);
            self.selection_end = Some(self.value.len());
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

    /// Calculate character index from mouse position
    fn index_for_mouse_position(&self, position: Point<Pixels>) -> usize {
        if self.value.is_empty() {
            return 0;
        }

        let Some(layout) = &self.last_layout else {
            return self.value.len();
        };

        // Check if click is within bounds
        if position.y < layout.bounds.top() {
            return 0;
        }
        if position.y > layout.bounds.bottom() {
            return self.value.len();
        }

        // Calculate relative x position
        let relative_x = position.x - layout.bounds.left();
        
        // Use ShapedLine to find closest character index
        let byte_index = layout.shaped_line.closest_index_for_x(relative_x);
        
        // For password mode, we need to convert from display text index to actual value index
        if self.is_password {
            // Each bullet is 3 bytes, each character in value is variable bytes
            let bullet_len = "•".len(); // 3 bytes
            let char_index = byte_index / bullet_len;
            // Convert character index to byte index in actual value
            self.value.char_indices().nth(char_index).map(|(i, _)| i).unwrap_or(self.value.len())
        } else {
            byte_index.min(self.value.len())
        }
    }

    /// Build TextRun array for rendering with selection support
    fn build_text_runs(&self, font: Font, _font_size: Pixels) -> (String, Vec<TextRun>) {
        let display_text = if self.is_password {
            "•".repeat(self.value.len())
        } else {
            self.value.clone()
        };

        if !self.has_selection() {
            // No selection: single text run
            return (
                display_text.clone(),
                vec![TextRun {
                    len: display_text.len(),
                    font,
                    color: rgb(0x333333).into(),
                    background_color: None,
                    underline: None,
                    strikethrough: None,
                }],
            );
        }

        // Has selection: build three text runs
        // IMPORTANT: Convert byte indices to character indices for password mode
        let (mut sel_start, mut sel_end) = if let (Some(start), Some(end)) =
            (self.selection_start, self.selection_end)
        {
            if start <= end {
                (start, end)
            } else {
                (end, start)
            }
        } else {
            (0, 0)
        };

        // Ensure indices are within bounds and on character boundaries
        sel_start = sel_start.min(self.value.len());
        sel_end = sel_end.min(self.value.len());
        
        while sel_start > 0 && !self.value.is_char_boundary(sel_start) {
            sel_start -= 1;
        }
        while sel_end > 0 && sel_end < self.value.len() && !self.value.is_char_boundary(sel_end) {
            sel_end += 1;
        }
        if sel_end > self.value.len() {
            sel_end = self.value.len();
        }

        // For password mode, we need to calculate positions in the display text
        // Each character in value becomes one bullet point (•) in display_text
        let (display_sel_start, display_sel_end) = if self.is_password {
            // Count characters, not bytes
            let char_start = self.value[..sel_start].chars().count();
            let char_end = self.value[..sel_end].chars().count();
            // Each bullet point is "•".len() = 3 bytes
            let bullet_len = "•".len();
            (char_start * bullet_len, char_end * bullet_len)
        } else {
            (sel_start, sel_end)
        };

        let mut runs = Vec::new();

        // Text before selection
        if display_sel_start > 0 {
            runs.push(TextRun {
                len: display_sel_start,
                font: font.clone(),
                color: rgb(0x333333).into(),
                background_color: None,
                underline: None,
                strikethrough: None,
            });
        }

        // Selected text - background color is a TextRun property!
        if display_sel_end > display_sel_start {
            runs.push(TextRun {
                len: display_sel_end - display_sel_start,
                font: font.clone(),
                color: rgb(0xFFFFFF).into(),
                background_color: Some(rgb(0x4A90E2).into()),
                underline: None,
                strikethrough: None,
            });
        }

        // Text after selection
        if display_sel_end < display_text.len() {
            runs.push(TextRun {
                len: display_text.len() - display_sel_end,
                font: font.clone(),
                color: rgb(0x333333).into(),
                background_color: None,
                underline: None,
                strikethrough: None,
            });
        }

        (display_text, runs)
    }

    /// Delete selected text and return the new cursor position
    fn delete_selection(&mut self) -> usize {
        if let (Some(start), Some(end)) = (self.selection_start, self.selection_end) {
            let (mut sel_start, mut sel_end) = if start <= end {
                (start, end)
            } else {
                (end, start)
            };

            // Ensure indices are within bounds
            sel_start = sel_start.min(self.value.len());
            sel_end = sel_end.min(self.value.len());

            // Ensure indices are on character boundaries
            while sel_start > 0 && !self.value.is_char_boundary(sel_start) {
                sel_start -= 1;
            }
            while sel_end > 0 && sel_end < self.value.len() && !self.value.is_char_boundary(sel_end) {
                sel_end += 1;
            }

            if sel_start < sel_end && sel_end <= self.value.len() {
                let before = self.value[..sel_start].to_string();
                let after = self.value[sel_end..].to_string();
                self.value = format!("{}{}", before, after);
            }
            self.clear_selection();
            sel_start
        } else {
            self.cursor_position
        }
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
            self.cursor_position = self.delete_selection();
            self.pause_blinking(cx);
            cx.emit(TextInputEvent::Change(self.value.clone()));
            cx.notify();
            return;
        }

        if self.cursor_position == 0 {
            return;
        }

        // Ensure cursor position is within bounds and on a character boundary
        if self.cursor_position > self.value.len() {
            self.cursor_position = self.value.len();
        }
        if !self.value.is_char_boundary(self.cursor_position) {
            // Find the nearest valid boundary
            while self.cursor_position > 0 && !self.value.is_char_boundary(self.cursor_position) {
                self.cursor_position -= 1;
            }
        }

        // Find the previous character boundary (handles multi-byte characters)
        let mut prev_pos = self.cursor_position;
        while prev_pos > 0 {
            prev_pos -= 1;
            if self.value.is_char_boundary(prev_pos) {
                break;
            }
        }

        // Remove character before cursor
        let mut new_value = String::new();
        new_value.push_str(&self.value[..prev_pos]);
        if self.cursor_position <= self.value.len() {
            new_value.push_str(&self.value[self.cursor_position..]);
        }

        self.value = new_value.clone();
        self.cursor_position = prev_pos;
        self.pause_blinking(cx);
        cx.emit(TextInputEvent::Change(new_value));
        cx.notify();
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
            self.cursor_position = self.delete_selection();
            self.pause_blinking(cx);
            cx.emit(TextInputEvent::Change(self.value.clone()));
            cx.notify();
            return;
        }

        // Ensure cursor position is within bounds and on a character boundary
        if self.cursor_position > self.value.len() {
            self.cursor_position = self.value.len();
        }
        if self.cursor_position >= self.value.len() {
            return;
        }
        if !self.value.is_char_boundary(self.cursor_position) {
            // Find the nearest valid boundary
            while self.cursor_position > 0 && !self.value.is_char_boundary(self.cursor_position) {
                self.cursor_position -= 1;
            }
        }

        // Find the next character boundary (handles multi-byte characters)
        let mut next_pos = self.cursor_position + 1;
        while next_pos < self.value.len() && !self.value.is_char_boundary(next_pos) {
            next_pos += 1;
        }

        // Remove character at cursor
        let mut new_value = String::new();
        if self.cursor_position <= self.value.len() {
            new_value.push_str(&self.value[..self.cursor_position]);
        }
        if next_pos <= self.value.len() {
            new_value.push_str(&self.value[next_pos..]);
        }

        self.value = new_value.clone();
        self.pause_blinking(cx);
        cx.emit(TextInputEvent::Change(new_value));
        cx.notify();
    }

    /// Extend or start selection to the cursor position
    fn extend_selection_to(&mut self, pos: usize) {
        if !self.has_selection() {
            // Start new selection: use the previous cursor position as anchor
            // But since cursor is already moved, we need to track anchor separately
            // For now, use current position as end, and adjust after cursor move
            // This is a simplified approach - in a full implementation, we'd track the anchor
            self.selection_start = Some(pos);
            self.selection_end = Some(pos);
        } else {
            // Extend existing selection
            // Determine which end to extend based on which is closer to the new position
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
    }

    fn move_cursor_left(&mut self, extend_selection: bool, cx: &mut Context<Self>) {
        if self.cursor_position > 0 {
            let old_pos = self.cursor_position;
            
            // Find the previous character boundary (handles multi-byte characters)
            let mut new_pos = self.cursor_position;
            while new_pos > 0 {
                new_pos -= 1;
                if self.value.is_char_boundary(new_pos) {
                    break;
                }
            }
            self.cursor_position = new_pos;
            
            if extend_selection {
                if !self.has_selection() {
                    // Start new selection: old position is anchor, new position is end
                    self.selection_start = Some(old_pos);
                    self.selection_end = Some(self.cursor_position);
                } else {
                    self.extend_selection_to(self.cursor_position);
                }
            } else {
                self.clear_selection();
            }
            self.pause_blinking(cx);
            cx.notify();
        }
    }

    fn move_cursor_right(&mut self, extend_selection: bool, cx: &mut Context<Self>) {
        if self.cursor_position < self.value.len() {
            let old_pos = self.cursor_position;
            
            // Find the next character boundary (handles multi-byte characters)
            let mut new_pos = self.cursor_position + 1;
            while new_pos < self.value.len() && !self.value.is_char_boundary(new_pos) {
                new_pos += 1;
            }
            self.cursor_position = new_pos;
            
            if extend_selection {
                if !self.has_selection() {
                    // Start new selection: old position is anchor, new position is end
                    self.selection_start = Some(old_pos);
                    self.selection_end = Some(self.cursor_position);
                } else {
                    self.extend_selection_to(self.cursor_position);
                }
            } else {
                self.clear_selection();
            }
            self.pause_blinking(cx);
            cx.notify();
        }
    }

    fn move_cursor_home(&mut self, extend_selection: bool, cx: &mut Context<Self>) {
        let old_pos = self.cursor_position;
        self.cursor_position = 0;
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
        let old_pos = self.cursor_position;
        self.cursor_position = self.value.len();
        if extend_selection {
            if !self.has_selection() {
                self.selection_start = Some(old_pos);
                self.selection_end = Some(self.value.len());
            } else {
                self.extend_selection_to(self.value.len());
            }
        } else {
            self.clear_selection();
        }
        self.pause_blinking(cx);
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

    /// Convert byte range to UTF-16 code unit range
    fn range_to_utf16(&self, range: &std::ops::Range<usize>) -> std::ops::Range<usize> {
        let start_idx = range.start.min(self.value.len());
        let end_idx = range.end.min(self.value.len());
        
        let start = self.value[..start_idx].encode_utf16().count();
        let end = self.value[..end_idx].encode_utf16().count();
        start..end
    }

    /// Convert UTF-16 code unit range to byte range
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

    /// Get current selection as a range
    fn selected_range(&self) -> std::ops::Range<usize> {
        if let (Some(start), Some(end)) = (self.selection_start, self.selection_end) {
            if start <= end {
                start..end
            } else {
                end..start
            }
        } else {
            self.cursor_position..self.cursor_position
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
        
        // Start blinking when focused
        if is_focused && self._blink_task.is_none() {
            let epoch = self.next_blink_epoch();
            self.start_blinking(epoch, cx);
        }
        
        let display_text = self.render_display_text();
        let show_placeholder = self.value.is_empty();
        let disabled = self.disabled;
        let placeholder = self.placeholder.clone();
        
        // Create a shared container for layout info that will be filled during paint
        let layout_container: Arc<Mutex<Option<TextInputLayout>>> = Arc::new(Mutex::new(None));

        div()
            .id("text-input")
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
                        this.handle_submit(cx);
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
                    cx.emit(TextInputEvent::Focus);
                    cx.focus_self(window);
                    
                    // Update last_layout from the container
                    if let Ok(layout) = layout_container.lock() {
                        this.last_layout = layout.clone();
                    }
                    
                    // Start dragging
                    this.is_dragging = true;
                    
                    // Calculate click position
                    let index = this.index_for_mouse_position(event.position);
                    
                    if event.modifiers.shift {
                        // Shift+click extends selection
                        if !this.has_selection() {
                            this.selection_start = Some(this.cursor_position);
                            this.selection_end = Some(index);
                        } else {
                            // Extend existing selection
                            this.selection_end = Some(index);
                        }
                    } else {
                        // Normal click - clear selection and set cursor
                        this.clear_selection();
                    }
                    
                    this.cursor_position = index;
                    this.pause_blinking(cx);
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
                        this.selection_start = Some(this.cursor_position);
                        this.selection_end = Some(index);
                    } else {
                        // Update selection end
                        this.selection_end = Some(index);
                    }
                    
                    this.cursor_position = index;
                    cx.notify();
                }
            }))
            .flex()
            .items_center()
            .w_full()
            .h(px(36.))
            .px_3()
            .bg(self.custom_bg_color.unwrap_or(if disabled {
                rgb(0xF5F5F5)
            } else {
                rgb(0xFFFFFF)
            }))
            .when(self.show_border, |this| {
                this.border_1()
                    .border_color(self.custom_border_color.unwrap_or(if is_focused {
                        rgb(0x696FC7)
                    } else {
                        rgb(0xE0E0E0)
                    }))
            })
            .rounded(px(6.))
            .when(!disabled, |this| {
                this.cursor(CursorStyle::IBeam)
            })
            .child(TextInputElement {
                entity: cx.entity().clone(),
            })
            .child(
                div()
                    .flex()
                    .items_center()
                    .flex_1()
                    .min_w(px(0.))  // Allow shrinking to prevent content from expanding
                    .text_sm()
                    .when(show_placeholder, |this| {
                        // Show placeholder with cursor when focused
                        if is_focused && !disabled && !self.has_selection() && self.cursor_visible {
                            this.text_color(rgb(0x999999))
                                .relative()
                                .child(placeholder)
                                .child(
                                    // Cursor at position 0 (empty input) - blinking
                                    div()
                                        .absolute()
                                        .left(px(0.))
                                        .top(px(2.))  // Adjusted to match input cursor position
                                        .w(px(2.))
                                        .h(px(18.))
                                        .bg(rgb(0x000000))  // Black cursor
                                )
                        } else {
                            this.text_color(rgb(0x999999))
                                .child(placeholder)
                        }
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
                        // Focused: show text with cursor and selection using TextRun API
                        let cursor_pos = self.cursor_position;
                        
                        // Get font from settings (you may need to adjust this)
                        let font = gpui::Font {
                            family: ".SystemUIFont".into(),
                            features: Default::default(),
                            weight: Default::default(),
                            style: Default::default(),
                            fallbacks: None,
                        };
                        let font_size = px(14.);
                        
                        // Build TextRun array with selection support
                        let (display_text, text_runs) = self.build_text_runs(font.clone(), font_size);
                        let display_text_for_cursor = display_text.clone();  // Clone for cursor calculation
                        let has_selection = self.has_selection();
                        let cursor_visible = self.cursor_visible;
                        let is_password = self.is_password;
                        
                        // Calculate display cursor position (for password mode)
                        let display_cursor_pos = if is_password {
                            // Convert byte index to character count, then to display bytes
                            let char_count = self.value[..cursor_pos.min(self.value.len())].chars().count();
                            char_count * "•".len()
                        } else {
                            cursor_pos
                        };
                        
                        // Clone layout container for use in canvas closure
                        let layout_container_for_canvas = layout_container.clone();
                        
                        // Use TextRun API to render text with selection
                        // This ensures consistent width regardless of selection state
                        
                        this.child(
                            div()
                                .flex()
                                .items_center()
                                .h_full()
                                .w_full()  // Ensure parent takes full width
                                .relative()  // For absolute cursor positioning
                                .child(
                                    canvas(
                                        move |bounds, _, _cx| {
                                            // Return the size for layout - match line height
                                            gpui::size(bounds.size.width, px(18.))
                                        },
                                        move |bounds, _, window, _cx| {
                                            let line_height = px(18.);
                                            
                                            // Only shape and paint if there's text
                                            if !display_text.is_empty() {
                                                // Shape the line using TextRun
                                                let shaped_line = window.text_system().shape_line(
                                                    display_text.clone().into(),
                                                    font_size,
                                                    &text_runs,
                                                    None,
                                                );
                                                
                                                // Save layout info for mouse position calculation
                                                if let Ok(mut layout) = layout_container_for_canvas.lock() {
                                                    *layout = Some(TextInputLayout {
                                                        bounds,
                                                        shaped_line: shaped_line.clone(),
                                                    });
                                                }
                                                
                                                // origin should be the TOP of the line, not the baseline
                                                // paint() and paint_background() will calculate baseline internally
                                                let origin = bounds.origin;
                                                
                                                // IMPORTANT: Paint background first!
                                                shaped_line.paint_background(origin, line_height, window, _cx).ok();
                                                
                                                // Then paint the text
                                                shaped_line.paint(origin, line_height, window, _cx).ok();
                                            }
                                            
                                            // Don't draw cursor in canvas - we'll use a div instead
                                        },
                                    )
                                    .w_full()
                                    .h(px(18.))
                                )
                                // Show cursor using canvas to get accurate position
                                .child(
                                    canvas(
                                        move |_bounds, _, _cx| {
                                            gpui::size(px(0.), px(0.))  // Zero size, just for painting
                                        },
                                        move |bounds, _, window, _cx| {
                                            if !has_selection && cursor_visible {
                                                // Calculate accurate cursor position using text measurement
                                                let cursor_x = if display_cursor_pos == 0 || display_text_for_cursor.is_empty() {
                                                    px(0.)
                                                } else {
                                                    // Get text before cursor in display text
                                                    let text_before = if display_cursor_pos <= display_text_for_cursor.len() {
                                                        display_text_for_cursor[..display_cursor_pos].to_string()
                                                    } else {
                                                        display_text_for_cursor.clone()
                                                    };
                                                    
                                                    if text_before.is_empty() {
                                                        px(0.)
                                                    } else {
                                                        // Use actual text measurement
                                                        let temp_runs = vec![TextRun {
                                                            len: text_before.len(),
                                                            font: font.clone(),
                                                            color: rgb(0x333333).into(),
                                                            background_color: None,
                                                            underline: None,
                                                            strikethrough: None,
                                                        }];
                                                        let temp_line = window.text_system().shape_line(
                                                            text_before.into(),
                                                            font_size,
                                                            &temp_runs,
                                                            None,
                                                        );
                                                        temp_line.width
                                                    }
                                                };
                                                
                                                // Draw cursor - shift down 1px to align with text
                                                let cursor_bounds = gpui::Bounds {
                                                    origin: bounds.origin + gpui::point(cursor_x, px(1.)),
                                                    size: gpui::size(px(2.), px(18.)),
                                                };
                                                window.paint_quad(gpui::fill(cursor_bounds, rgb(0x000000)));
                                            }
                                        },
                                    )
                                    .absolute()
                                    .top(px(0.))
                                    .left(px(0.))
                                    .w(px(0.))
                                    .h(px(18.))
                                )
                        )
                    })
            )
    }
}

// Implement EntityInputHandler for IME (Input Method Editor) support
impl EntityInputHandler for TextInput {
    fn text_for_range(
        &mut self,
        range_utf16: std::ops::Range<usize>,
        actual_range: &mut Option<std::ops::Range<usize>>,
        _window: &mut Window,
        _cx: &mut Context<Self>,
    ) -> Option<String> {
        let range = self.range_from_utf16(&range_utf16);
        actual_range.replace(self.range_to_utf16(&range));
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
            range: self.range_to_utf16(&range),
            reversed: false, // Simplified - could track selection direction
        })
    }

    fn marked_text_range(
        &self,
        _window: &mut Window,
        _cx: &mut Context<Self>,
    ) -> Option<std::ops::Range<usize>> {
        self.marked_range
            .as_ref()
            .map(|range| self.range_to_utf16(range))
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
        let mut range = range_utf16
            .as_ref()
            .map(|range_utf16| self.range_from_utf16(range_utf16))
            .or(self.marked_range.clone())
            .unwrap_or_else(|| self.selected_range());

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

        self.value = new_value;
        self.cursor_position = range.start + new_text.len();
        self.clear_selection();
        self.marked_range = None;
        
        self.pause_blinking(cx);
        cx.emit(TextInputEvent::Change(self.value.clone()));
        cx.notify();
    }

    fn replace_and_mark_text_in_range(
        &mut self,
        range_utf16: Option<std::ops::Range<usize>>,
        new_text: &str,
        new_selected_range_utf16: Option<std::ops::Range<usize>>,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let mut range = range_utf16
            .as_ref()
            .map(|range_utf16| self.range_from_utf16(range_utf16))
            .or(self.marked_range.clone())
            .unwrap_or_else(|| self.selected_range());

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

        self.value = new_value;

        // Mark the newly inserted text for IME composition
        if !new_text.is_empty() {
            self.marked_range = Some(range.start..range.start + new_text.len());
        } else {
            self.marked_range = None;
        }

        // Update selection
        if let Some(new_range_utf16) = new_selected_range_utf16 {
            let new_range = self.range_from_utf16(&new_range_utf16);
            self.cursor_position = range.start + new_range.end;
        } else {
            self.cursor_position = range.start + new_text.len();
        }
        self.clear_selection();

        self.pause_blinking(cx);
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
