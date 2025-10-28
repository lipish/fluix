use gpui::prelude::FluentBuilder;
use gpui::*;
use crate::theme::*;
use crate::components::basic::icon::{Icon, IconName};

// ============================================================================
// Events
// ============================================================================

/// Events emitted by the Select component
#[derive(Clone, Debug)]
pub enum SelectEvent {
    /// Single selection changed with the selected value
    Changed(String),
    /// Multiple selection changed with all selected values
    MultiChanged(Vec<String>),
}

impl EventEmitter<SelectEvent> for Select {}

// ============================================================================
// Types
// ============================================================================

/// Visual variant of the Select component
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub enum SelectVariant {
    /// Default style with border and background
    #[default]
    Default,
    /// No border, transparent background
    Ghost,
    /// Only border, transparent background
    Outline,
}

/// Direction for dropdown expansion
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub enum DropdownDirection {
    /// Expand downward (default)
    #[default]
    Down,
    /// Expand upward
    Up,
    /// Auto-detect based on available space
    Auto,
}

/// Alignment of dropdown menu relative to trigger
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub enum DropdownAlignment {
    /// Align to left edge (default)
    #[default]
    Left,
    /// Align to right edge
    Right,
    /// Center align
    Center,
}

/// An option in the select dropdown
#[derive(Clone, Debug)]
pub struct SelectOption {
    pub value: String,
    pub label: String,
}

impl SelectOption {
    pub fn new(value: impl Into<String>, label: impl Into<String>) -> Self {
        Self {
            value: value.into(),
            label: label.into(),
        }
    }
}

/// A group of options in the select dropdown
#[derive(Clone, Debug)]
pub struct SelectOptionGroup {
    pub label: String,
    pub options: Vec<SelectOption>,
}

impl SelectOptionGroup {
    pub fn new(label: impl Into<String>, options: Vec<SelectOption>) -> Self {
        Self {
            label: label.into(),
            options,
        }
    }
}

// ============================================================================
// Component
// ============================================================================

/// A select/dropdown component
///
/// # Example
///
/// ```rust,ignore
/// let select = cx.new(|cx| {
///     Select::new(cx)
///         .placeholder("Select an option...")
///         .options(vec![
///             SelectOption::new("react", "React"),
///             SelectOption::new("vue", "Vue"),
///             SelectOption::new("angular", "Angular"),
///         ])
/// });
///
/// cx.subscribe(&select, |this, select, event: &SelectEvent, cx| {
///     match event {
///         SelectEvent::Changed(value) => println!("Selected: {}", value),
///         SelectEvent::MultiChanged(values) => println!("Selected: {:?}", values),
///     }
/// });
/// ```
pub struct Select {
    /// Available options (flat list)
    options: Vec<SelectOption>,
    /// Grouped options
    option_groups: Vec<SelectOptionGroup>,
    /// Currently selected value (single select)
    selected_value: Option<String>,
    /// Currently selected values (multi select)
    selected_values: Vec<String>,
    /// Placeholder text
    placeholder: String,
    /// Whether the dropdown is open
    is_open: bool,
    /// Whether the select is disabled
    disabled: bool,
    /// Size of the select
    size: ComponentSize,
    /// Visual variant
    variant: SelectVariant,
    /// Dropdown expansion direction
    dropdown_direction: DropdownDirection,
    /// Dropdown alignment
    dropdown_alignment: DropdownAlignment,
    /// Custom font size (overrides size.font_size() if set)
    custom_font_size: Option<Pixels>,
    /// Custom background color
    custom_bg_color: Option<Rgba>,
    /// Custom text color
    custom_text_color: Option<Rgba>,
    /// Custom border color
    custom_border_color: Option<Rgba>,
    /// Whether to show border
    show_border: bool,
    /// Whether to show shadow
    show_shadow: bool,
    /// Whether to use compact spacing for dropdown items
    compact: bool,
    /// Whether to allow multiple selection
    multiple: bool,
    /// Flag to prevent closing when clicking inside menu
    clicking_menu: bool,
}

impl Select {
    /// Create a new Select
    pub fn new(_cx: &mut Context<Self>) -> Self {
        Self {
            options: Vec::new(),
            option_groups: Vec::new(),
            selected_value: None,
            selected_values: Vec::new(),
            placeholder: "Select...".to_string(),
            is_open: false,
            disabled: false,
            size: ComponentSize::Medium,
            variant: SelectVariant::Default,
            dropdown_direction: DropdownDirection::Down,
            dropdown_alignment: DropdownAlignment::Left,
            custom_font_size: None,
            custom_bg_color: None,
            custom_text_color: None,
            custom_border_color: None,
            show_border: true,
            show_shadow: true,
            compact: false,
            multiple: false,
            clicking_menu: false,
        }
    }

    /// Set the options
    pub fn options(mut self, options: Vec<SelectOption>) -> Self {
        self.options = options;
        self
    }

    /// Set the option groups
    pub fn option_groups(mut self, groups: Vec<SelectOptionGroup>) -> Self {
        self.option_groups = groups;
        self
    }

    /// Set the placeholder text
    pub fn placeholder(mut self, placeholder: impl Into<String>) -> Self {
        self.placeholder = placeholder.into();
        self
    }

    /// Set the selected value (single select)
    pub fn value(mut self, value: impl Into<String>) -> Self {
        self.selected_value = Some(value.into());
        self
    }

    /// Set the selected values (multi select)
    pub fn values(mut self, values: Vec<String>) -> Self {
        self.selected_values = values;
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

    /// Set custom font size (independent of component size)
    /// This allows you to change the text size without affecting the component height
    pub fn font_size(mut self, size: Pixels) -> Self {
        self.custom_font_size = Some(size);
        self
    }

    /// Set custom background color
    pub fn bg_color(mut self, color: Rgba) -> Self {
        self.custom_bg_color = Some(color);
        self
    }

    /// Set custom text color
    pub fn text_color(mut self, color: Rgba) -> Self {
        self.custom_text_color = Some(color);
        self
    }

    /// Set custom border color
    pub fn border_color(mut self, color: Rgba) -> Self {
        self.custom_border_color = Some(color);
        self
    }

    /// Set visual variant
    pub fn variant(mut self, variant: SelectVariant) -> Self {
        self.variant = variant;
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

    /// Align dropdown to left (convenience method)
    pub fn align_left(mut self) -> Self {
        self.dropdown_alignment = DropdownAlignment::Left;
        self
    }

    /// Align dropdown to right (convenience method)
    pub fn align_right(mut self) -> Self {
        self.dropdown_alignment = DropdownAlignment::Right;
        self
    }

    /// Center align dropdown (convenience method)
    pub fn align_center(mut self) -> Self {
        self.dropdown_alignment = DropdownAlignment::Center;
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

    /// Make background transparent (convenience method)
    pub fn transparent(mut self) -> Self {
        self.custom_bg_color = Some(rgba(0x00000000));
        self
    }

    /// Clean style: no border, no shadow, transparent background (convenience method)
    pub fn clean(mut self) -> Self {
        self.show_border = false;
        self.show_shadow = false;
        self.custom_bg_color = Some(rgba(0x00000000));
        self
    }

    /// Use compact spacing for dropdown items (less padding)
    pub fn compact(mut self) -> Self {
        self.compact = true;
        self
    }

    /// Enable multiple selection
    pub fn multiple(mut self, multiple: bool) -> Self {
        self.multiple = multiple;
        self
    }
    
    /// Get all options (flat list from both options and groups)
    fn all_options(&self) -> Vec<SelectOption> {
        let mut all = self.options.clone();
        for group in &self.option_groups {
            all.extend(group.options.clone());
        }
        all
    }

    /// Get the display text (selected label or placeholder)
    fn display_text(&self) -> String {
        if self.multiple {
            if self.selected_values.is_empty() {
                self.placeholder.clone()
            } else {
                format!("{} selected", self.selected_values.len())
            }
        } else if let Some(value) = &self.selected_value {
            self.all_options()
                .iter()
                .find(|opt| &opt.value == value)
                .map(|opt| opt.label.clone())
                .unwrap_or_else(|| self.placeholder.clone())
        } else {
            self.placeholder.clone()
        }
    }

    /// Toggle dropdown open/closed
    fn toggle_dropdown(&mut self) {
        if !self.disabled {
            self.is_open = !self.is_open;
        }
    }

    /// Close the dropdown
    fn close_dropdown(&mut self, cx: &mut Context<Self>) {
        // Don't close if we're clicking inside the menu in multi-select mode
        if self.clicking_menu && self.multiple {
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
        if self.multiple {
            // Toggle selection in multi-select mode
            if let Some(pos) = self.selected_values.iter().position(|v| v == &value) {
                self.selected_values.remove(pos);
            } else {
                self.selected_values.push(value);
            }
            cx.emit(SelectEvent::MultiChanged(self.selected_values.clone()));
        } else {
            // Single select mode
            self.selected_value = Some(value.clone());
            self.is_open = false;
            cx.emit(SelectEvent::Changed(value));
        }
        cx.notify();
    }

    /// Remove a selected value (for multi-select tags)
    fn remove_value(&mut self, value: String, cx: &mut Context<Self>) {
        if let Some(pos) = self.selected_values.iter().position(|v| v == &value) {
            self.selected_values.remove(pos);
            cx.emit(SelectEvent::MultiChanged(self.selected_values.clone()));
            cx.notify();
        }
    }
    
    /// Render the dropdown overlay (positioning layer)
    /// This layer handles the absolute positioning of the dropdown menu
    fn render_dropdown_overlay(&self, cx: &Context<Self>) -> impl IntoElement {
        let theme = Theme::default();

        div()
            .absolute()
            .map(|this| match self.dropdown_direction {
                DropdownDirection::Down | DropdownDirection::Auto => {
                    this.top_full().mt_1()
                }
                DropdownDirection::Up => {
                    this.bottom_full().mb_1()
                }
            })
            .map(|this| match self.dropdown_alignment {
                DropdownAlignment::Left => {
                    this.left_0()
                }
                DropdownAlignment::Right => {
                    this.right_0()
                }
                DropdownAlignment::Center => {
                    // For center alignment, we'll use left_0 and right_0
                    // and let the menu width determine centering
                    this.left_0().right_0()
                }
            })
            .occlude()
            .on_mouse_down(MouseButton::Left, cx.listener(|this, _event: &MouseDownEvent, _window, _cx| {
                // Mark that we're clicking inside the menu
                this.clicking_menu = true;
            }))
            .child(self.render_dropdown_menu(&theme, cx))
    }
    
    /// Render the dropdown menu (content and styles layer)
    /// This layer handles the visual appearance and content of the dropdown
    fn render_dropdown_menu(&self, theme: &Theme, cx: &Context<Self>) -> impl IntoElement {
        let has_groups = !self.option_groups.is_empty();

        let mut menu = div()
            .occlude()
            .id("select-popup")
            .min_w(px(180.))
            .max_h(px(300.))
            .overflow_y_scroll()
            .rounded(px(BorderRadius::LG))
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
            .p(px(6.));

        // Render grouped or flat options
        if has_groups {
            let mut item_counter: usize = 0;
            menu = menu.children(self.option_groups.iter().enumerate().map(|(_group_idx, group)| {
                div()
                    .flex()
                    .flex_col()
                    .gap_1()
                    .child({
                        // Group label with compact spacing
                        let label_py = if self.compact { px(3.) } else { px(6.) };
                        let label_px = if self.compact { px(8.) } else { px(12.) };

                        div()
                            .px(label_px)
                            .py(label_py)
                            .text_xs()
                            .font_weight(FontWeight::SEMIBOLD)
                            .text_color(theme.colors.text_secondary)
                            .child(group.label.clone())
                    })
                    .children(group.options.iter().map(|option| {
                        let id = ("select-group-item", item_counter);
                        item_counter += 1;
                        self.render_option(option, id, theme, cx)
                    }))
            }));
        } else {
            menu = menu.children(self.options.iter().enumerate().map(|(idx, option)| {
                self.render_option(option, ("select-item", idx), theme, cx)
            }));
        }

        menu
    }

    /// Render a single option item
    fn render_option(&self, option: &SelectOption, id: impl Into<ElementId>, theme: &Theme, cx: &Context<Self>) -> impl IntoElement {
        let value = option.value.clone();
        let label = option.label.clone();
        let multiple = self.multiple;
        let size = self.size;

        let is_selected = if multiple {
            self.selected_values.contains(&value)
        } else {
            self.selected_value.as_ref() == Some(&value)
        };

        // Use compact spacing if enabled
        let padding_y = if self.compact { px(4.) } else { px(8.) };
        let padding_x = if self.compact { px(8.) } else { px(12.) };

        div()
            .id(id)
            .relative()
            .flex()
            .items_center()
            .justify_between()
            .w_full()
            .px(padding_x)
            .py(padding_y)
            .cursor(CursorStyle::PointingHand)
            .text_size(self.custom_font_size.unwrap_or(size.font_size()))
            .rounded(px(BorderRadius::SM))
            .map(|this| {
                if is_selected && !multiple {
                    this.bg(theme.colors.primary)
                        .text_color(rgb(0xFFFFFF))
                } else {
                    this.text_color(self.custom_text_color.unwrap_or(theme.colors.text))
                        .hover(|style| style.bg(theme.colors.background_hover))
                }
            })
            .on_mouse_down(MouseButton::Left, cx.listener(move |this, _event: &MouseDownEvent, _window, cx| {
                this.select_option(value.clone(), cx);
            }))
            .child(
                div()
                    .flex()
                    .items_center()
                    .gap_2()
                    // Checkbox for multi-select
                    .when(multiple, |this| {
                        this.child(self.render_checkbox(is_selected, theme))
                    })
                    .child(label)
            )
            // Show checkmark for single select
            .when(is_selected && !multiple, |this| {
                this.child(
                    div()
                        .text_xs()
                        .child("✓")
                )
            })
    }

    /// Render a checkbox for multi-select
    fn render_checkbox(&self, checked: bool, theme: &Theme) -> impl IntoElement {
        div()
            .flex()
            .items_center()
            .justify_center()
            .w(px(16.))
            .h(px(16.))
            .rounded(px(4.))
            .border_1()
            .border_color(if checked { theme.colors.primary } else { theme.colors.border })
            .bg(if checked { theme.colors.primary } else { rgb(0xFFFFFF) })
            .when(checked, |this| {
                this.child(
                    div()
                        .text_xs()
                        .text_color(rgb(0xFFFFFF))
                        .child("✓")
                )
            })
    }

    /// Render selected tags for multi-select
    fn render_selected_tags(&self, theme: &Theme, cx: &Context<Self>) -> Vec<impl IntoElement> {
        let all_options = self.all_options();

        self.selected_values.iter().map(|value| {
            let label = all_options
                .iter()
                .find(|opt| &opt.value == value)
                .map(|opt| opt.label.clone())
                .unwrap_or_else(|| value.clone());

            let value_for_remove = value.clone();

            div()
                .flex()
                .items_center()
                .gap_1()
                .px(px(8.))
                .py(px(4.))
                .rounded(px(6.))
                .bg(theme.colors.primary)
                .text_color(rgb(0xFFFFFF))
                .text_xs()
                .child(label)
                .child(
                    // Remove button
                    div()
                        .flex()
                        .items_center()
                        .justify_center()
                        .w(px(14.))
                        .h(px(14.))
                        .rounded(px(7.))
                        .cursor(CursorStyle::PointingHand)
                        .hover(|style| style.bg(rgba(0xFFFFFF20)))
                        .on_mouse_down(MouseButton::Left, cx.listener(move |this, _event: &MouseDownEvent, _window, cx| {
                            this.remove_value(value_for_remove.clone(), cx);
                        }))
                        .child(
                            div()
                                .text_xs()
                                .child("×")
                        )
                )
        }).collect()
    }
}

// ============================================================================
// Render
// ============================================================================

impl Render for Select {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = Theme::default();
        let disabled = self.disabled;
        let is_open = self.is_open;
        let multiple = self.multiple;
        let (padding_y, padding_x) = self.size.padding();

        let is_placeholder = if multiple {
            self.selected_values.is_empty()
        } else {
            self.selected_value.is_none()
        };

        div()
            .id("select-wrapper")
            .w_full()
            // Close dropdown when clicking outside (only for single select, multi-select uses backdrop)
            .when(is_open && !multiple, |this| {
                this.on_mouse_down_out(cx.listener(|this, _event: &MouseDownEvent, _window, cx| {
                    this.close_dropdown(cx);
                }))
            })
            .child(
                div()
                    .id("select-container")
                    .relative()
                    .w_full()
                    .child(
                        // Trigger button
                        div()
                            .id("select-trigger")
                            .relative()
                            .flex()
                            .w_full()
                            .items_center()
                            .justify_between()
                            .gap_2()
                            .py(padding_y)
                            .px(padding_x)
                            .rounded(px(BorderRadius::LG))
                            .when(self.show_border && self.variant != SelectVariant::Ghost, |this| {
                                this.border_1()
                                    .border_color(self.custom_border_color.unwrap_or(theme.colors.border))
                            })
                            .map(|this| match self.variant {
                                SelectVariant::Default => {
                                    this.bg(self.custom_bg_color.unwrap_or(theme.colors.background))
                                }
                                SelectVariant::Ghost => {
                                    this.bg(self.custom_bg_color.unwrap_or(rgba(0x00000000)))
                                }
                                SelectVariant::Outline => {
                                    this.bg(self.custom_bg_color.unwrap_or(rgba(0x00000000)))
                                        .border_1()
                                        .border_color(self.custom_border_color.unwrap_or(theme.colors.border))
                                }
                            })
                            .text_size(self.custom_font_size.unwrap_or(self.size.font_size()))
                            .when(self.show_shadow, |this| {
                                this.shadow(vec![BoxShadow {
                                    color: rgba(0x0000000A).into(),
                                    offset: point(px(0.), px(1.)),
                                    blur_radius: px(2.),
                                    spread_radius: px(0.),
                                }])
                            })
                            .when(!disabled, |this| {
                                this.cursor(CursorStyle::PointingHand)
                            })
                            .when(disabled, |this| {
                                this.opacity(0.64)
                            })
                            .on_mouse_down(MouseButton::Left, cx.listener(|this, _event: &MouseDownEvent, _window, cx| {
                                this.toggle_dropdown();
                                cx.notify();
                            }))
                            .child(
                                // Content area
                                div()
                                    .flex()
                                    .flex_1()
                                    .items_center()
                                    .gap_1()
                                    .overflow_hidden()
                                    .when(multiple && !self.selected_values.is_empty(), |this| {
                                        // Show tags for multi-select
                                        this.children(self.render_selected_tags(&theme, cx))
                                    })
                                    .when(!multiple || self.selected_values.is_empty(), |this| {
                                        // Show single value or placeholder
                                        this.child(
                                            div()
                                                .when(is_placeholder, |this| {
                                                    this.text_color(self.custom_text_color.unwrap_or(theme.colors.text_secondary))
                                                })
                                                .when(!is_placeholder, |this| {
                                                    this.text_color(self.custom_text_color.unwrap_or(theme.colors.text))
                                                })
                                                .child(self.display_text())
                                        )
                                    })
                            )
                            .child(
                                // Chevron up/down icon
                                Icon::new(IconName::ChevronUpDown)
                                    .small()
                                    .color(rgb(0x666666))
                            )
                    )
                    // Use deferred() to render dropdown overlay at the end, ensuring it's on top
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
