use gpui::prelude::FluentBuilder;
use gpui::*;
use crate::theme::*;
use crate::components::basic::icon::{Icon, IconName};

// ============================================================================
// Events
// ============================================================================

/// Events emitted by the CheckboxGroup component
#[derive(Clone, Debug)]
pub enum CheckboxGroupEvent {
    /// Selected values changed
    Changed(Vec<String>),
}

impl EventEmitter<CheckboxGroupEvent> for CheckboxGroup {}

// ============================================================================
// Types
// ============================================================================

/// An option in the checkbox group
#[derive(Clone, Debug)]
pub struct CheckboxOption {
    pub value: String,
    pub label: String,
}

impl CheckboxOption {
    pub fn new(value: impl Into<String>, label: impl Into<String>) -> Self {
        Self {
            value: value.into(),
            label: label.into(),
        }
    }
}

// ============================================================================
// Component
// ============================================================================

/// A checkbox group component that manages multiple checkboxes
///
/// # Example
///
/// ```rust,ignore
/// let checkbox_group = cx.new(|cx| {
///     CheckboxGroup::new(cx)
///         .options(vec![
///             CheckboxOption::new("apple", "Apple"),
///             CheckboxOption::new("banana", "Banana"),
///             CheckboxOption::new("orange", "Orange"),
///         ])
///         .values(vec!["apple".to_string()])
/// });
///
/// cx.subscribe(&checkbox_group, |this, group, event: &CheckboxGroupEvent, cx| {
///     match event {
///         CheckboxGroupEvent::Changed(values) => println!("Selected: {:?}", values),
///     }
/// });
/// ```
pub struct CheckboxGroup {
    /// Available options
    options: Vec<CheckboxOption>,
    /// Currently selected values
    selected_values: Vec<String>,
    /// Whether the group is disabled
    disabled: bool,
    /// Size of the checkboxes
    size: ComponentSize,
    /// Layout direction
    direction: CheckboxGroupDirection,
    /// Custom text color
    custom_text_color: Option<Rgba>,
}

/// Layout direction for checkbox group
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub enum CheckboxGroupDirection {
    /// Vertical layout (default)
    #[default]
    Vertical,
    /// Horizontal layout
    Horizontal,
}

impl CheckboxGroup {
    /// Create a new CheckboxGroup
    pub fn new(_cx: &mut Context<Self>) -> Self {
        Self {
            options: Vec::new(),
            selected_values: Vec::new(),
            disabled: false,
            size: ComponentSize::Medium,
            direction: CheckboxGroupDirection::Vertical,
            custom_text_color: None,
        }
    }

    /// Set the options
    pub fn options(mut self, options: Vec<CheckboxOption>) -> Self {
        self.options = options;
        self
    }

    /// Set the selected values
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

    /// Set the layout direction
    pub fn direction(mut self, direction: CheckboxGroupDirection) -> Self {
        self.direction = direction;
        self
    }

    /// Set vertical layout (convenience method)
    pub fn vertical(mut self) -> Self {
        self.direction = CheckboxGroupDirection::Vertical;
        self
    }

    /// Set horizontal layout (convenience method)
    pub fn horizontal(mut self) -> Self {
        self.direction = CheckboxGroupDirection::Horizontal;
        self
    }

    /// Set custom text color
    pub fn text_color(mut self, color: Rgba) -> Self {
        self.custom_text_color = Some(color);
        self
    }

    /// Toggle an option
    fn toggle_option(&mut self, value: String, cx: &mut Context<Self>) {
        if self.disabled {
            return;
        }

        if let Some(pos) = self.selected_values.iter().position(|v| v == &value) {
            self.selected_values.remove(pos);
        } else {
            self.selected_values.push(value);
        }

        cx.emit(CheckboxGroupEvent::Changed(self.selected_values.clone()));
        cx.notify();
    }
}

// ============================================================================
// Render
// ============================================================================

impl CheckboxGroup {
    /// Get the checkbox box size
    fn box_size(&self) -> Pixels {
        match self.size {
            ComponentSize::XSmall => px(14.0),
            ComponentSize::Small => px(16.0),
            ComponentSize::Medium => px(18.0),
            ComponentSize::Large => px(20.0),
            ComponentSize::XLarge => px(22.0),
        }
    }
}

impl Render for CheckboxGroup {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = Theme::default();
        let disabled = self.disabled;
        let direction = self.direction;
        let options = self.options.clone();
        let selected_values = self.selected_values.clone();
        let box_size = self.box_size();
        let text_size = self.size.font_size();
        let text_color = self.custom_text_color.unwrap_or(theme.colors.text);

        div()
            .id("checkbox-group")
            .flex()
            .gap_3()
            .when(matches!(direction, CheckboxGroupDirection::Vertical), |this| {
                this.flex_col()
            })
            .when(matches!(direction, CheckboxGroupDirection::Horizontal), |this| {
                this.flex_row()
            })
            .children(options.iter().enumerate().map({
                let theme = theme.clone();
                let disabled = disabled;
                let box_size = box_size;
                let text_size = text_size;
                let text_color = text_color;
                move |(idx, option)| {
                    let value = option.value.clone();
                    let label = option.label.clone();
                    let is_selected = selected_values.contains(&value);

                    div()
                        .id(("checkbox-group-item-wrapper", idx))
                        .on_mouse_down(MouseButton::Left, cx.listener({
                            let value = value.clone();
                            move |this, _event: &MouseDownEvent, _window, cx| {
                                if !this.disabled {
                                    this.toggle_option(value.clone(), cx);
                                }
                            }
                        }))
                        .child(
                            div()
                                .id(("checkbox-group-item", idx))
                                .flex()
                                .items_center()
                                .gap_2()
                                .cursor(if disabled { CursorStyle::Arrow } else { CursorStyle::PointingHand })
                                .when(disabled, |this| {
                                    this.opacity(0.64)
                                })
                                .child(
                                    // Checkbox box
                                    div()
                                        .flex()
                                        .items_center()
                                        .justify_center()
                                        .size(box_size)
                                        .rounded(px(4.))
                                        .border_1()
                                        .border_color(if is_selected {
                                            theme.colors.primary
                                        } else {
                                            theme.colors.border
                                        })
                                        .bg(if is_selected {
                                            theme.colors.primary
                                        } else {
                                            rgb(0xFFFFFF)
                                        })
                                        .when(is_selected, |this| {
                                            this.child(
                                                Icon::new(IconName::Check)
                                                    .size(crate::components::basic::icon::IconSize::Small)
                                                    .color(rgb(0xFFFFFF))
                                            )
                                        })
                                )
                                .child(
                                    div()
                                        .text_size(text_size)
                                        .text_color(text_color)
                                        .child(label)
                                )
                        )
                }
            }))
    }
}

