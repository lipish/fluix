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
// Password Mask Mode
// ============================================================================

/// Password masking mode for TextInput component.
///
/// Controls how password characters are displayed when `password` mode is enabled
/// and `password_visible` is `false`.
///
/// # Examples
///
/// ## Full Mask Mode (Default)
///
/// All characters are masked with bullets:
///
/// ```rust,no_run
/// use fluix::components::form::text_input::PasswordMaskMode;
/// use fluix::TextInput;
/// use gpui::*;
///
/// # fn example(cx: &mut Context<TextInput>) {
/// let password_input = TextInput::new(cx)
///     .password(true)
///     .password_mask_mode(PasswordMaskMode::All);
/// // Password "f26612345678944u9" displays as "•••••••••••••••••"
/// # }
/// ```
///
/// ## Partial Mask Mode
///
/// Show first and last few characters, mask the middle:
///
/// ```rust,no_run
/// use fluix::components::form::text_input::PasswordMaskMode;
/// use fluix::TextInput;
/// use gpui::*;
///
/// # fn example(cx: &mut Context<TextInput>) {
/// let password_input = TextInput::new(cx)
///     .password(true)
///     .password_mask_mode(PasswordMaskMode::Partial {
///         prefix_len: 2,  // Show first 2 characters
///         suffix_len: 2,  // Show last 2 characters
///     });
/// // Password "f26612345678944u9" displays as "f2••••••••••••••44u9"
/// # }
/// ```
///
/// If the password is too short (length <= prefix_len + suffix_len),
/// all characters will be masked regardless of the mode.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PasswordMaskMode {
    /// Mask all characters with bullets (•).
    ///
    /// This is the default masking mode and provides maximum security
    /// by hiding all password characters.
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use fluix::components::form::text_input::PasswordMaskMode;
    ///
    /// let mode = PasswordMaskMode::All;
    /// ```
    All,
    /// Show first and last few characters, mask the middle part.
    ///
    /// This mode provides a balance between security and usability,
    /// allowing users to verify they're typing the correct password
    /// while still hiding most of it.
    ///
    /// # Parameters
    ///
    /// * `prefix_len` - Number of characters to show at the start (typically 2-4)
    /// * `suffix_len` - Number of characters to show at the end (typically 2-4)
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use fluix::components::form::text_input::PasswordMaskMode;
    ///
    /// // Show first 3 and last 3 characters
    /// let mode = PasswordMaskMode::Partial {
    ///     prefix_len: 3,
    ///     suffix_len: 3,
    /// };
    /// ```
    ///
    /// # Display Format
    ///
    /// The display format is: `{prefix}{masked_middle}{suffix}`
    ///
    /// For example, with `prefix_len: 2` and `suffix_len: 2`:
    /// - Input: `"password123"`
    /// - Display: `"pa••••••••23"`
    Partial {
        /// Number of characters to show at the start of the password.
        ///
        /// Must be a positive integer. Typically 2-4 characters.
        prefix_len: usize,
        /// Number of characters to show at the end of the password.
        ///
        /// Must be a positive integer. Typically 2-4 characters.
        suffix_len: usize,
    },
}

impl Default for PasswordMaskMode {
    fn default() -> Self {
        Self::All
    }
}

// ============================================================================
// Events
// ============================================================================

/// Events emitted by the TextInput component.
///
/// These events allow you to react to user interactions with the input.
///
/// # Examples
///
/// ```rust,no_run
/// use fluix::TextInput;
/// use gpui::*;
///
/// # fn example(cx: &mut Context<TextInput>) {
/// let input = cx.new(|cx| {
///     TextInput::new(cx)
///         .placeholder("Enter text")
/// });
///
/// # let window = todo!();
/// cx.subscribe_in(&input, window, |_, _, event, _, _| {
///     match event {
///         fluix::TextInputEvent::Change(value) => {
///             println!("Value changed to: {}", value);
///         }
///         fluix::TextInputEvent::Submit(value) => {
///             println!("User pressed Enter with value: {}", value);
///         }
///         fluix::TextInputEvent::Focus => {
///             println!("Input gained focus");
///         }
///         fluix::TextInputEvent::Blur => {
///             println!("Input lost focus");
///         }
///     }
/// }).detach();
/// # }
/// ```
#[derive(Clone, Debug)]
pub enum TextInputEvent {
    /// The input value has changed.
    ///
    /// This event is emitted whenever the user types, deletes, or the value
    /// is changed programmatically (via `set_value` or `clear`).
    ///
    /// The event contains the new value as a `String`.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// # use fluix::TextInput;
    /// # use gpui::*;
    /// # fn example(cx: &mut Context<TextInput>) {
    /// # let input = cx.new(|cx| TextInput::new(cx));
    /// # let window = todo!();
    /// cx.subscribe_in(&input, window, |_, _, event, _, _| {
    ///     if let fluix::TextInputEvent::Change(value) = event {
    ///         // Validate or process the new value
    ///         println!("New value: {}", value);
    ///     }
    /// }).detach();
    /// # }
    /// ```
    Change(String),
    /// Enter key was pressed.
    ///
    /// This event is emitted when the user presses the Enter key while
    /// the input is focused. It's commonly used to submit forms or
    /// trigger actions.
    ///
    /// The event contains the current value as a `String`.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// # use fluix::TextInput;
    /// # use gpui::*;
    /// # fn example(cx: &mut Context<TextInput>) {
    /// # let input = cx.new(|cx| TextInput::new(cx));
    /// # let window = todo!();
    /// cx.subscribe_in(&input, window, |_, _, event, _, _| {
    ///     if let fluix::TextInputEvent::Submit(value) = event {
    ///         // Process the submission
    ///         println!("Submitted: {}", value);
    ///     }
    /// }).detach();
    /// # }
    /// ```
    Submit(String),
    /// Input gained focus.
    ///
    /// This event is emitted when the input receives keyboard focus,
    /// either by clicking on it or programmatically via `focus()`.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// # use fluix::TextInput;
    /// # use gpui::*;
    /// # fn example(cx: &mut Context<TextInput>) {
    /// # let input = cx.new(|cx| TextInput::new(cx));
    /// # let window = todo!();
    /// cx.subscribe_in(&input, window, |_, _, event, _, _| {
    ///     if let fluix::TextInputEvent::Focus = event {
    ///         println!("Input is now focused");
    ///     }
    /// }).detach();
    /// # }
    /// ```
    Focus,
    /// Input lost focus.
    ///
    /// This event is emitted when the input loses keyboard focus,
    /// typically when the user clicks elsewhere or tabs away.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// # use fluix::TextInput;
    /// # use gpui::*;
    /// # fn example(cx: &mut Context<TextInput>) {
    /// # let input = cx.new(|cx| TextInput::new(cx));
    /// # let window = todo!();
    /// cx.subscribe_in(&input, window, |_, _, event, _, _| {
    ///     if let fluix::TextInputEvent::Blur = event {
    ///         println!("Input lost focus - validate now");
    ///         // Perform validation or cleanup
    ///     }
    /// }).detach();
    /// # }
    /// ```
    Blur,
}

// ============================================================================
// TextInput Component
// ============================================================================

/// A powerful single-line text input component with full editing capabilities.
///
/// `TextInput` provides a complete text editing experience with features like:
/// - Full keyboard and mouse support
/// - Text selection and cursor management
/// - Password masking with multiple modes
/// - Input validation
/// - IME (Input Method Editor) support for Chinese, Japanese, and Korean
///
/// # Basic Usage
///
/// ```rust,no_run
/// use fluix::TextInput;
/// use gpui::*;
///
/// # fn example(cx: &mut Context<TextInput>) {
/// let input = cx.new(|cx| {
///     TextInput::new(cx)
///         .placeholder("Enter your name")
/// });
///
/// // Subscribe to events
/// # let window = todo!();
/// cx.subscribe_in(&input, window, |_, _, event, _, _| {
///     match event {
///         fluix::TextInputEvent::Change(value) => {
///             println!("Value changed: {}", value);
///         }
///         fluix::TextInputEvent::Submit(value) => {
///             println!("Submitted: {}", value);
///         }
///         _ => {}
///     }
/// }).detach();
/// # }
/// ```
///
/// # Password Input
///
/// ```rust,no_run
/// use fluix::components::form::text_input::PasswordMaskMode;
/// use fluix::TextInput;
/// use gpui::*;
///
/// # fn example(cx: &mut Context<TextInput>) {
/// let password_input = cx.new(|cx| {
///     TextInput::new(cx)
///         .password(true)
///         .password_mask_mode(PasswordMaskMode::Partial {
///             prefix_len: 2,
///             suffix_len: 2,
///         })
///         .placeholder("Enter password")
/// });
/// # }
/// ```
///
/// # See Also
///
/// - [`PasswordMaskMode`] - Password masking modes
/// - [`TextInputEvent`] - Events emitted by TextInput
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
    /// Whether password is visible (only applies when is_password is true)
    password_visible: bool,
    /// Password masking mode
    password_mask_mode: PasswordMaskMode,
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
    /// Create a new TextInput component.
    ///
    /// This constructor creates a new text input with default settings:
    /// - Empty value
    /// - No placeholder
    /// - Enabled (not disabled)
    /// - Password mode disabled
    /// - Password mask mode: `All`
    /// - Password not visible
    /// - No maximum length limit
    /// - No validator
    /// - Border visible
    ///
    /// # Arguments
    ///
    /// * `cx` - The component context
    ///
    /// # Returns
    ///
    /// A new `TextInput` instance ready to be configured with builder methods.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use fluix::TextInput;
    /// use gpui::*;
    ///
    /// # fn example(cx: &mut Context<TextInput>) {
    /// // Create a basic input
    /// let input = TextInput::new(cx);
    ///
    /// // Create and configure in one go
    /// let configured_input = TextInput::new(cx)
    ///     .placeholder("Enter text")
    ///     .value("Initial value")
    ///     .max_length(100);
    /// # }
    /// ```
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
            password_visible: false,
            password_mask_mode: PasswordMaskMode::All,
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

    /// Set the placeholder text displayed when the input is empty.
    ///
    /// The placeholder is shown in a lighter color when the input has no value.
    /// It disappears as soon as the user starts typing.
    ///
    /// # Arguments
    ///
    /// * `placeholder` - The placeholder text to display
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use fluix::TextInput;
    /// use gpui::*;
    ///
    /// # fn example(cx: &mut Context<TextInput>) {
    /// let input = TextInput::new(cx)
    ///     .placeholder("Enter your name");
    /// # }
    /// ```
    pub fn placeholder(mut self, placeholder: impl Into<String>) -> Self {
        self.placeholder = placeholder.into();
        self
    }

    /// Set the initial value for the input.
    ///
    /// This method sets the initial text content and positions the cursor
    /// at the end of the text. Any existing selection is cleared.
    ///
    /// # Arguments
    ///
    /// * `value` - The initial value to set
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use fluix::TextInput;
    /// use gpui::*;
    ///
    /// # fn example(cx: &mut Context<TextInput>) {
    /// let input = TextInput::new(cx)
    ///     .value("Initial value")
    ///     .placeholder("Enter text");
    ///
    /// // Input starts with "Initial value" and cursor at the end
    /// # }
    /// ```
    pub fn value(mut self, value: impl Into<String>) -> Self {
        self.value = value.into();
        self.cursor_position = self.value.len();
        self.selection_start = None;
        self.selection_end = None;
        self
    }

    /// Enable or disable the input.
    ///
    /// When disabled, the input cannot be focused or edited.
    /// It appears with reduced opacity to indicate the disabled state.
    ///
    /// # Arguments
    ///
    /// * `disabled` - `true` to disable the input, `false` to enable it
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use fluix::TextInput;
    /// use gpui::*;
    ///
    /// # fn example(cx: &mut Context<TextInput>) {
    /// let disabled_input = TextInput::new(cx)
    ///     .value("Cannot edit this")
    ///     .disabled(true);
    ///
    /// let enabled_input = TextInput::new(cx)
    ///     .placeholder("Can edit this")
    ///     .disabled(false);
    /// # }
    /// ```
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    /// Enable or disable password mode (mask characters).
    ///
    /// When `is_password` is `true`, the input will mask characters according to
    /// the current `password_mask_mode` and `password_visible` settings.
    ///
    /// # Arguments
    ///
    /// * `is_password` - `true` to enable password mode, `false` to disable
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use fluix::TextInput;
    /// use gpui::*;
    ///
    /// # fn example(cx: &mut Context<TextInput>) {
    /// // Enable password mode
    /// let password_input = TextInput::new(cx)
    ///     .password(true)
    ///     .placeholder("Enter password");
    ///
    /// // Disable password mode (normal text input)
    /// let normal_input = TextInput::new(cx)
    ///     .password(false)
    ///     .placeholder("Enter text");
    /// # }
    /// ```
    pub fn password(mut self, is_password: bool) -> Self {
        self.is_password = is_password;
        self
    }

    /// Toggle password visibility programmatically.
    ///
    /// This method switches between showing and hiding password characters.
    /// It only works when password mode is enabled (`password(true)`).
    ///
    /// # Behavior
    ///
    /// * If password mode is disabled, this method has no effect
    /// * If password is currently visible, it will be hidden
    /// * If password is currently hidden, it will be shown
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use fluix::TextInput;
    /// use gpui::*;
    ///
    /// # fn example(cx: &mut Context<TextInput>) {
    /// let password_input = cx.new(|cx| {
    ///     TextInput::new(cx)
    ///         .password(true)
    ///         .placeholder("Enter password")
    /// });
    ///
    /// // Toggle visibility when user clicks a button
    /// password_input.update(cx, |input, cx| {
    ///     input.toggle_password_visibility(cx);
    /// });
    /// # }
    /// ```
    pub fn toggle_password_visibility(&mut self, cx: &mut Context<Self>) {
        if self.is_password {
            self.password_visible = !self.password_visible;
            cx.notify();
        }
    }

    /// Set password visibility state.
    ///
    /// This method sets the initial visibility state of password characters.
    /// It only has effect when password mode is enabled (`password(true)`).
    ///
    /// # Arguments
    ///
    /// * `visible` - `true` to show password characters, `false` to mask them
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use fluix::TextInput;
    /// use gpui::*;
    ///
    /// # fn example(cx: &mut Context<TextInput>) {
    /// // Password visible by default (not recommended for security)
    /// let visible_password = TextInput::new(cx)
    ///     .password(true)
    ///     .show_password(true)
    ///     .placeholder("Enter password");
    ///
    /// // Password hidden by default (recommended)
    /// let hidden_password = TextInput::new(cx)
    ///     .password(true)
    ///     .show_password(false)  // This is the default
    ///     .placeholder("Enter password");
    /// # }
    /// ```
    pub fn show_password(mut self, visible: bool) -> Self {
        self.password_visible = visible;
        self
    }

    /// Set the password masking mode.
    ///
    /// This method controls how password characters are displayed when
    /// password mode is enabled and password is not visible.
    ///
    /// # Arguments
    ///
    /// * `mode` - The masking mode to use. See [`PasswordMaskMode`] for details.
    ///
    /// # Examples
    ///
    /// ## Full Mask Mode (Default)
    ///
    /// ```rust,no_run
    /// use fluix::components::form::text_input::PasswordMaskMode;
    /// use fluix::TextInput;
    /// use gpui::*;
    ///
    /// # fn example(cx: &mut Context<TextInput>) {
    /// let password_input = TextInput::new(cx)
    ///     .password(true)
    ///     .password_mask_mode(PasswordMaskMode::All)
    ///     .placeholder("Enter password");
    /// // All characters will be masked: "••••••••"
    /// # }
    /// ```
    ///
    /// ## Partial Mask Mode
    ///
    /// ```rust,no_run
    /// use fluix::components::form::text_input::PasswordMaskMode;
    /// use fluix::TextInput;
    /// use gpui::*;
    ///
    /// # fn example(cx: &mut Context<TextInput>) {
    /// let password_input = TextInput::new(cx)
    ///     .password(true)
    ///     .password_mask_mode(PasswordMaskMode::Partial {
    ///         prefix_len: 2,  // Show first 2 characters
    ///         suffix_len: 2,  // Show last 2 characters
    ///     })
    ///     .placeholder("Enter password");
    /// // Password "password123" displays as "pa••••••••23"
    /// # }
    /// ```
    ///
    /// ## Combined with Visibility Toggle
    ///
    /// ```rust,no_run
    /// use fluix::components::form::text_input::PasswordMaskMode;
    /// use fluix::TextInput;
    /// use gpui::*;
    ///
    /// # fn example(cx: &mut Context<TextInput>) {
    /// let password_input = cx.new(|cx| {
    ///     TextInput::new(cx)
    ///         .password(true)
    ///         .password_mask_mode(PasswordMaskMode::Partial {
    ///             prefix_len: 2,
    ///             suffix_len: 2,
    ///         })
    ///         .show_password(false)  // Start hidden
    /// });
    ///
    /// // User can toggle visibility
    /// password_input.update(cx, |input, cx| {
    ///     input.toggle_password_visibility(cx);
    /// });
    /// # }
    /// ```
    pub fn password_mask_mode(mut self, mode: PasswordMaskMode) -> Self {
        self.password_mask_mode = mode;
        self
    }

    /// Set the maximum allowed length for the input value.
    ///
    /// This method limits the number of characters that can be entered.
    /// Attempts to exceed this limit will be rejected.
    ///
    /// # Arguments
    ///
    /// * `max_length` - Maximum number of characters allowed
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use fluix::TextInput;
    /// use gpui::*;
    ///
    /// # fn example(cx: &mut Context<TextInput>) {
    /// // Limit input to 10 characters
    /// let limited_input = TextInput::new(cx)
    ///     .placeholder("Max 10 chars")
    ///     .max_length(10);
    ///
    /// // User cannot type more than 10 characters
    /// # }
    /// ```
    pub fn max_length(mut self, max_length: usize) -> Self {
        self.max_length = Some(max_length);
        self
    }

    /// Set a validation function for the input.
    ///
    /// This method sets a custom validator function that is called whenever
    /// the input value changes. If the validator returns `false`, the change
    /// is rejected and the value remains unchanged.
    ///
    /// # Arguments
    ///
    /// * `validator` - A function that takes a `&str` and returns `bool`
    ///   - `true` means the value is valid and accepted
    ///   - `false` means the value is invalid and rejected
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use fluix::TextInput;
    /// use gpui::*;
    ///
    /// # fn example(cx: &mut Context<TextInput>) {
    /// // Only allow numeric input
    /// let numeric_input = TextInput::new(cx)
    ///     .validator(|value| value.chars().all(|c| c.is_numeric()))
    ///     .placeholder("Numbers only");
    ///
    /// // Email validation
    /// let email_input = TextInput::new(cx)
    ///     .validator(|value| value.contains('@') && value.contains('.'))
    ///     .placeholder("Enter email");
    ///
    /// // Minimum length validation
    /// let password_input = TextInput::new(cx)
    ///     .password(true)
    ///     .validator(|value| value.len() >= 8)
    ///     .placeholder("Password (min 8 chars)");
    /// # }
    /// ```
    pub fn validator(mut self, validator: impl Fn(&str) -> bool + 'static) -> Self {
        self.validator = Some(Arc::new(validator));
        self
    }

    /// Hide the border around the input.
    ///
    /// This method removes the border, making the input borderless.
    /// Useful for embedded use cases like combobox components.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use fluix::TextInput;
    /// use gpui::*;
    ///
    /// # fn example(cx: &mut Context<TextInput>) {
    /// let borderless_input = TextInput::new(cx)
    ///     .placeholder("No border")
    ///     .no_border();
    /// # }
    /// ```
    pub fn no_border(mut self) -> Self {
        self.show_border = false;
        self
    }

    /// Set a custom background color for the input.
    ///
    /// # Arguments
    ///
    /// * `color` - The background color to use
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use fluix::TextInput;
    /// use gpui::*;
    ///
    /// # fn example(cx: &mut Context<TextInput>) {
    /// let styled_input = TextInput::new(cx)
    ///     .placeholder("Light blue background")
    ///     .bg_color(rgb(0xF0F9FF));  // Light blue
    /// # }
    /// ```
    pub fn bg_color(mut self, color: Rgba) -> Self {
        self.custom_bg_color = Some(color);
        self
    }

    /// Set a custom border color for the input.
    ///
    /// # Arguments
    ///
    /// * `color` - The border color to use
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use fluix::TextInput;
    /// use gpui::*;
    ///
    /// # fn example(cx: &mut Context<TextInput>) {
    /// let styled_input = TextInput::new(cx)
    ///     .placeholder("Blue border")
    ///     .border_color(rgb(0x3B82F6));  // Blue
    /// # }
    /// ```
    pub fn border_color(mut self, color: Rgba) -> Self {
        self.custom_border_color = Some(color);
        self
    }

    /// Set the background to transparent.
    ///
    /// This is a convenience method equivalent to `.bg_color(rgba(0x00000000))`.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use fluix::TextInput;
    /// use gpui::*;
    ///
    /// # fn example(cx: &mut Context<TextInput>) {
    /// let transparent_input = TextInput::new(cx)
    ///     .placeholder("Transparent background")
    ///     .transparent()
    ///     .no_border();
    /// # }
    /// ```
    pub fn transparent(mut self) -> Self {
        self.custom_bg_color = Some(rgba(0x00000000));
        self
    }

    /// Get the current input value (read-only).
    ///
    /// This method returns a reference to the current value string.
    /// It does not include any masking characters - you get the actual value.
    ///
    /// # Returns
    ///
    /// A string slice containing the current input value.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use fluix::TextInput;
    /// use gpui::*;
    ///
    /// # fn example(cx: &mut Context<TextInput>) {
    /// let input = cx.new(|cx| {
    ///     TextInput::new(cx)
    ///         .value("Hello, World!")
    /// });
    ///
    /// // Read the current value
    /// let current_value = input.read(cx).get_value();
    /// println!("Current value: {}", current_value);  // "Hello, World!"
    ///
    /// // For password inputs, this returns the actual password, not the masked version
    /// let password_input = cx.new(|cx| {
    ///     TextInput::new(cx)
    ///         .password(true)
    ///         .value("secret123")
    /// });
    /// let password = password_input.read(cx).get_value();
    /// println!("Password: {}", password);  // "secret123" (actual value, not masked)
    /// # }
    /// ```
    pub fn get_value(&self) -> &str {
        &self.value
    }

    /// Set the value programmatically.
    ///
    /// This method allows you to set the input value programmatically.
    /// It performs validation and length checking before setting the value.
    ///
    /// # Arguments
    ///
    /// * `value` - The new value to set
    /// * `cx` - The component context
    ///
    /// # Behavior
    ///
    /// * Validates the value using the validator function (if set)
    /// * Checks against maximum length (if set)
    /// * Moves cursor to the end of the text
    /// * Clears any existing selection
    /// * Emits a `Change` event
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use fluix::TextInput;
    /// use gpui::*;
    ///
    /// # fn example(cx: &mut Context<TextInput>) {
    /// let input = cx.new(|cx| {
    ///     TextInput::new(cx)
    ///         .placeholder("Enter text")
    /// });
    ///
    /// // Set value programmatically
    /// input.update(cx, |input, cx| {
    ///     input.set_value("New value".to_string(), cx);
    /// });
    ///
    /// // With validation
    /// let validated_input = cx.new(|cx| {
    ///     TextInput::new(cx)
    ///         .validator(|v| v.len() >= 3)
    /// });
    ///
    /// validated_input.update(cx, |input, cx| {
    ///     input.set_value("Valid".to_string(), cx);  // OK
    /// });
    ///
    /// validated_input.update(cx, |input, cx| {
    ///     input.set_value("AB".to_string(), cx);  // Rejected (too short)
    /// });
    /// # }
    /// ```
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

    /// Clear all text from the input.
    ///
    /// This method removes all text, resets the cursor position, and clears
    /// any selection. It emits a `Change` event with an empty string.
    ///
    /// # Arguments
    ///
    /// * `cx` - The component context
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use fluix::TextInput;
    /// use gpui::*;
    ///
    /// # fn example(cx: &mut Context<TextInput>) {
    /// let input = cx.new(|cx| {
    ///     TextInput::new(cx)
    ///         .value("Some text")
    /// });
    ///
    /// // Clear the input
    /// input.update(cx, |input, cx| {
    ///     input.clear(cx);
    /// });
    ///
    /// // Input is now empty
    /// let value = input.read(cx).get_value();
    /// assert_eq!(value, "");
    /// # }
    /// ```
    pub fn clear(&mut self, cx: &mut Context<Self>) {
        self.value.clear();
        self.cursor_position = 0;
        self.selection_start = None;
        self.selection_end = None;
        cx.emit(TextInputEvent::Change(String::new()));
        cx.notify();
    }

    /// Focus the input programmatically.
    ///
    /// This method moves keyboard focus to the input, making it ready
    /// for text input. The cursor will be visible and blinking.
    ///
    /// # Arguments
    ///
    /// * `window` - The window containing the input
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use fluix::TextInput;
    /// use gpui::*;
    ///
    /// # fn example(window: &mut Window, cx: &mut Context<TextInput>) {
    /// let input = cx.new(|cx| {
    ///     TextInput::new(cx)
    ///         .placeholder("Enter text")
    /// });
    ///
    /// // Focus the input (e.g., when a button is clicked)
    /// input.read(cx).focus(window);
    ///
    /// // Now the user can type directly without clicking
    /// # }
    /// ```
    pub fn focus(&self, window: &mut Window) {
        self.focus_handle.focus(window);
    }

    /// Select all text in the input.
    ///
    /// This method selects all text, making it easy to replace or delete
    /// all content at once. The selection is highlighted visually.
    ///
    /// # Arguments
    ///
    /// * `cx` - The component context
    ///
    /// # Behavior
    ///
    /// * If the input is empty, this method has no effect
    /// * Sets `selection_start` to 0 and `selection_end` to the end
    /// * Triggers a re-render to show the selection highlight
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use fluix::TextInput;
    /// use gpui::*;
    ///
    /// # fn example(cx: &mut Context<TextInput>) {
    /// let input = cx.new(|cx| {
    ///     TextInput::new(cx)
    ///         .value("Select all this text")
    /// });
    ///
    /// // Select all text programmatically
    /// input.update(cx, |input, cx| {
    ///     input.select_all(cx);
    /// });
    ///
    /// // Now user can type to replace, or press Delete/Backspace to clear
    /// # }
    /// ```
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
        if self.is_password && !self.password_visible {
            match self.password_mask_mode {
                PasswordMaskMode::All => {
                    // Each bullet is 3 bytes, each character in value is variable bytes
                    let bullet_len = "•".len(); // 3 bytes
                    let char_index = byte_index / bullet_len;
                    // Convert character index to byte index in actual value
                    self.value.char_indices().nth(char_index).map(|(i, _)| i).unwrap_or(self.value.len())
                }
                PasswordMaskMode::Partial { prefix_len, suffix_len } => {
                    // For partial mode, map display byte position to value byte position
                    let chars: Vec<char> = self.value.chars().collect();
                    let len = chars.len();
                    
                    if len <= prefix_len + suffix_len {
                        // Too short, treat as all masked
                        let bullet_len = "•".len();
                        let char_index = byte_index / bullet_len;
                        self.value.char_indices().nth(char_index).map(|(i, _)| i).unwrap_or(self.value.len())
                    } else {
                        // Map display byte position to value byte position
                        let mut display_pos = 0;
                        let mut value_pos = 0;
                        
                        for (i, ch) in chars.iter().enumerate() {
                            let ch_bytes = ch.len_utf8();
                            
                            if i < prefix_len {
                                // Prefix characters are visible
                                if display_pos + ch_bytes > byte_index {
                                    break;
                                }
                                display_pos += ch_bytes;
                                value_pos += ch_bytes;
                            } else if i >= len - suffix_len {
                                // Suffix characters are visible
                                if display_pos + ch_bytes > byte_index {
                                    break;
                                }
                                display_pos += ch_bytes;
                                value_pos += ch_bytes;
                            } else {
                                // Middle characters are masked
                                let bullet_len = "•".len();
                                if display_pos + bullet_len > byte_index {
                                    break;
                                }
                                display_pos += bullet_len;
                                value_pos += ch_bytes;
                            }
                        }
                        
                        value_pos.min(self.value.len())
                    }
                }
            }
        } else {
            byte_index.min(self.value.len())
        }
    }

    /// Build TextRun array for rendering with selection support
    fn build_text_runs(&self, font: Font, _font_size: Pixels) -> (String, Vec<TextRun>) {
        let display_text = if self.is_password && !self.password_visible {
            self.build_password_display_text()
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
        let (display_sel_start, display_sel_end) = if self.is_password && !self.password_visible {
            match self.password_mask_mode {
                PasswordMaskMode::All => {
                    // Count characters, not bytes
                    let char_start = self.value[..sel_start].chars().count();
                    let char_end = self.value[..sel_end].chars().count();
                    // Each bullet point is "•".len() = 3 bytes
                    let bullet_len = "•".len();
                    (char_start * bullet_len, char_end * bullet_len)
                }
                PasswordMaskMode::Partial { prefix_len, suffix_len } => {
                    // For partial mode, we need to map byte positions to display positions
                    let chars: Vec<char> = self.value.chars().collect();
                    let len = chars.len();
                    
                    if len <= prefix_len + suffix_len {
                        // Too short, treat as all masked
                        let char_start = self.value[..sel_start].chars().count();
                        let char_end = self.value[..sel_end].chars().count();
                        let bullet_len = "•".len();
                        (char_start * bullet_len, char_end * bullet_len)
                    } else {
                        // Calculate display positions by iterating through characters
                        let mut display_start = 0;
                        let mut display_end = 0;
                        
                        // Convert byte positions to character indices
                        let sel_start_chars = self.value[..sel_start.min(self.value.len())].chars().count();
                        let sel_end_chars = self.value[..sel_end.min(self.value.len())].chars().count();
                        
                        for (char_idx, ch) in chars.iter().enumerate() {
                            if char_idx < sel_start_chars {
                                if char_idx < prefix_len {
                                    // In prefix - each char is 1 char in display
                                    display_start += ch.len_utf8();
                                } else if char_idx >= len - suffix_len {
                                    // In suffix - each char is 1 char in display
                                    display_start += ch.len_utf8();
                                } else {
                                    // In middle - each char is 1 bullet (3 bytes)
                                    display_start += "•".len();
                                }
                            }
                            if char_idx < sel_end_chars {
                                if char_idx < prefix_len {
                                    display_end += ch.len_utf8();
                                } else if char_idx >= len - suffix_len {
                                    display_end += ch.len_utf8();
                                } else {
                                    display_end += "•".len();
                                }
                            }
                        }
                        
                        (display_start, display_end)
                    }
                }
            }
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

    /// Build password display text based on mask mode
    fn build_password_display_text(&self) -> String {
        if self.value.is_empty() {
            return String::new();
        }

        match self.password_mask_mode {
            PasswordMaskMode::All => {
                "•".repeat(self.value.len())
            }
            PasswordMaskMode::Partial { prefix_len, suffix_len } => {
                let chars: Vec<char> = self.value.chars().collect();
                let len = chars.len();
                
                // If password is too short, just mask all
                if len <= prefix_len + suffix_len {
                    return "•".repeat(len);
                }
                
                let prefix: String = chars[..prefix_len.min(len)].iter().collect();
                let suffix: String = chars[len.saturating_sub(suffix_len)..].iter().collect();
                let mask_len = len - prefix_len - suffix_len;
                
                format!("{}{}{}", prefix, "•".repeat(mask_len), suffix)
            }
        }
    }

    fn render_display_text(&self) -> String {
        if self.is_password && !self.password_visible && !self.value.is_empty() {
            self.build_password_display_text()
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
                        let password_visible = self.password_visible;
                        
                        // Calculate display cursor position (for password mode)
                        let display_cursor_pos = if is_password && !password_visible {
                            // Calculate cursor position in display text
                            match self.password_mask_mode {
                                PasswordMaskMode::All => {
                                    // Count characters, not bytes
                                    let char_count = self.value[..cursor_pos.min(self.value.len())].chars().count();
                                    char_count * "•".len()
                                }
                                PasswordMaskMode::Partial { prefix_len, suffix_len } => {
                                    let chars: Vec<char> = self.value.chars().collect();
                                    let len = chars.len();
                                    
                                    if len <= prefix_len + suffix_len {
                                        // Too short, treat as all masked
                                        let char_count = self.value[..cursor_pos.min(self.value.len())].chars().count();
                                        char_count * "•".len()
                                    } else {
                                        // Calculate display position based on prefix/suffix
                                        let mut display_pos = 0;
                                        
                                        for (i, ch) in chars.iter().enumerate() {
                                            let value_byte_pos = self.value[..i].len() + ch.len_utf8();
                                            if value_byte_pos > cursor_pos {
                                                break;
                                            }
                                            
                                            if i < prefix_len {
                                                // Prefix characters are visible
                                                display_pos += ch.len_utf8();
                                            } else if i >= len - suffix_len {
                                                // Suffix characters are visible
                                                display_pos += ch.len_utf8();
                                            } else {
                                                // Middle characters are masked
                                                display_pos += "•".len();
                                            }
                                        }
                                        
                                        display_pos
                                    }
                                }
                            }
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
