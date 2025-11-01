use gpui::prelude::FluentBuilder;
use gpui::*;
use crate::theme::*;

// ============================================================================
// Events
// ============================================================================

/// Events emitted by the RadioGroup component
#[derive(Clone, Debug)]
pub enum RadioGroupEvent {
    /// Selection changed with the selected value
    Changed(String),
}

impl EventEmitter<RadioGroupEvent> for RadioGroup {}

// ============================================================================
// Types
// ============================================================================

/// An option in the radio group
#[derive(Clone, Debug)]
pub struct RadioOption {
    pub value: String,
    pub label: String,
}

impl RadioOption {
    pub fn new(value: impl Into<String>, label: impl Into<String>) -> Self {
        Self {
            value: value.into(),
            label: label.into(),
        }
    }
}

/// Layout direction for radio group
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub enum RadioGroupDirection {
    /// Vertical layout (default)
    #[default]
    Vertical,
    /// Horizontal layout
    Horizontal,
}

// ============================================================================
// Component
// ============================================================================

/// A radio group component for single selection
///
/// # Example
///
/// ```rust,ignore
/// let radio_group = cx.new(|cx| {
///     RadioGroup::new(cx)
///         .options(vec![
///             RadioOption::new("apple", "Apple"),
///             RadioOption::new("banana", "Banana"),
///             RadioOption::new("orange", "Orange"),
///         ])
///         .value("apple")
/// });
///
/// cx.subscribe(&radio_group, |this, group, event: &RadioGroupEvent, cx| {
///     match event {
///         RadioGroupEvent::Changed(value) => println!("Selected: {}", value),
///     }
/// });
/// ```
pub struct RadioGroup {
    /// Available options
    options: Vec<RadioOption>,
    /// Currently selected value
    selected_value: Option<String>,
    /// Whether the group is disabled
    disabled: bool,
    /// Size of the radio buttons
    size: ComponentSize,
    /// Layout direction
    direction: RadioGroupDirection,
    /// Custom text color
    custom_text_color: Option<Rgba>,
}

impl RadioGroup {
    /// Create a new RadioGroup
    pub fn new(_cx: &mut Context<Self>) -> Self {
        Self {
            options: Vec::new(),
            selected_value: None,
            disabled: false,
            size: ComponentSize::Medium,
            direction: RadioGroupDirection::Vertical,
            custom_text_color: None,
        }
    }

    /// Set the options
    pub fn options(mut self, options: Vec<RadioOption>) -> Self {
        self.options = options;
        self
    }

    /// Set the selected value
    pub fn value(mut self, value: impl Into<String>) -> Self {
        self.selected_value = Some(value.into());
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
    pub fn direction(mut self, direction: RadioGroupDirection) -> Self {
        self.direction = direction;
        self
    }

    /// Set vertical layout (convenience method)
    pub fn vertical(mut self) -> Self {
        self.direction = RadioGroupDirection::Vertical;
        self
    }

    /// Set horizontal layout (convenience method)
    pub fn horizontal(mut self) -> Self {
        self.direction = RadioGroupDirection::Horizontal;
        self
    }

    /// Set custom text color
    pub fn text_color(mut self, color: Rgba) -> Self {
        self.custom_text_color = Some(color);
        self
    }

    /// Select an option
    fn select_option(&mut self, value: String, cx: &mut Context<Self>) {
        if self.disabled {
            return;
        }

        self.selected_value = Some(value.clone());
        cx.emit(RadioGroupEvent::Changed(value));
        cx.notify();
    }

    /// Check if an option is selected
    fn is_selected(&self, value: &str) -> bool {
        self.selected_value.as_ref() == Some(&value.to_string())
    }

    /// Get the radio button size
    fn radio_size(&self) -> f32 {
        match self.size {
            ComponentSize::XSmall => 14.0,
            ComponentSize::Small => 16.0,
            ComponentSize::Medium => 18.0,
            ComponentSize::Large => 20.0,
            ComponentSize::XLarge => 22.0,
        }
    }

    /// Render a single radio button
    fn render_radio(&self, option: &RadioOption, index: usize, theme: &Theme) -> impl IntoElement {
        let value = option.value.clone();
        let label = option.label.clone();
        let is_selected = self.is_selected(&value);
        let disabled = self.disabled;
        let radio_size_val = self.radio_size();
        let radio_size = px(radio_size_val);
        let text_size = self.size.font_size();
        let text_color = self.custom_text_color.unwrap_or(theme.colors.text);

        div()
            .id(("radio-item", index))
            .flex()
            .items_center()
            .gap_2()
            .cursor(if disabled { CursorStyle::Arrow } else { CursorStyle::PointingHand })
            .when(disabled, |this| {
                this.opacity(0.64)
            })
            .child(
                // Radio button circle
                div()
                    .flex()
                    .items_center()
                    .justify_center()
                    .size(radio_size)
                    .rounded(px(radio_size_val / 2.0)) // Full circle
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
                        // Inner dot
                        this.child(
                            div()
                                .size(px(radio_size_val * 0.5))
                                .rounded(px(radio_size_val * 0.25))
                                .bg(rgb(0xFFFFFF))
                        )
                    })
            )
            .child(
                div()
                    .text_size(text_size)
                    .text_color(text_color)
                    .child(label)
            )
    }
}

// ============================================================================
// Render
// ============================================================================

impl Render for RadioGroup {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = Theme::default();
        let options = self.options.clone();
        let direction = self.direction;

        div()
            .id("radio-group")
            .flex()
            .gap_3()
            .when(matches!(direction, RadioGroupDirection::Vertical), |this| {
                this.flex_col()
            })
            .when(matches!(direction, RadioGroupDirection::Horizontal), |this| {
                this.flex_row()
            })
            .children(options.iter().enumerate().map({
                let theme = theme.clone();
                move |(idx, option)| {
                    let value = option.value.clone();
                    let option_clone = option.clone();

                    div()
                        .id(("radio-group-item", idx))
                        .on_mouse_down(MouseButton::Left, cx.listener({
                            let value = value.clone();
                            move |this, _event: &MouseDownEvent, _window, cx| {
                                if !this.disabled {
                                    this.select_option(value.clone(), cx);
                                }
                            }
                        }))
                        .child(self.render_radio(&option_clone, idx, &theme))
                }
            }))
    }
}

