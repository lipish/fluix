// Theme system for Fluix components

use gpui::*;

/// Component size variants
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ComponentSize {
    XSmall,
    Small,
    Medium,
    Large,
    XLarge,
}

impl ComponentSize {
    /// Get pixel size for the variant
    pub fn px(&self) -> f32 {
        match self {
            ComponentSize::XSmall => 20.0,
            ComponentSize::Small => 28.0,
            ComponentSize::Medium => 36.0,
            ComponentSize::Large => 44.0,
            ComponentSize::XLarge => 52.0,
        }
    }

    /// Get font size for the variant
    pub fn font_size(&self) -> Pixels {
        match self {
            ComponentSize::XSmall => px(11.0),
            ComponentSize::Small => px(13.0),
            ComponentSize::Medium => px(14.0),
            ComponentSize::Large => px(16.0),
            ComponentSize::XLarge => px(18.0),
        }
    }

    /// Get padding for the variant
    pub fn padding(&self) -> (Pixels, Pixels) {
        match self {
            ComponentSize::XSmall => (px(4.0), px(8.0)),
            ComponentSize::Small => (px(6.0), px(12.0)),
            ComponentSize::Medium => (px(8.0), px(16.0)),
            ComponentSize::Large => (px(10.0), px(20.0)),
            ComponentSize::XLarge => (px(12.0), px(24.0)),
        }
    }
}

/// Color palette for the theme
#[derive(Debug, Clone)]
pub struct ColorPalette {
    // Primary colors
    pub primary: Rgba,
    pub primary_hover: Rgba,
    pub primary_active: Rgba,
    
    // Secondary colors
    pub secondary: Rgba,
    pub secondary_hover: Rgba,
    pub secondary_active: Rgba,
    
    // Neutral colors
    pub text: Rgba,
    pub text_secondary: Rgba,
    pub text_disabled: Rgba,
    
    // Background colors
    pub background: Rgba,
    pub background_secondary: Rgba,
    pub background_hover: Rgba,
    
    // Border colors
    pub border: Rgba,
    pub border_focus: Rgba,
    pub border_error: Rgba,
    
    // State colors
    pub success: Rgba,
    pub warning: Rgba,
    pub error: Rgba,
    pub info: Rgba,
}

impl Default for ColorPalette {
    fn default() -> Self {
        Self {
            // Primary colors (紫色系)
            primary: rgb(0x696FC7),
            primary_hover: rgb(0xA7AAE1),
            primary_active: rgb(0x5A5FB0),
            
            // Secondary colors (灰色系)
            secondary: rgb(0x666666),
            secondary_hover: rgb(0x888888),
            secondary_active: rgb(0x555555),
            
            // Neutral colors
            text: rgb(0x333333),
            text_secondary: rgb(0x666666),
            text_disabled: rgb(0x999999),
            
            // Background colors
            background: rgb(0xFFFFFF),
            background_secondary: rgb(0xF5F5F5),
            background_hover: rgb(0xF0F0F0),
            
            // Border colors
            border: rgb(0xE0E0E0),
            border_focus: rgb(0x696FC7),
            border_error: rgb(0xE74C3C),
            
            // State colors
            success: rgb(0x27AE60),
            warning: rgb(0xF39C12),
            error: rgb(0xE74C3C),
            info: rgb(0x3498DB),
        }
    }
}

/// Spacing system
pub struct Spacing;

impl Spacing {
    pub const XXXS: f32 = 2.0;
    pub const XXS: f32 = 4.0;
    pub const XS: f32 = 6.0;
    pub const SM: f32 = 8.0;
    pub const MD: f32 = 12.0;
    pub const LG: f32 = 16.0;
    pub const XL: f32 = 20.0;
    pub const XXL: f32 = 24.0;
    pub const XXXL: f32 = 32.0;
}

/// Border radius values
pub struct BorderRadius;

impl BorderRadius {
    pub const NONE: f32 = 0.0;
    pub const SM: f32 = 4.0;
    pub const MD: f32 = 6.0;
    pub const LG: f32 = 8.0;
    pub const XL: f32 = 12.0;
    pub const FULL: f32 = 9999.0;
}

/// Main theme configuration
#[derive(Debug, Clone)]
pub struct Theme {
    pub colors: ColorPalette,
}

impl Default for Theme {
    fn default() -> Self {
        Self {
            colors: ColorPalette::default(),
        }
    }
}

impl Theme {
    /// Create a new theme
    pub fn new() -> Self {
        Self::default()
    }
    
    /// Create a custom theme with specific colors
    pub fn custom(colors: ColorPalette) -> Self {
        Self { colors }
    }
}
