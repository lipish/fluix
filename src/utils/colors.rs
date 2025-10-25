// Color utility functions

use gpui::*;

/// Convert hex color to Rgba
pub fn hex_to_rgba(hex: u32) -> Rgba {
    rgb(hex)
}

/// Lighten a color by a percentage (0.0 - 1.0)
pub fn lighten(color: Rgba, amount: f32) -> Rgba {
    let amount = amount.clamp(0.0, 1.0);
    Rgba {
        r: color.r + (1.0 - color.r) * amount,
        g: color.g + (1.0 - color.g) * amount,
        b: color.b + (1.0 - color.b) * amount,
        a: color.a,
    }
}

/// Darken a color by a percentage (0.0 - 1.0)
pub fn darken(color: Rgba, amount: f32) -> Rgba {
    let amount = amount.clamp(0.0, 1.0);
    Rgba {
        r: color.r * (1.0 - amount),
        g: color.g * (1.0 - amount),
        b: color.b * (1.0 - amount),
        a: color.a,
    }
}

/// Set alpha channel of a color
pub fn with_alpha(color: Rgba, alpha: f32) -> Rgba {
    Rgba {
        a: alpha.clamp(0.0, 1.0),
        ..color
    }
}

/// Mix two colors
pub fn mix(color1: Rgba, color2: Rgba, weight: f32) -> Rgba {
    let weight = weight.clamp(0.0, 1.0);
    Rgba {
        r: color1.r * weight + color2.r * (1.0 - weight),
        g: color1.g * weight + color2.g * (1.0 - weight),
        b: color1.b * weight + color2.b * (1.0 - weight),
        a: color1.a * weight + color2.a * (1.0 - weight),
    }
}
