use gpui::prelude::FluentBuilder;
use gpui::*;
use crate::theme::*;
use crate::components::basic::icon::{Icon, IconName};
use crate::components::form::select::{SelectOption, DropdownDirection, DropdownAlignment, DropdownWidth};
use crate::components::form::text_input::{TextInput, TextInputEvent};

// ============================================================================
// Events
// ============================================================================

/// Events emitted by the Combobox component
#[derive(Clone, Debug)]
pub enum ComboboxEvent {
    /// Selection changed with the selected value
    Changed(String),
    /// Input value changed
    InputChanged(String),
}

impl EventEmitter<ComboboxEvent> for Combobox {}

// ============================================================================
// Component
// ============================================================================

/// A combobox component that combines text input with dropdown selection
///
/// The core value of Combobox lies in its flexibility. It provides two main interaction modes:
/// 1. Select from preset items: Users can click the arrow button on the right side of the control
///    to expand the dropdown list and select from a preset list of options
/// 2. Free text input: Users can directly input custom values in the text box that don't exist in the list
///
/// # Example
///
/// ```rust,ignore
/// let combobox = cx.new(|cx| {
///     Combobox::new(cx)
///         .placeholder("Search or select...")
///         .options(vec![
///             SelectOption::new("react", "React"),
///             SelectOption::new("vue", "Vue"),
///             SelectOption::new("angular", "Angular"),
///         ])
/// });
///
/// cx.subscribe(&combobox, |this, combobox, event: &ComboboxEvent, cx| {
///     match event {
///         ComboboxEvent::Changed(value) => println!("Selected: {}", value),
///         ComboboxEvent::InputChanged(value) => println!("Input: {}", value),
///     }
/// });
/// ```
pub struct Combobox {
    /// Available options
    options: Vec<SelectOption>,
    /// Currently selected value
    selected_value: Option<String>,
    /// Current input text
    input_value: String,
    /// Placeholder text
    placeholder: String,
    /// Whether the dropdown is open
    is_open: bool,
    /// Whether the combobox is disabled
    disabled: bool,
    /// Size of the combobox
    size: ComponentSize,
    /// Dropdown expansion direction
    dropdown_direction: DropdownDirection,
    /// Dropdown alignment
    dropdown_alignment: DropdownAlignment,
    /// Dropdown width
    dropdown_width: DropdownWidth,
    /// Whether to show border
    show_border: bool,
    /// Whether to show shadow
    show_shadow: bool,
    /// Flag to prevent closing when clicking inside menu
    clicking_menu: bool,
    /// Internal text input component
    text_input: Option<Entity<TextInput>>,
    /// Flag to track if user is actively typing (for filtering)
    is_user_typing: bool,
}

impl Combobox {
    /// Create a new Combobox
    pub fn new(cx: &mut Context<Self>) -> Self {
        let placeholder = "Search or select...".to_string();
        let text_input = cx.new(|cx| {
            TextInput::new(cx)
                .placeholder(placeholder.clone())
                .no_border()
                .transparent()
        });

        Self {
            options: Vec::new(),
            selected_value: None,
            input_value: String::new(),
            placeholder: "Search or select...".to_string(),
            is_open: false,
            disabled: false,
            size: ComponentSize::Medium,
            dropdown_direction: DropdownDirection::Down,
            dropdown_alignment: DropdownAlignment::Left,
            dropdown_width: DropdownWidth::MatchTrigger,
            show_border: true,
            show_shadow: true,
            clicking_menu: false,
            text_input: Some(text_input),
            is_user_typing: false,
        }
    }

    /// Set the options
    pub fn options(mut self, options: Vec<SelectOption>) -> Self {
        self.options = options;
        self
    }

    /// Set the placeholder text
    pub fn placeholder(mut self, placeholder: impl Into<String>) -> Self {
        self.placeholder = placeholder.into();
        self
    }

    /// Set the selected value
    pub fn value(mut self, value: impl Into<String>) -> Self {
        let value_str = value.into();
        self.selected_value = Some(value_str.clone());
        // Find matching option and set input value
        if let Some(option) = self.options.iter().find(|opt| opt.value == value_str) {
            self.input_value = option.label.clone();
        }
        self
    }

    /// Set the input value
    pub fn input_value(mut self, value: impl Into<String>) -> Self {
        self.input_value = value.into();
        self
    }

    /// Set the disabled state
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    /// Set the size
    pub fn size(mut self, size: ComponentSize) -> Self {
        self.size = size;
        self
    }

    /// Set dropdown expansion direction
    pub fn dropdown_direction(mut self, direction: DropdownDirection) -> Self {
        self.dropdown_direction = direction;
        self
    }

    /// Set dropdown alignment
    pub fn dropdown_alignment(mut self, alignment: DropdownAlignment) -> Self {
        self.dropdown_alignment = alignment;
        self
    }

    /// Set dropdown width
    pub fn dropdown_width(mut self, width: DropdownWidth) -> Self {
        self.dropdown_width = width;
        self
    }

    /// Remove border (convenience method)
    pub fn no_border(mut self) -> Self {
        self.show_border = false;
        self
    }

    /// Remove shadow (convenience method)
    pub fn no_shadow(mut self) -> Self {
        self.show_shadow = false;
        self
    }

    /// Filter options based on input value
    /// Only filters when user is actively typing, not when dropdown is opened after selection
    fn filtered_options(&self) -> Vec<SelectOption> {
        // If user is not actively typing, show all options
        if !self.is_user_typing {
            return self.options.clone();
        }

        if self.input_value.is_empty() {
            return self.options.clone();
        }

        let input_lower = self.input_value.to_lowercase();
        self.options
            .iter()
            .filter(|opt| {
                opt.label.to_lowercase().contains(&input_lower) ||
                opt.value.to_lowercase().contains(&input_lower)
            })
            .cloned()
            .collect()
    }

    /// Toggle dropdown open/closed
    fn toggle_dropdown(&mut self) {
        if !self.disabled {
            self.is_open = !self.is_open;
            // When opening dropdown via arrow click, reset typing flag to show all options
            if self.is_open {
                self.is_user_typing = false;
            }
        }
    }

    /// Close the dropdown
    fn close_dropdown(&mut self, cx: &mut Context<Self>) {
        if self.clicking_menu {
            self.clicking_menu = false;
            return;
        }
        if self.is_open {
            self.is_open = false;
            cx.notify();
        }
    }

    /// Select an option
    fn select_option(&mut self, value: String, cx: &mut Context<Self>) {
        if let Some(option) = self.options.iter().find(|opt| opt.value == value) {
            self.selected_value = Some(value.clone());
            self.input_value = option.label.clone();
            self.is_open = false;
            self.is_user_typing = false; // Reset typing flag after selection
            
            // Update TextInput value when option is selected
            // This allows user to continue editing after selection
            if let Some(text_input) = &self.text_input {
                text_input.update(cx, |input, cx| {
                    input.set_value(option.label.clone(), cx);
                });
            }
            
            cx.emit(ComboboxEvent::Changed(value));
            cx.notify();
        }
    }

    /// Render the dropdown overlay
    fn render_dropdown_overlay(&self, cx: &Context<Self>) -> impl IntoElement {
        let theme = Theme::default();
        let filtered_options = self.filtered_options();

        div()
            .absolute()
            .map(|this| match self.dropdown_direction {
                DropdownDirection::Down | DropdownDirection::Auto => {
                    this.top_full().mt(px(0.))  // No gap, connect directly
                }
                DropdownDirection::Up => {
                    this.bottom_full().mb(px(0.))
                }
            })
            .map(|this| match self.dropdown_alignment {
                DropdownAlignment::Left => this.left_0(),
                DropdownAlignment::Right => this.right_0(),
                DropdownAlignment::Center => this.left_0().right_0(),
            })
            .occlude()  // Ensure dropdown is above other content
            .on_mouse_down(MouseButton::Left, cx.listener(|this, _event: &MouseDownEvent, _window, _cx| {
                this.clicking_menu = true;
            }))
            .child(
                div()
                    .occlude()  // Ensure popup content is above other content
                    .id("combobox-popup")
                    .map(|this| match self.dropdown_width {
                        DropdownWidth::MatchTrigger => this,
                        DropdownWidth::Fixed(width) => this.w(width),
                        DropdownWidth::MinWidth(width) => this.min_w(width),
                        DropdownWidth::MaxWidth(width) => this.max_w(width),
                    })
                    .when(matches!(self.dropdown_width, DropdownWidth::MatchTrigger), |this| {
                        this.min_w(px(180.))
                    })
                    .max_h(px(300.))
                    .overflow_y_scroll()
                    // Only round bottom corners when connected to input
                    .rounded_bl(px(BorderRadius::LG))
                    .rounded_br(px(BorderRadius::LG))
                    .border_1()
                    .border_color(theme.colors.border)
                    // Remove top border to connect seamlessly with input
                    .border_t_0()
                    .bg(theme.colors.background)
                    .when(self.show_shadow, |this| {
                        this.shadow(vec![
                            BoxShadow {
                                color: rgba(0x00000010).into(),
                                offset: point(px(0.), px(4.)),
                                blur_radius: px(16.),
                                spread_radius: px(-2.),
                            },
                            BoxShadow {
                                color: rgba(0x00000008).into(),
                                offset: point(px(0.), px(2.)),
                                blur_radius: px(8.),
                                spread_radius: px(0.),
                            },
                        ])
                    })
                    .p(px(6.))
                    .children(filtered_options.iter().enumerate().map(|(idx, option)| {
                        self.render_option(option, ("combobox-item", idx), &theme, cx)
                    }))
            )
    }

    /// Render a single option item
    fn render_option(&self, option: &SelectOption, id: impl Into<ElementId>, theme: &Theme, cx: &Context<Self>) -> impl IntoElement {
        let value = option.value.clone();
        let label = option.label.clone();
        let size = self.size;

        // Check if this option is selected
        // Only show as selected if:
        // 1. selected_value matches this option's value
        // 2. AND input_value matches this option's label (to handle text deletion case)
        let is_selected = if let Some(ref selected_value) = self.selected_value {
            selected_value == &value && self.input_value == label
        } else {
            false
        };

        div()
            .id(id)
            .relative()
            .flex()
            .items_center()
            .justify_between()
            .w_full()
            .px(px(12.))
            .py(px(8.))
            .cursor(CursorStyle::PointingHand)
            .text_size(size.font_size())
            .rounded(px(BorderRadius::SM))
            .map(|this| {
                if is_selected {
                    this.bg(theme.colors.primary)
                        .text_color(rgb(0xFFFFFF))
                } else {
                    this.text_color(theme.colors.text)
                        .hover(|style| style.bg(theme.colors.background_hover))
                }
            })
            .on_mouse_down(MouseButton::Left, cx.listener(move |this, _event: &MouseDownEvent, _window, cx| {
                this.select_option(value.clone(), cx);
            }))
            .child(label)
            .when(is_selected, |this| {
                this.child(
                    div()
                        .text_xs()
                        .child("✓")
                )
            })
    }
}

// ============================================================================
// Render
// ============================================================================

impl Render for Combobox {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = Theme::default();
        let disabled = self.disabled;
        let is_open = self.is_open;
        let text_input = self.text_input.clone();

        // Subscribe to TextInput events - don't sync on every render to avoid overwriting user input
        if let Some(text_input_entity) = &text_input {
            // Only subscribe once, don't sync on every render
            // The TextInput will emit events when user types, and we'll update our state
            let _ = cx.subscribe_in(text_input_entity, window, |this: &mut Self, _input, event: &TextInputEvent, _window, cx| {
                match event {
                    TextInputEvent::Change(value) => {
                        // Update our internal state from user input
                        this.input_value = value.clone();
                        // Always clear selected value when user modifies the text (including deleting)
                        // This ensures that selecting an option and then editing/deleting the text
                        // will clear the selection
                        this.selected_value = None;
                        this.is_user_typing = true; // Mark that user is actively typing
                        // Auto-open dropdown when user starts typing
                        if !this.is_open && !value.is_empty() {
                            this.is_open = true;
                        }
                        // Close dropdown if text is completely deleted
                        if value.is_empty() && this.is_open {
                            this.is_open = false;
                        }
                        cx.emit(ComboboxEvent::InputChanged(value.clone()));
                        cx.notify();
                    }
                    TextInputEvent::Focus => {
                        // Auto-open dropdown when focused
                        if !this.is_open && !this.disabled {
                            this.is_open = true;
                            // When focusing, don't set typing flag - show all options initially
                            this.is_user_typing = false;
                            cx.notify();
                        }
                    }
                    TextInputEvent::Blur => {
                        // Close dropdown when losing focus (but allow clicking dropdown)
                        // This is handled by clicking_menu flag
                    }
                    _ => {}
                }
            });
        }

        div()
            .id("combobox-wrapper")
            .w_full()
            .when(is_open, |this| {
                this.on_mouse_down_out(cx.listener(|this, _event: &MouseDownEvent, _window, cx| {
                    this.close_dropdown(cx);
                }))
            })
            .child(
                div()
                    .id("combobox-container")
                    .relative()
                    .w_full()
                    .child(
                        div()
                            .id("combobox-trigger")
                            .relative()
                            .flex()
                            .w_full()
                            .items_center()
                            .gap_0()
                            .when(is_open, |this| {
                                // When dropdown is open, only round top corners
                                this.rounded_tl(px(BorderRadius::LG))
                                    .rounded_tr(px(BorderRadius::LG))
                            })
                            .when(!is_open, |this| {
                                // When dropdown is closed, round all corners
                                this.rounded(px(BorderRadius::LG))
                            })
                            .when(self.show_border, |this| {
                                this.border_1()
                                    .border_color(theme.colors.border)
                                    // When dropdown is open, remove bottom border to connect seamlessly
                                    // When closed, show all borders including bottom
                                    .when(is_open, |this| {
                                        this.border_b_0()
                                    })
                            })
                            .bg(theme.colors.background)
                            .min_h(px(36.))  // Ensure minimum height matches TextInput
                            .when(self.show_shadow, |this| {
                                this.shadow(vec![BoxShadow {
                                    color: rgba(0x0000000A).into(),
                                    offset: point(px(0.), px(1.)),
                                    blur_radius: px(2.),
                                    spread_radius: px(0.),
                                }])
                            })
                            .when(disabled, |this| {
                                this.opacity(0.64)
                            })
                            .child(
                                div()
                                    .flex_1()
                                    .overflow_hidden()
                                    .when_some(text_input.clone(), |this, input| {
                                        // TextInput container - TextInput has no border and transparent background
                                        // The outer combobox-trigger provides the unified border
                                        this.child(
                                            div()
                                                .w_full()
                                                .h_full()
                                                .bg(theme.colors.background)
                                                .child(input.clone())
                                        )
                                    })
                            )
                            .child(
                                div()
                                    .flex()
                                    .items_center()
                                    .justify_center()
                                    .px(px(8.))
                                    .flex_none()
                                    .cursor(CursorStyle::PointingHand)
                                    .on_mouse_down(MouseButton::Left, cx.listener(|this, _event: &MouseDownEvent, _window, cx| {
                                        this.toggle_dropdown();
                                        cx.notify();
                                    }))
                                    .child(
                                        Icon::new(IconName::ChevronUpDown)
                                            .small()
                                            .color(rgb(0x666666))
                                    )
                            )
                    )
                    .children(if is_open && !disabled {
                        Some(deferred(
                            self.render_dropdown_overlay(cx).into_any_element()
                        ))
                    } else {
                        None
                    })
            )
    }
}

