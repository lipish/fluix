use gpui::prelude::FluentBuilder;
use gpui::*;
use crate::theme::*;
use crate::components::basic::icon::{Icon, IconName};

// ============================================================================
// Events
// ============================================================================

/// Events emitted by the Checkbox component
#[derive(Clone, Debug)]
pub enum CheckboxEvent {
    /// Checkbox state changed
    Changed(bool),
}

impl EventEmitter<CheckboxEvent> for Checkbox {}

// ============================================================================
// Component
// ============================================================================

/// A checkbox component
///
/// # Example
///
/// ```rust,ignore
/// let checkbox = cx.new(|cx| {
///     Checkbox::new(cx)
///         .label("Accept terms")
///         .checked(true)
/// });
///
/// cx.subscribe(&checkbox, |this, checkbox, event: &CheckboxEvent, cx| {
///     match event {
///         CheckboxEvent::Changed(checked) => println!("Checked: {}", checked),
///     }
/// });
/// ```
pub struct Checkbox {
    /// Label text
    label: Option<String>,
    /// Whether the checkbox is checked
    checked: bool,
    /// Whether the checkbox is disabled
    disabled: bool,
    /// Size of the checkbox
    size: ComponentSize,
    /// Custom text color
    custom_text_color: Option<Rgba>,
}

impl Checkbox {
    /// Create a new Checkbox
    pub fn new(_cx: &mut Context<Self>) -> Self {
        Self {
            label: None,
            checked: false,
            disabled: false,
            size: ComponentSize::Medium,
            custom_text_color: None,
        }
    }

    /// Set the label text
    pub fn label(mut self, label: impl Into<String>) -> Self {
        self.label = Some(label.into());
        self
    }

    /// Set the checked state
    pub fn checked(mut self, checked: bool) -> Self {
        self.checked = checked;
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

    /// Set custom text color
    pub fn text_color(mut self, color: Rgba) -> Self {
        self.custom_text_color = Some(color);
        self
    }

    /// Toggle the checkbox state
    fn toggle(&mut self, cx: &mut Context<Self>) {
        if !self.disabled {
            self.checked = !self.checked;
            cx.emit(CheckboxEvent::Changed(self.checked));
            cx.notify();
        }
    }

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

// ============================================================================
// Render
// ============================================================================

impl Render for Checkbox {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = Theme::default();
        let checked = self.checked;
        let disabled = self.disabled;
        let label = self.label.clone();
        let box_size = self.box_size();
        let text_size = self.size.font_size();

        div()
            .id("checkbox")
            .flex()
            .items_center()
            .gap_2()
            .cursor(if disabled { CursorStyle::Arrow } else { CursorStyle::PointingHand })
            .when(disabled, |this| {
                this.opacity(0.64)
            })
            .on_mouse_down(MouseButton::Left, cx.listener(|this, _event: &MouseDownEvent, _window, cx| {
                this.toggle(cx);
            }))
            .child(
                // Checkbox box
                div()
                    .flex()
                    .items_center()
                    .justify_center()
                    .size(box_size)
                    .rounded(px(4.))
                    .border_1()
                    .border_color(if checked {
                        theme.colors.primary
                    } else {
                        theme.colors.border
                    })
                    .bg(if checked {
                        theme.colors.primary
                    } else {
                        rgb(0xFFFFFF)
                    })
                    .when(checked, |this| {
                        this.child(
                            Icon::new(IconName::Check)
                                .size(crate::components::basic::icon::IconSize::Small)
                                .color(rgb(0xFFFFFF))
                        )
                    })
            )
            .when_some(label, |this, label| {
                this.child(
                    div()
                        .text_size(text_size)
                        .text_color(self.custom_text_color.unwrap_or(theme.colors.text))
                        .child(label)
                )
            })
    }
}

