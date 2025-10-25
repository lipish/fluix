use gpui::prelude::FluentBuilder;
use gpui::*;
use crate::theme::*;

// ============================================================================
// Events
// ============================================================================

/// Events emitted by the Button component
#[derive(Clone, Debug)]
pub enum ButtonEvent {
    /// Button was clicked
    Click,
}

impl EventEmitter<ButtonEvent> for Button {}

// ============================================================================
// Types
// ============================================================================

/// Button visual variants
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ButtonVariant {
    /// Primary button (filled with primary color)
    Primary,
    /// Secondary button (outlined)
    Secondary,
    /// Outline button (border only)
    Outline,
    /// Text button (no background)
    Text,
    /// Danger button (destructive action)
    Danger,
}

// ============================================================================
// Component
// ============================================================================

/// An interactive button component
/// 
/// # Example
/// 
/// ```rust,ignore
/// let button = cx.new(|cx| {
///     Button::new("Click me")
///         .variant(ButtonVariant::Primary)
///         .size(Size::Medium)
/// });
/// 
/// cx.subscribe(&button, |this, button, event: &ButtonEvent, cx| {
///     match event {
///         ButtonEvent::Click => println!("Button clicked!"),
///     }
/// });
/// ```
pub struct Button {
    /// Button label text
    label: String,
    /// Visual variant
    variant: ButtonVariant,
    /// Size of the button
    size: ComponentSize,
    /// Whether the button is disabled
    disabled: bool,
    /// Whether the button is in loading state
    loading: bool,
    /// Whether the button takes full width
    full_width: bool,
}

impl Button {
    /// Create a new Button with the given label
    pub fn new(label: impl Into<String>) -> Self {
        Self {
            label: label.into(),
            variant: ButtonVariant::Primary,
            size: ComponentSize::Medium,
            disabled: false,
            loading: false,
            full_width: false,
        }
    }
    
    /// Set the button variant
    pub fn variant(mut self, variant: ButtonVariant) -> Self {
        self.variant = variant;
        self
    }
    
    /// Set the button size
    pub fn size(mut self, size: ComponentSize) -> Self {
        self.size = size;
        self
    }
    
    /// Set the disabled state
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }
    
    /// Set the loading state
    pub fn loading(mut self, loading: bool) -> Self {
        self.loading = loading;
        self
    }
    
    /// Set whether the button takes full width
    pub fn full_width(mut self, full_width: bool) -> Self {
        self.full_width = full_width;
        self
    }
    
    /// Get the background color for the current variant
    fn background_color(&self, theme: &Theme, is_hovered: bool) -> Rgba {
        if self.disabled {
            return theme.colors.background_secondary;
        }
        
        match self.variant {
            ButtonVariant::Primary => {
                if is_hovered {
                    theme.colors.primary_hover
                } else {
                    theme.colors.primary
                }
            }
            ButtonVariant::Danger => {
                if is_hovered {
                    crate::utils::lighten(theme.colors.error, 0.1)
                } else {
                    theme.colors.error
                }
            }
            ButtonVariant::Secondary | ButtonVariant::Outline => {
                if is_hovered {
                    theme.colors.background_hover
                } else {
                    rgb(0xFFFFFF)
                }
            }
            ButtonVariant::Text => {
                if is_hovered {
                    theme.colors.background_hover
                } else {
                    rgb(0x00000000) // Transparent
                }
            }
        }
    }
    
    /// Get the text color for the current variant
    fn text_color(&self, theme: &Theme) -> Rgba {
        if self.disabled {
            return theme.colors.text_disabled;
        }
        
        match self.variant {
            ButtonVariant::Primary | ButtonVariant::Danger => rgb(0xFFFFFF),
            ButtonVariant::Secondary | ButtonVariant::Outline | ButtonVariant::Text => {
                theme.colors.text
            }
        }
    }
    
    /// Get the border color for the current variant
    fn border_color(&self, theme: &Theme) -> Option<Rgba> {
        if self.disabled {
            return Some(theme.colors.border);
        }
        
        match self.variant {
            ButtonVariant::Primary | ButtonVariant::Text => None,
            ButtonVariant::Secondary | ButtonVariant::Outline => Some(theme.colors.border),
            ButtonVariant::Danger => None,
        }
    }
}

// ============================================================================
// Render
// ============================================================================

impl Render for Button {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = Theme::default();
        let label = self.label.clone();
        let disabled = self.disabled;
        let loading = self.loading;
        let size = self.size;
        let (padding_y, padding_x) = size.padding();
        
        div()
            .flex()
            .items_center()
            .justify_center()
            .py(padding_y)
            .px(padding_x)
            .rounded(px(BorderRadius::MD))
            .text_size(size.font_size())
            .font_weight(FontWeight::MEDIUM)
            .when(self.full_width, |this| this.w_full())
            .when(!self.full_width, |this| this.flex_shrink_0())
            .when(!disabled && !loading, |this| {
                this.cursor(CursorStyle::PointingHand)
            })
            .when(disabled || loading, |this| {
                this.opacity(0.6)
            })
            .map(|this| {
                let is_hovered = false; // TODO: Track hover state
                this.bg(self.background_color(&theme, is_hovered))
                    .text_color(self.text_color(&theme))
            })
            .when_some(self.border_color(&theme), |this, color| {
                this.border_1().border_color(color)
            })
            .when(!disabled && !loading, |this| {
                this.on_mouse_down(MouseButton::Left, cx.listener(|_this, _, _, cx| {
                    cx.emit(ButtonEvent::Click);
                }))
            })
            .child(
                div()
                    .flex()
                    .items_center()
                    .gap_2()
                    .when(loading, |this| {
                        this.child("⏳") // TODO: Replace with proper loading indicator
                    })
                    .child(label)
            )
    }
}
