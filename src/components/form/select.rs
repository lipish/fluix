use gpui::prelude::FluentBuilder;
use gpui::*;
use crate::theme::*;

// ============================================================================
// Events
// ============================================================================

/// Events emitted by the Select component
#[derive(Clone, Debug)]
pub enum SelectEvent {
    /// Selection changed with the selected value
    Changed(String),
}

impl EventEmitter<SelectEvent> for Select {}

// ============================================================================
// Types
// ============================================================================

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
///     }
/// });
/// ```
pub struct Select {
    /// Available options
    options: Vec<SelectOption>,
    /// Currently selected value
    selected_value: Option<String>,
    /// Placeholder text
    placeholder: String,
    /// Whether the dropdown is open
    is_open: bool,
    /// Whether the select is disabled
    disabled: bool,
    /// Size of the select
    size: ComponentSize,
}

impl Select {
    /// Create a new Select
    pub fn new(_cx: &mut Context<Self>) -> Self {
        Self {
            options: Vec::new(),
            selected_value: None,
            placeholder: "Select...".to_string(),
            is_open: false,
            disabled: false,
            size: ComponentSize::Medium,
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
    
    /// Get the display text (selected label or placeholder)
    fn display_text(&self) -> String {
        if let Some(value) = &self.selected_value {
            self.options
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
        self.is_open = false;
        cx.notify();
    }

    /// Select an option
    fn select_option(&mut self, value: String, cx: &mut Context<Self>) {
        self.selected_value = Some(value.clone());
        self.is_open = false;
        cx.emit(SelectEvent::Changed(value));
        cx.notify();
    }
    
    /// Render the dropdown overlay (positioning layer)
    /// This layer handles the absolute positioning of the dropdown menu
    fn render_dropdown_overlay(&self, options: Vec<SelectOption>, selected_value: Option<String>, size: ComponentSize, cx: &Context<Self>) -> impl IntoElement {
        let theme = Theme::default();

        div()
            .absolute()
            .top_full()  // Position below the trigger button
            .left_0()
            .right_0()
            .mt_1()  // Small margin from the trigger
            // Close dropdown when clicking outside
            .on_mouse_down_out(cx.listener(|this, _event: &MouseDownEvent, _window, cx| {
                this.close_dropdown(cx);
            }))
            .child(self.render_dropdown_menu(options, selected_value, size, &theme, cx))
    }
    
    /// Render the dropdown menu (content and styles layer)
    /// This layer handles the visual appearance and content of the dropdown
    fn render_dropdown_menu(&self, options: Vec<SelectOption>, selected_value: Option<String>, size: ComponentSize, theme: &Theme, cx: &Context<Self>) -> impl IntoElement {
        div()
            .occlude()
            .id("select-popup")
            .min_w(px(180.))
            .max_h(px(300.))
            .overflow_y_scroll()
            .rounded(px(BorderRadius::LG))
            .border_1()
            .border_color(theme.colors.border)
            .bg(theme.colors.background)
            .shadow(vec![
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
            .p(px(6.))
            // Prevent event bubbling to avoid closing the dropdown when clicking inside
            .on_mouse_down(MouseButton::Left, |_event, _window, _cx| {
                // Stop propagation
            })
            .children(
                options.iter().enumerate().map(|(idx, option)| {
                    let value = option.value.clone();
                    let label = option.label.clone();
                    let is_selected = selected_value.as_ref() == Some(&value);
                    let theme = Theme::default();

                    div()
                        .id(("select-item", idx))
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
                        .child(
                            div()
                                .flex()
                                .items_center()
                                .gap_2()
                                .child(label)
                        )
                        // Show checkmark for selected item
                        .when(is_selected, |this| {
                            this.child(
                                div()
                                    .text_xs()
                                    .child("✓")
                            )
                        })
                })
            )
    }
}

// ============================================================================
// Render
// ============================================================================

impl Render for Select {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = Theme::default();
        let display_text = self.display_text();
        let is_placeholder = self.selected_value.is_none();
        let disabled = self.disabled;
        let is_open = self.is_open;
        let (padding_y, padding_x) = self.size.padding();
        let options = self.options.clone();
        let selected_value = self.selected_value.clone();
        let size = self.size;
        
        div()
            .id("select-wrapper")
            .w_full()
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
                            .border_1()
                            .border_color(theme.colors.border)
                            .bg(theme.colors.background)
                            .text_size(self.size.font_size())
                            .shadow(vec![BoxShadow {
                                color: rgba(0x0000000A).into(),
                                offset: point(px(0.), px(1.)),
                                blur_radius: px(2.),
                                spread_radius: px(0.),
                            }])
                            .when(!disabled, |this| {
                                this.cursor(CursorStyle::PointingHand)
                            })
                            .when(disabled, |this| {
                                this.opacity(0.64)
                            })
                            .when(is_placeholder, |this| {
                                this.text_color(theme.colors.text_secondary)
                            })
                            .when(!is_placeholder, |this| {
                                this.text_color(theme.colors.text)
                            })
                            .on_mouse_down(MouseButton::Left, cx.listener(|this, _event: &MouseDownEvent, _window, cx| {
                                this.toggle_dropdown();
                                cx.notify();
                            }))
                            .child(
                                div()
                                    .flex()
                                    .items_center()
                                    .child(display_text)
                            )
                            .child(
                                // Chevron icon - double arrows (unfold more icon)
                                div()
                                    .flex()
                                    .flex_col()
                                    .items_center()
                                    .justify_center()
                                    .w(px(20.))
                                    .h(px(20.))
                                    .text_color(theme.colors.text_secondary)
                                    .child(
                                        div()
                                            .flex()
                                            .flex_col()
                                            .items_center()
                                            .gap(px(-2.))
                                            .text_xs()
                                            .line_height(relative(0.8))
                                            .child("⌃")  // Up chevron
                                            .child("⌄")  // Down chevron
                                    )
                            )
                    )
            )
            // Use deferred() to render dropdown overlay at the end, ensuring it's on top
            .children(if is_open && !disabled {
                Some(deferred(
                    self.render_dropdown_overlay(options, selected_value, size, cx).into_any_element()
                ))
            } else {
                None
            })
    }
}
