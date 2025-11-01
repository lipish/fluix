# Styling and Theming

Learn how to customize the appearance of Fluix components and create consistent designs.

## 🎨 Component Sizing

### Size System

Fluix uses a consistent size system across all components:

```rust
pub enum ComponentSize {
    XSmall,   // 11px font, 20px height
    Small,    // 13px font, 28px height
    Medium,   // 14px font, 36px height (default)
    Large,    // 16px font, 44px height
    XLarge,   // 18px font, 52px height
}
```

### Applying Sizes

```rust
// Button
Button::new("Click").size(ComponentSize::Large)

// Select
Select::new("select").size(ComponentSize::Small)

// All components support the same sizes
```

### Size Comparison

| Size | Font | Height | Padding (Y, X) | Use Case |
|------|------|--------|----------------|----------|
| XSmall | 11px | 20px | 4px, 8px | Compact UIs, toolbars |
| Small | 13px | 28px | 6px, 12px | Dense layouts |
| Medium | 14px | 36px | 8px, 16px | Default, most UIs |
| Large | 16px | 44px | 10px, 20px | Prominent actions |
| XLarge | 18px | 52px | 12px, 24px | Hero sections |

## 🎨 Color System

### Theme Colors

Fluix provides a default color palette:

```rust
use fluix::theme::*;

let theme = Theme::default();

// Primary colors
theme.colors.primary      // #696FC7 (Purple)
theme.colors.secondary    // #6B7280 (Gray)

// Semantic colors
theme.colors.success      // #22C55E (Green)
theme.colors.warning      // #F59E0B (Orange)
theme.colors.error        // #EF4444 (Red)
theme.colors.info         // #3B82F6 (Blue)

// Neutral colors
theme.colors.background   // #FFFFFF (White)
theme.colors.surface      // #F9FAFB (Light Gray)
theme.colors.border       // #E5E7EB (Border Gray)
theme.colors.text         // #111827 (Dark Gray)
theme.colors.text_muted   // #6B7280 (Muted Gray)
```

### Using Colors

```rust
use gpui::*;

// Direct RGB
div().bg(rgb(0x696FC7))
div().text_color(rgb(0x333333))

// With alpha
div().bg(rgba(0x696FC7AA))

// Theme colors
let theme = Theme::default();
div().bg(theme.colors.primary)
```

### Semantic Color Usage

```rust
// Success state
Icon::new(IconName::Success)
    .color(rgb(0x22C55E))

// Error state
Icon::new(IconName::Error)
    .color(rgb(0xEF4444))

// Warning state
Icon::new(IconName::Warning)
    .color(rgb(0xF59E0B))

// Info state
Icon::new(IconName::Info)
    .color(rgb(0x3B82F6))
```

## 📏 Spacing System

### Spacing Constants

```rust
use fluix::theme::Spacing;

Spacing::XXXS  // 2px
Spacing::XXS   // 4px
Spacing::XS    // 6px
Spacing::SM    // 8px
Spacing::MD    // 12px
Spacing::LG    // 16px
Spacing::XL    // 20px
Spacing::XXL   // 24px
Spacing::XXXL  // 32px
```

### Using Spacing

```rust
use gpui::*;
use fluix::theme::Spacing;

div()
    .p(px(Spacing::MD))      // Padding: 12px
    .gap(px(Spacing::SM))    // Gap: 8px
    .m(px(Spacing::LG))      // Margin: 16px
```

### Common Spacing Patterns

```rust
// Card with consistent spacing
div()
    .p(px(Spacing::LG))      // 16px padding
    .gap(px(Spacing::MD))    // 12px gap between children
    .rounded(px(BorderRadius::MD))

// Form layout
div()
    .flex()
    .flex_col()
    .gap(px(Spacing::LG))    // 16px between form fields
    .p(px(Spacing::XL))      // 20px padding
```

## 🔲 Border Radius

### Border Radius Constants

```rust
use fluix::theme::BorderRadius;

BorderRadius::NONE  // 0px
BorderRadius::SM    // 4px
BorderRadius::MD    // 6px
BorderRadius::LG    // 8px
BorderRadius::XL    // 12px
BorderRadius::FULL  // 9999px (fully rounded)
```

### Using Border Radius

```rust
// Rounded corners
div().rounded(px(BorderRadius::MD))

// Fully rounded (pills, circles)
div().rounded(px(BorderRadius::FULL))

// Individual corners
div()
    .rounded_tl(px(BorderRadius::LG))
    .rounded_tr(px(BorderRadius::LG))
```

## 🎭 Component Variants

### Button Variants

```rust
// Primary - Blue background
Button::new("Primary")
    .variant(ButtonVariant::Primary)

// Secondary - Gray background
Button::new("Secondary")
    .variant(ButtonVariant::Secondary)

// Outline - Transparent with border
Button::new("Outline")
    .variant(ButtonVariant::Outline)

// Ghost - Transparent, no border
Button::new("Ghost")
    .variant(ButtonVariant::Ghost)

// Danger - Red background
Button::new("Delete")
    .variant(ButtonVariant::Danger)
```

## 🎨 Custom Styling with GPUI

### Layout

```rust
div()
    .flex()              // Flexbox layout
    .flex_col()          // Column direction
    .items_center()      // Center items
    .justify_between()   // Space between
    .gap_4()             // Gap between children
```

### Sizing

```rust
div()
    .w_full()            // Width: 100%
    .h(px(200.))         // Height: 200px
    .max_w(px(800.))     // Max width: 800px
    .min_h(px(100.))     // Min height: 100px
```

### Spacing

```rust
div()
    .p_4()               // Padding: 16px
    .px_8()              // Padding X: 32px
    .py_2()              // Padding Y: 8px
    .m_4()               // Margin: 16px
```

### Colors

```rust
div()
    .bg(rgb(0xFFFFFF))           // Background
    .text_color(rgb(0x333333))   // Text color
    .border_color(rgb(0xE5E7EB)) // Border color
```

### Borders

```rust
div()
    .border_1()                  // 1px border
    .border_color(rgb(0xE5E7EB))
    .rounded(px(8.))             // Border radius
```

### Shadows

```rust
use gpui::*;

div().shadow(vec![BoxShadow {
    color: rgba(0x0000001A).into(),
    offset: point(px(0.), px(2.)),
    blur_radius: px(4.),
    spread_radius: px(0.),
}])
```

## 🎯 Complete Styling Example

```rust
use fluix::*;
use gpui::*;

fn styled_card() -> impl IntoElement {
    div()
        // Layout
        .flex()
        .flex_col()
        .gap(px(Spacing::MD))
        
        // Sizing
        .w_full()
        .max_w(px(400.))
        .p(px(Spacing::LG))
        
        // Colors
        .bg(rgb(0xFFFFFF))
        .border_1()
        .border_color(rgb(0xE5E7EB))
        
        // Border radius
        .rounded(px(BorderRadius::LG))
        
        // Shadow
        .shadow(vec![BoxShadow {
            color: rgba(0x0000000D).into(),
            offset: point(px(0.), px(1.)),
            blur_radius: px(3.),
            spread_radius: px(0.),
        }])
        
        // Content
        .child(
            div()
                .text_xl()
                .font_weight(FontWeight::BOLD)
                .text_color(rgb(0x111827))
                .child("Card Title")
        )
        .child(
            div()
                .text_sm()
                .text_color(rgb(0x6B7280))
                .child("Card description goes here")
        )
        .child(
            Button::new("Action")
                .variant(ButtonVariant::Primary)
                .size(ComponentSize::Medium)
        )
}
```

## 🎨 Design Patterns

### Card Pattern

```rust
fn card(title: &str, content: impl IntoElement) -> impl IntoElement {
    div()
        .flex()
        .flex_col()
        .gap(px(Spacing::MD))
        .p(px(Spacing::LG))
        .bg(rgb(0xFFFFFF))
        .border_1()
        .border_color(rgb(0xE5E7EB))
        .rounded(px(BorderRadius::LG))
        .child(
            div()
                .text_lg()
                .font_weight(FontWeight::SEMIBOLD)
                .child(title)
        )
        .child(content)
}
```

### Form Field Pattern

```rust
fn form_field(label: &str, input: impl IntoElement) -> impl IntoElement {
    div()
        .flex()
        .flex_col()
        .gap(px(Spacing::XS))
        .child(
            div()
                .text_sm()
                .font_weight(FontWeight::MEDIUM)
                .text_color(rgb(0x374151))
                .child(label)
        )
        .child(input)
}
```

### Button Group Pattern

```rust
fn button_group(buttons: Vec<Entity<Button>>) -> impl IntoElement {
    div()
        .flex()
        .gap(px(Spacing::SM))
        .children(buttons)
}
```

## 💡 Best Practices

### 1. Use Theme Constants

```rust
// ✅ Good - Uses theme constants
div().p(px(Spacing::MD))

// ❌ Avoid - Magic numbers
div().p(px(12.))
```

### 2. Consistent Sizing

```rust
// ✅ Good - Consistent component sizes
Button::new("Save").size(ComponentSize::Large)
Select::new("type").size(ComponentSize::Large)

// ❌ Avoid - Mixing sizes randomly
Button::new("Save").size(ComponentSize::Large)
Select::new("type").size(ComponentSize::Small)
```

### 3. Semantic Colors

```rust
// ✅ Good - Semantic color usage
Icon::new(IconName::Error).color(rgb(0xEF4444))

// ❌ Avoid - Random colors
Icon::new(IconName::Error).color(rgb(0xFF00FF))
```

### 4. Spacing Hierarchy

```rust
// ✅ Good - Clear spacing hierarchy
div()
    .p(px(Spacing::XL))      // Outer padding: 20px
    .gap(px(Spacing::LG))    // Section gap: 16px
    .child(
        div()
            .gap(px(Spacing::SM))  // Item gap: 8px
    )
```

---

**Next**: [Event Handling →](./04-EVENTS.md)

