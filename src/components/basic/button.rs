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
    fn background_color(&self, theme: &Theme, is_hovered: bool, is_pressed: bool) -> Rgba {
        if self.disabled {
            return theme.colors.background_secondary;
        }
        
        match self.variant {
            ButtonVariant::Primary => {
                if is_pressed {
                    crate::utils::darken(theme.colors.primary, 0.1)
                } else if is_hovered {
                    crate::utils::lighten(theme.colors.primary, 0.1)
                } else {
                    theme.colors.primary
                }
            }
            ButtonVariant::Danger => {
                if is_pressed {
                    crate::utils::darken(theme.colors.error, 0.1)
                } else if is_hovered {
                    crate::utils::lighten(theme.colors.error, 0.1)
                } else {
                    theme.colors.error
                }
            }
            ButtonVariant::Secondary => {
                if is_pressed {
                    crate::utils::darken(theme.colors.background_hover, 0.05)
                } else if is_hovered {
                    theme.colors.background_hover
                } else {
                    theme.colors.background
                }
            }
            ButtonVariant::Outline => {
                if is_pressed || is_hovered {
                    rgba(0x00000005)
                } else {
                    rgba(0x00000000)  // Transparent, not black
                }
            }
            ButtonVariant::Text => {
                if is_pressed || is_hovered {
                    rgba(0x00000008)
                } else {
                    rgba(0x00000000)  // Transparent, not black
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
            ButtonVariant::Primary => {
                // Primary buttons always use white text for better contrast
                rgb(0xFFFFFF)
            }
            ButtonVariant::Danger => {
                // Danger buttons always use white text
                rgb(0xFFFFFF)
            }
            ButtonVariant::Secondary => {
                // Secondary buttons check background brightness
                let bg = self.background_color(theme, false, false);
                if self.is_dark_background(bg) {
                    rgb(0xFFFFFF)
                } else {
                    theme.colors.text
                }
            }
            ButtonVariant::Outline | ButtonVariant::Text => {
                theme.colors.text
            }
        }
    }
    
    /// Check if a color is dark (needs light text)
    fn is_dark_background(&self, color: Rgba) -> bool {
        // Using perceived luminance formula
        let luminance = 0.299 * color.r + 0.587 * color.g + 0.114 * color.b;
        luminance < 0.6  // Increased threshold for better readability
    }
    
    /// Get the border color for the current variant
    fn border_color(&self, theme: &Theme) -> Option<Rgba> {
        if self.disabled {
            return Some(theme.colors.border);
        }
        
        match self.variant {
            ButtonVariant::Primary | ButtonVariant::Danger | ButtonVariant::Text => None,
            ButtonVariant::Secondary | ButtonVariant::Outline => Some(theme.colors.border),
        }
    }

    /// Get shadow style for the button
    fn shadow_style(&self) -> Option<BoxShadow> {
        if self.disabled {
            return None;
        }

        match self.variant {
            ButtonVariant::Primary => Some(BoxShadow {
                color: rgba(0x00000018).into(),
                offset: point(px(0.), px(1.)),
                blur_radius: px(2.),
                spread_radius: px(0.),
            }),
            ButtonVariant::Secondary => Some(BoxShadow {
                color: rgba(0x0000000A).into(),
                offset: point(px(0.), px(1.)),
                blur_radius: px(2.),
                spread_radius: px(0.),
            }),
            _ => None,
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
        let min_width = size.min_width();
        let text_color = self.text_color(&theme);
        let bg_color = self.background_color(&theme, false, false);
        
        div()
            .id("button")
            .relative()
            .flex()
            .flex_shrink_0()
            .items_center()
            .justify_center()
            .gap_2()
            .py(padding_y)
            .px(padding_x)
            .min_w(min_width)  // Apply standard minimum width
            .rounded(px(BorderRadius::LG))
            .text_size(size.font_size())
            .font_weight(FontWeight::MEDIUM)
            .bg(bg_color)
            .when(self.full_width, |this| this.w_full())
            .when(!disabled && !loading, |this| {
                this.cursor(CursorStyle::PointingHand)
            })
            .when(disabled || loading, |this| {
                this.opacity(0.64)
            })
            .when_some(self.border_color(&theme), |this, color| {
                this.border_1().border_color(color)
            })
            .when_some(self.shadow_style(), |this, shadow| {
                this.shadow(vec![shadow])
            })
            .on_mouse_down(MouseButton::Left, cx.listener(|this, _event: &MouseDownEvent, _window, cx| {
                if !this.disabled && !this.loading {
                    cx.emit(ButtonEvent::Click);
                }
            }))
            .text_color(text_color)
            .when(loading, |this| {
                this.child("⏳ ")
            })
            .child(label)
    }
}
