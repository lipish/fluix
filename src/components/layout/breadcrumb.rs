use gpui::prelude::FluentBuilder;
use gpui::*;
use crate::theme::*;

// ============================================================================
// Events
// ============================================================================

/// Events emitted by the Breadcrumb component
#[derive(Clone, Debug)]
pub enum BreadcrumbEvent {
    /// Breadcrumb item was clicked
    ItemClicked { index: usize },
}

impl EventEmitter<BreadcrumbEvent> for Breadcrumb {}

// ============================================================================
// Types
// ============================================================================

/// A breadcrumb item
#[derive(Clone, Debug)]
pub struct BreadcrumbItem {
    pub label: String,
    pub value: Option<String>,
}

impl BreadcrumbItem {
    pub fn new(label: impl Into<String>) -> Self {
        Self {
            label: label.into(),
            value: None,
        }
    }

    pub fn with_value(label: impl Into<String>, value: impl Into<String>) -> Self {
        Self {
            label: label.into(),
            value: Some(value.into()),
        }
    }
}

// ============================================================================
// Component
// ============================================================================

/// A breadcrumb navigation component
///
/// # Example
///
/// ```rust,ignore
/// let breadcrumb = cx.new(|cx| {
///     Breadcrumb::new(cx)
///         .items(vec![
///             BreadcrumbItem::new("Home"),
///             BreadcrumbItem::new("Products"),
///             BreadcrumbItem::new("Laptop"),
///         ])
/// });
///
/// cx.subscribe(&breadcrumb, |this, breadcrumb, event: &BreadcrumbEvent, cx| {
///     match event {
///         BreadcrumbEvent::ItemClicked { index } => println!("Clicked: {}", index),
///     }
/// });
/// ```
pub struct Breadcrumb {
    /// List of breadcrumb items
    items: Vec<BreadcrumbItem>,
    /// Separator between items
    separator: BreadcrumbSeparator,
    /// Size of the breadcrumb
    size: ComponentSize,
    /// Custom text color
    custom_text_color: Option<Rgba>,
    /// Custom separator color
    custom_separator_color: Option<Rgba>,
}

/// Separator type for breadcrumb items
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BreadcrumbSeparator {
    /// Slash separator (/)
    Slash,
    /// Chevron right (›)
    Chevron,
    /// Arrow right (→)
    Arrow,
    /// Custom character
    Custom(char),
}

impl Default for BreadcrumbSeparator {
    fn default() -> Self {
        Self::Chevron
    }
}

impl BreadcrumbSeparator {
    fn display(&self) -> String {
        match self {
            Self::Slash => "/".to_string(),
            Self::Chevron => "›".to_string(),
            Self::Arrow => "→".to_string(),
            Self::Custom(ch) => ch.to_string(),
        }
    }
}

impl Breadcrumb {
    /// Create a new Breadcrumb
    pub fn new(_cx: &mut Context<Self>) -> Self {
        Self {
            items: Vec::new(),
            separator: BreadcrumbSeparator::Chevron,
            size: ComponentSize::Medium,
            custom_text_color: None,
            custom_separator_color: None,
        }
    }

    /// Set the breadcrumb items
    pub fn items(mut self, items: Vec<BreadcrumbItem>) -> Self {
        self.items = items;
        self
    }

    /// Set the separator
    pub fn separator(mut self, separator: BreadcrumbSeparator) -> Self {
        self.separator = separator;
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

    /// Set custom separator color
    pub fn separator_color(mut self, color: Rgba) -> Self {
        self.custom_separator_color = Some(color);
        self
    }

    /// Handle item click
    fn handle_item_click(&mut self, index: usize, cx: &mut Context<Self>) {
        cx.emit(BreadcrumbEvent::ItemClicked { index });
        cx.notify();
    }
}

// ============================================================================
// Render
// ============================================================================

impl Render for Breadcrumb {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = Theme::default();
        let items = self.items.clone();
        let separator = self.separator;
        let text_size = self.size.font_size();
        let text_color = self.custom_text_color.unwrap_or(theme.colors.text);
        let separator_color = self.custom_separator_color.unwrap_or(theme.colors.text_secondary);

        div()
            .id("breadcrumb")
            .flex()
            .flex_row()
            .items_center()
            .gap_2()
            .children(items.iter().enumerate().flat_map(|(index, item)| {
                let is_last = index == items.len() - 1;
                let item_label = item.label.clone();
                let text_color = text_color;
                let separator_color = separator_color;
                let mut elements: Vec<AnyElement> = Vec::new();

                // Add item
                elements.push(
                    div()
                        .text_size(text_size)
                        .text_color(if is_last {
                            text_color
                        } else {
                            text_color
                        })
                        .when(!is_last, |this| {
                            this.cursor(CursorStyle::PointingHand)
                                .hover(|style| style.text_color(theme.colors.primary))
                        })
                        .when(is_last, |this| {
                            this.font_weight(FontWeight::MEDIUM)
                        })
                        .on_mouse_down(MouseButton::Left, cx.listener({
                            let index = index;
                            move |this, _event: &MouseDownEvent, _window, cx| {
                                this.handle_item_click(index, cx);
                            }
                        }))
                        .child(item_label)
                        .into_any_element()
                );

                // Add separator if not last item
                if !is_last {
                    elements.push(
                        div()
                            .text_size(text_size)
                            .text_color(separator_color)
                            .px(px(4.))
                            .child(separator.display())
                            .into_any_element()
                    );
                }

                elements.into_iter()
            }))
    }
}

