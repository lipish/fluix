use gpui::prelude::FluentBuilder;
use gpui::*;
use crate::theme::*;
use crate::components::basic::icon::{Icon, IconName};
use crate::components::form::select::{SelectOption, SelectOptionGroup, DropdownDirection, DropdownAlignment, DropdownWidth};
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
    /// Available options (flat list)
    options: Vec<SelectOption>,
    /// Grouped options
    option_groups: Vec<SelectOptionGroup>,
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
    /// Whether to use transparent background
    transparent_background: bool,
    /// Flag to prevent closing when clicking inside menu
    clicking_menu: bool,
    /// Internal text input component
    text_input: Option<Entity<TextInput>>,
    /// Flag to track if user is actively typing ( for filtering)
    is_user_typing: bool,
    /// Whether to use compact spacing for dropdown items
    compact: bool,
    /// Calculated width based on input text (for dynamic sizing)
    calculated_width: Option<f32>,
    /// Flag to blur the input after selection
    should_blur: bool,
    /// Focus handle for the combobox
    focus_handle: FocusHandle,
    /// Whether to use fixed width (text and button don't move with content)
    /// When true: width is fixed, text and button stay in place (good with borders)
    /// When false: width adjusts with content, text and button move (good without borders)
    fixed_width: bool,
    /// Event subscriptions
    _subscriptions: Vec<Subscription>,
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
                .right_padding(2.0) // Further reduced right padding for tighter spacing with icon
        });

        Self {
            options: Vec::new(),
            option_groups: Vec::new(),
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
            transparent_background: false,
            clicking_menu: false,
            text_input: Some(text_input),
            is_user_typing: false,
            compact: false,
            calculated_width: None,
            should_blur: false,
            focus_handle: cx.focus_handle(),
            fixed_width: false, // Default to dynamic width
            _subscriptions: Vec::new(),
        }
    }

    /// Set the options
    pub fn options(mut self, options: Vec<SelectOption>) -> Self {
        self.options = options;
        self.option_groups.clear(); // Clear groups when setting flat options
        self
    }

    /// Set the option groups
    pub fn option_groups(mut self, groups: Vec<SelectOptionGroup>) -> Self {
        self.option_groups = groups;
        self.options.clear(); // Clear flat options when setting groups
        self
    }

    /// Use compact spacing for dropdown items (less padding)
    pub fn compact(mut self) -> Self {
        self.compact = true;
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
        let all_options = self.all_options();
        if let Some(option) = all_options.iter().find(|opt| opt.value == value_str) {
            self.input_value = option.label.clone();
        }
        self
    }

    /// Set the selected value (mutable reference version for use in update closures)
    pub fn set_value(&mut self, value: impl Into<String>) {
        let value_str = value.into();
        self.selected_value = Some(value_str.clone());
        // Find matching option and set input value
        let all_options = self.all_options();
        if let Some(option) = all_options.iter().find(|opt| opt.value == value_str) {
            self.input_value = option.label.clone();
        }
    }

    /// Set the input value
    pub fn input_value(mut self, value: impl Into<String>) -> Self {
        self.input_value = value.into();
        self
    }

    /// Set the input value (mutable reference version for use in update closures)
    pub fn set_input_value(&mut self, value: impl Into<String>) {
        self.input_value = value.into();
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

    /// Set fixed dropdown width (convenience method)
    pub fn dropdown_fixed_width(mut self, width: Pixels) -> Self {
        self.dropdown_width = DropdownWidth::Fixed(width);
        self
    }

    /// Set minimum dropdown width (convenience method)
    pub fn dropdown_min_width(mut self, width: Pixels) -> Self {
        self.dropdown_width = DropdownWidth::MinWidth(width);
        self
    }

    /// Set maximum dropdown width (convenience method)
    pub fn dropdown_max_width(mut self, width: Pixels) -> Self {
        self.dropdown_width = DropdownWidth::MaxWidth(width);
        self
    }

    /// Match dropdown width to trigger width (convenience method)
    pub fn dropdown_match_trigger(mut self) -> Self {
        self.dropdown_width = DropdownWidth::MatchTrigger;
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

    /// Use transparent background (convenience method)
    pub fn transparent_background(mut self) -> Self {
        self.transparent_background = true;
        self
    }

    /// Use fixed width mode (convenience method)
    ///
    /// When enabled, the combobox maintains a fixed width regardless of content.
    /// The text and expand button stay in place, and the border remains fixed.
    /// This is ideal when using borders, as it prevents the button from moving.
    ///
    /// When disabled (default), the width adjusts with content, and the text/button
    /// move accordingly. This is ideal for borderless comboboxes.
    ///
    /// # Examples
    ///
    /// ```rust,ignore
    /// // With border - use fixed width
    /// Combobox::new(cx)
    ///     .fixed_width(true)
    ///     .show_border(true)
    ///
    /// // Without border - use dynamic width (default)
    /// Combobox::new(cx)
    ///     .no_border()
    ///     // fixed_width is false by default
    /// ```
    pub fn fixed_width(mut self, fixed: bool) -> Self {
        self.fixed_width = fixed;
        self
    }

    /// Filter options based on input value
    /// Only filters when user is actively typing, not when dropdown is opened after selection
    fn filtered_options(&self) -> Vec<SelectOption> {
        let all_options = if !self.option_groups.is_empty() {
            // Flatten option groups
            self.option_groups
                .iter()
                .flat_map(|group| group.options.iter())
                .cloned()
                .collect()
        } else {
            self.options.clone()
        };

        // Only filter if user is actively typing (not just when input has value from selection)
        if self.is_user_typing && !self.input_value.is_empty() {
            let input_lower = self.input_value.to_lowercase();
            let filtered: Vec<SelectOption> = all_options
                .iter()
                .filter(|opt| {
                    opt.label.to_lowercase().contains(&input_lower) ||
                    opt.value.to_lowercase().contains(&input_lower)
                })
                .cloned()
                .collect();

            // Return filtered results (even if empty - user can see no matches)
            return filtered;
        }

        // No filtering - show all options
        all_options
    }

    /// Filter option groups based on input value
    fn filtered_option_groups(&self) -> Vec<SelectOptionGroup> {
        // Only filter if user is actively typing (not just when input has value from selection)
        if self.is_user_typing && !self.input_value.is_empty() {
            let input_lower = self.input_value.to_lowercase();
            let filtered: Vec<SelectOptionGroup> = self.option_groups
                .iter()
                .map(|group| {
                    let filtered_options: Vec<SelectOption> = group.options
                        .iter()
                        .filter(|opt| {
                            opt.label.to_lowercase().contains(&input_lower) ||
                            opt.value.to_lowercase().contains(&input_lower)
                        })
                        .cloned()
                        .collect();

                    SelectOptionGroup {
                        label: group.label.clone(),
                        options: filtered_options,
                    }
                })
                .filter(|group| !group.options.is_empty()) // Only include groups with matching options
                .collect();

            // Return filtered results (even if empty - user can see no matches)
            return filtered;
        }

        // No filtering - show all option groups
        self.option_groups.clone()
    }

    /// Get all options (flattened from groups if needed) for selection
    fn all_options(&self) -> Vec<SelectOption> {
        if !self.option_groups.is_empty() {
            self.option_groups
                .iter()
                .flat_map(|group| group.options.iter())
                .cloned()
                .collect()
        } else {
            self.options.clone()
        }
    }

    /// Toggle dropdown open/closed
    fn toggle_dropdown(&mut self) {
        if !self.disabled {
            self.is_open = !self.is_open;
            // When opening dropdown via arrow click, only reset typing flag if input is empty
            // If input has value, preserve typing state to maintain filtering
            if self.is_open && self.input_value.is_empty() {
                self.is_user_typing = false;
            }
            // If input has value, keep is_user_typing as is to preserve filtering state
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
    fn select_option(&mut self, value: String, _window: &mut Window, cx: &mut Context<Self>) {
        let all_options = self.all_options();
        if let Some(option) = all_options.iter().find(|opt| opt.value == value) {
            self.selected_value = Some(value.clone());
            self.input_value = option.label.clone();
            // Reset typing flag after selection - when user selects an option, clear typing state
            // This ensures when dropdown reopens, all options are shown (not filtered)
            // But if user then modifies the text, is_user_typing will be set to true again
            self.is_user_typing = false;
            // Close dropdown after selection - user can reopen to select another option
            self.is_open = false;

            // Update TextInput value when option is selected
            if let Some(text_input) = &self.text_input {
                text_input.update(cx, |input, cx| {
                    input.set_value(option.label.clone(), cx);
                });
            }

            // Set flag to blur the input after selection
            self.should_blur = true;

            // Recalculate width after selection to match selected option label
            // This ensures width is correct after selection
            if let Some(text_input) = &self.text_input {
                text_input.update(cx, |_input, _cx| {
                    // Width will be recalculated in next render
                });
            }

            cx.emit(ComboboxEvent::Changed(value));
            cx.notify();
        }
    }

    /// Render the dropdown overlay
    fn render_dropdown_overlay(&self, cx: &Context<Self>) -> impl IntoElement {
        let theme = Theme::default();
        let has_groups = !self.option_groups.is_empty();

        let mut menu = div()
            .occlude()  // Ensure popup content is above other content
            .id("combobox-popup")
            .flex()
            .flex_col()
            .map(|this| match self.dropdown_width {
                DropdownWidth::MatchTrigger => this,
                DropdownWidth::Fixed(width) => this.w(width),
                DropdownWidth::MinWidth(width) => this.min_w(width),
                DropdownWidth::MaxWidth(width) => this.max_w(width).min_w(px(200.)),
            })
            .when(matches!(self.dropdown_width, DropdownWidth::MatchTrigger), |this| {
                // Set a reasonable minimum width to ensure content fits
                this.min_w(px(200.))
            })
            .max_h(px(300.))
            .overflow_y_scroll()
            .overflow_x_hidden()
            // Only round bottom corners when connected to input
            .rounded_bl(px(BorderRadius::LG))
            .rounded_br(px(BorderRadius::LG))
            .border_1()
            .border_color(theme.colors.border)
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
            .pb(px(6.));

        // Render grouped or flat options
        if has_groups {
            let filtered_groups = self.filtered_option_groups();
            let mut item_counter: usize = 0;
            menu = menu.children(filtered_groups.iter().enumerate().map(|(group_idx, group)| {
                div()
                    .flex()
                    .flex_col()
                    .map(|this| {
                        // Add top margin for groups after the first one
                        if group_idx > 0 {
                            this.mt(if self.compact { px(2.) } else { px(4.) })
                        } else {
                            this.mt(px(6.))
                        }
                    })
                    .child({
                        // Group label with clear, bold styling
                        let label_py = if self.compact { px(4.) } else { px(8.) };
                        let label_px = if self.compact { px(8.) } else { px(12.) };

                        div()
                            .px(label_px)
                            .py(label_py)
                            .text_sm()
                            .font_weight(FontWeight::BOLD)
                            .text_color(theme.colors.text)
                            .child(group.label.clone())
                    })
                    .children(group.options.iter().map(|option| {
                        let _id = ("combobox-group-item", item_counter);
                        item_counter += 1;
                        // For grouped items, create a container with full-width background
                        // and overlay content with indentation
                        let value = option.value.clone();
                        let label = option.label.clone();
                        let size = self.size;
                        
                        // Check if this option is selected
                        let is_selected = if let Some(ref selected_value) = self.selected_value {
                            selected_value == &value && self.input_value == label
                        } else {
                            false
                        };
                        
                        // Single item div with background and content
                        div()
                            .w_full()
                            .min_h(px(32.)) // Ensure minimum height for items
                            .mx(px(6.)) // Horizontal margin instead of padding to allow background to span full width
                            .px(if self.compact { px(8.) } else { px(12.) })
                            .py(if self.compact { px(3.) } else { px(8.) })
                            .flex()
                            .items_center()
                            .justify_between()
                            .cursor(CursorStyle::PointingHand)
                            .text_size(size.font_size())
                            .rounded(px(BorderRadius::SM))
                            .map(|this| {
                                if is_selected {
                                    this.bg(theme.colors.primary)
                                        .text_color(rgb(0xFFFFFF))
                                } else {
                                    this.hover(|style| style.bg(theme.colors.background_hover))
                                        .text_color(theme.colors.text)
                                }
                            })
                            .on_mouse_down(MouseButton::Left, cx.listener(move |this, _event: &MouseDownEvent, window, cx| {
                                this.select_option(value.clone(), window, cx);
                            }))
                            .child(label)
                    }))
                    .child(
                        // Separator line below the last option in this group
                        div()
                            .h(px(1.))
                            .bg(theme.colors.border)
                            .mx(px(12.))
                            .mt(if self.compact { px(1.) } else { px(2.) })
                    )
            }));
        } else {
            let filtered_options = self.filtered_options();
            menu = menu.children(filtered_options.iter().enumerate().map(|(idx, option)| {
                div()
                    .when(idx == 0, |this| {
                        this.mt(px(6.))
                    })
                    .child(self.render_option(option, ("combobox-item", idx), &theme, cx))
            }));
        }

        let input_width = self.calculated_width.unwrap_or_else(|| {
            // Fallback: estimate width based on placeholder length
            // Average character width is approximately 8px for 14px font
            let estimated_text_width = self.placeholder.len() as f32 * 8.0;
            (estimated_text_width + 24.0 + 22.0).max(60.0)
        });

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
                DropdownAlignment::Left => {
                    // Align to input text left edge (input container starts at left_0)
                    // For MatchTrigger mode, also set width to match trigger
                    let positioned = this.left_0();
                    if matches!(self.dropdown_width, DropdownWidth::MatchTrigger) {
                        positioned.w(px(input_width))
                    } else {
                        positioned
                    }
                }
                DropdownAlignment::Right => {
                    // Align dropdown right edge to input text right edge
                    match self.dropdown_width {
                        DropdownWidth::MatchTrigger => {
                            // Match input width and align right edge
                            this.left_0().w(px(input_width))
                        }
                        DropdownWidth::Fixed(width) => {
                            // Position dropdown so its right edge aligns with input right edge
                            // Calculate left offset: left = input_width - width
                            // But if width > input_width, we can't align right edge, so we'll align left edge instead
                            let input_width_px = px(input_width);
                            if width <= input_width_px {
                                let left_offset = input_width_px - width;
                                this.left(left_offset).w(width)
                            } else {
                                // If dropdown is wider than input, align left edge to input left edge
                                this.left_0()
                            }
                        }
                        _ => {
                            // For MinWidth/MaxWidth, align left edge to input left edge
                            // Right edge alignment for dynamic widths is complex
                            this.left_0()
                        }
                    }
                }
                DropdownAlignment::Center => {
                    // Center align relative to input text
                    // For center alignment, we'll align left edge for now
                    // Full center alignment would require knowing dropdown width
                    this.left_0()
                }
            })
            .occlude()  // Ensure dropdown is above other content
            .on_mouse_down(MouseButton::Left, cx.listener(|this, _event: &MouseDownEvent, _window, _cx| {
                this.clicking_menu = true;
            }))
            .child(menu)
    }

    /// Render a single option item with full background styling
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

        // Use compact spacing if enabled
        let padding_y = if self.compact { px(3.) } else { px(8.) };
        let padding_x = if self.compact { px(8.) } else { px(12.) };

        // Single item div with background and content
        div()
            .id(id)
            .w_full()
            .min_h(px(32.)) // Ensure minimum height for items
            .mx(px(6.)) // Horizontal margin instead of padding to allow background to span full width
            .px(padding_x)
            .py(padding_y)
            .flex()
            .items_center()
            .justify_between()
            .cursor(CursorStyle::PointingHand)
            .text_size(size.font_size())
            .rounded(px(BorderRadius::SM))
            .map(|this| {
                if is_selected {
                    this.bg(theme.colors.primary)
                        .text_color(rgb(0xFFFFFF))
                } else {
                    this.hover(|style| style.bg(theme.colors.background_hover))
                        .text_color(theme.colors.text)
                }
            })
            .on_mouse_down(MouseButton::Left, cx.listener(move |this, _event: &MouseDownEvent, window, cx| {
                this.select_option(value.clone(), window, cx);
            }))
            .child(label)
    }
}

impl Focusable for Combobox {
    fn focus_handle(&self, _cx: &App) -> FocusHandle {
        self.focus_handle.clone()
    }
}

// ============================================================================
// Render
// ============================================================================

impl Render for Combobox {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        // Handle blur after selection
        if self.should_blur {
            self.should_blur = false;
            // Focus the combobox itself to remove focus from the text input
            cx.focus_self(window);
        }

        let theme = Theme::default();
        let disabled = self.disabled;
        let is_open = self.is_open;
        let text_input = self.text_input.clone();

        // Always recalculate width to ensure it's accurate
        // This handles cases where input_value changes but calculated_width hasn't been updated
        let text_to_measure = if self.input_value.is_empty() {
            &self.placeholder
        } else {
            &self.input_value
        };
        
        // Measure text width using TextRun API
        let font = gpui::Font {
            family: ".SystemUIFont".into(),
            features: Default::default(),
            weight: Default::default(),
            style: Default::default(),
            fallbacks: None,
        };
        let font_size = px(14.);
        let text_runs = vec![gpui::TextRun {
            len: text_to_measure.len(),
            font: font.clone(),
            color: rgb(0x333333).into(),
            background_color: None,
            underline: None,
            strikethrough: None,
        }];
        
        let shaped_line = window.text_system().shape_line(
            text_to_measure.into(),
            font_size,
            &text_runs,
            None,
        );
        // Add padding (left: 12px, right: 2px reduced) + icon width (~20px) + gap
        let min_width = px(60.);
        // In dynamic width mode, reduce the right padding to bring icon closer to text
        let right_space = if self.fixed_width {
            px(17.) // Fixed width: icon + 1px padding - 4px margin = 17px
        } else {
            px(4.) // Dynamic width: minimal space for icon (just padding)
        };
        let calculated_width = shaped_line.width + px(14.) + right_space; // Left padding 12px + right padding 2px = 14px
        // Always update calculated_width to ensure it matches current input
        self.calculated_width = Some(calculated_width.max(min_width).into());

        // Subscribe to TextInput events if not already subscribed
        if let Some(text_input_entity) = &text_input {
            if self._subscriptions.is_empty() {
                let sub = cx.subscribe_in(text_input_entity, window, |this, _input, event: &TextInputEvent, window, cx| {
                    match event {
                        TextInputEvent::Change(value) => {
                            // Check if this is a programmatic change by comparing with selected option
                            let is_programmatic = this.selected_value.is_some() &&
                                this.all_options().iter().any(|opt|
                                    Some(&opt.value) == this.selected_value.as_ref() && &opt.label == value
                                );

                            // Skip processing if this is a programmatic change (from select_option)
                            if is_programmatic {
                                return;
                            }

                            // Update our internal state from user input
                            this.input_value = value.clone();
                            // Always clear selected value when user modifies the text (including deleting)
                            // This ensures that selecting an option and then editing/deleting the text
                            // will clear the selection
                            this.selected_value = None;
                            // Always set typing flag to true when user is typing (even if value is empty after deletion)
                            // This ensures filtering works correctly when user continues typing after selection
                            // Only set to false when input is completely cleared and user hasn't typed anything
                            this.is_user_typing = true; // Always true when user is actively modifying input

                            // Calculate text width for dynamic sizing
                            let text_to_measure = if value.is_empty() {
                                &this.placeholder
                            } else {
                                &value
                            };
                            
                            // Measure text width using TextRun API
                            let font = gpui::Font {
                                family: ".SystemUIFont".into(),
                                features: Default::default(),
                                weight: Default::default(),
                                style: Default::default(),
                                fallbacks: None,
                            };
                            let font_size = px(14.);
                            let text_runs = vec![gpui::TextRun {
                                len: text_to_measure.len(),
                                font: font.clone(),
                                color: rgb(0x333333).into(),
                                background_color: None,
                                underline: None,
                                strikethrough: None,
                            }];
                            
                            let shaped_line = window.text_system().shape_line(
                                text_to_measure.into(),
                                font_size,
                                &text_runs,
                                None,
                            );
                            // Add padding (left: 12px, right: 2px reduced) + icon width (~20px) + gap
                            let min_width = px(60.);
                            // In dynamic width mode, reduce the right padding to bring icon closer to text
                            let right_space = if this.fixed_width {
                                px(17.) // Fixed width: icon + 1px padding - 4px margin = 17px
                            } else {
                                px(4.) // Dynamic width: minimal space for icon (just padding)
                            };
                            let calculated_width = shaped_line.width + px(14.) + right_space; // Left padding 12px + right padding 2px = 14px
                            this.calculated_width = Some(calculated_width.max(min_width).into());

                            // Always keep dropdown open when user is typing (for filtering)
                            if !this.is_open {
                                this.is_open = true;
                            }
                            cx.emit(ComboboxEvent::InputChanged(value.clone()));
                            cx.notify();
                        }
                        TextInputEvent::Focus => {
                            // Auto-open dropdown when focused
                            if !this.disabled {
                                // Don't reset typing flag on focus - preserve user's typing state
                                // This ensures filtering continues to work when user clicks back into input
                                // Only reset if input is completely empty (no text at all)
                                if this.input_value.is_empty() {
                                    this.is_user_typing = false;
                                }
                                // If input has value, keep is_user_typing as is (don't reset)
                                // This allows filtering to work immediately when user clicks back into input

                                if !this.is_open {
                                    this.is_open = true;
                                }
                                cx.notify();
                            }
                        }
                        TextInputEvent::Submit(value) => {
                            // When user presses Enter, check if input matches any option
                            if !value.is_empty() {
                                let all_options = this.all_options();

                                // First, try exact match (case-insensitive)
                                let exact_match = all_options.iter().find(|opt|
                                    opt.label.eq_ignore_ascii_case(value) || opt.value.eq_ignore_ascii_case(value)
                                );

                                if let Some(matched_option) = exact_match {
                                    // Found exact match, select it
                                    this.select_option(matched_option.value.clone(), window, cx);
                                } else {
                                    // No exact match, check if there's only one filtered option
                                    let filtered = all_options.iter().filter(|opt|
                                        opt.label.to_lowercase().contains(&value.to_lowercase()) ||
                                        opt.value.to_lowercase().contains(&value.to_lowercase())
                                    ).collect::<Vec<_>>();

                                    if filtered.len() == 1 {
                                        // Only one match, select it
                                        this.select_option(filtered[0].value.clone(), window, cx);
                                    }
                                    // If multiple matches or no matches, do nothing (keep dropdown open)
                                }
                            }
                        }
                        TextInputEvent::Blur => {
                            // Close dropdown when losing focus (but allow clicking dropdown)
                            // This is handled by clicking_menu flag
                        }
                    }
                });
                self._subscriptions.push(sub);
            }
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
                    .min_w(px(200.)) // 设置最小宽度，避免输入时宽度变化
                    .map(|this| match self.dropdown_width {
                        DropdownWidth::MatchTrigger => this.max_w(px(400.)),
                        DropdownWidth::Fixed(width) => this.max_w(width),
                        DropdownWidth::MinWidth(_width) => this.max_w(px(400.)),
                        DropdownWidth::MaxWidth(width) => this.max_w(width),
                    })
                    .child(
                        div()
                            .id("combobox-trigger")
                            .relative()
                            .flex()
                            .when(self.fixed_width, |this| {
                                // Fixed width mode: trigger takes full width
                                this.w_full()
                            })
                            .when(!self.fixed_width, |this| {
                                // Dynamic width mode: trigger shrinks/grows based on content
                                this.flex_none()
                            })
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
                            .when(!self.transparent_background, |this| {
                                this.bg(theme.colors.background)
                            })
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
                                // Compact layout for combobox: text input with icon close by
                                div()
                                    .flex()
                                    .items_center()
                                    .w_full() // Always take full width of parent
                                    .when(self.fixed_width, |this| {
                                        // Fixed width mode: use space-between to push icon to right
                                        this.justify_between()
                                    })
                                    .when(!self.fixed_width, |this| {
                                        // Dynamic width mode: no gap between text and icon
                                        this.gap_0()
                                    })
                                    .child(
                                        div()
                                            .when(self.fixed_width, |this| {
                                                // Fixed width mode: text container doesn't grow, stays at content size
                                                this.flex_none().overflow_hidden().min_w(px(60.))
                                            })
                                            .when(!self.fixed_width, |this| {
                                                // Dynamic width mode: text container only takes needed space
                                                // Add negative right margin to pull icon closer
                                                this.flex_none().overflow_hidden().min_w(px(60.)).mr(px(-6.))
                                            })
                                            .when(!self.fixed_width, |this| {
                                                // Apply calculated width only in dynamic mode
                                                if let Some(width) = self.calculated_width {
                                                    this.w(px(width))
                                                } else {
                                                    this
                                                }
                                            })
                                            .when_some(text_input.clone(), |this, input| {
                                                // TextInput container - TextInput has no border and transparent background
                                                // The outer combobox-trigger provides the unified border
                                                // Container width adapts to content
                                                // Important: TextInput uses .w_full() internally, but it will be constrained
                                                // by this container's width set above
                                                // TextInput now uses custom right_padding(4.0) for tighter spacing
                                                this.child(
                                                    div()
                                                        .w_full() // Match parent container width
                                                        .h_full()
                                                        .when(!self.transparent_background, |this| {
                                                            this.bg(theme.colors.background)
                                                        })
                                                        .child(input.clone())
                                                )
                                            })
                                    )
                                    .child(
                                        div()
                                            .flex()
                                            .items_center()
                                            .justify_center()
                                            .when(self.fixed_width, |this| {
                                                // Fixed width mode: icon at right with padding
                                                this.px(px(8.))
                                            })
                                            .when(!self.fixed_width, |this| {
                                                // Dynamic width mode: icon close to text (text container has negative margin)
                                                this.px(px(2.))
                                            })
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

