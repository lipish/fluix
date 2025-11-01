# Tutorial 4: Icon Backgrounds

Learn how to add backgrounds to icons for buttons, badges, and status indicators.

## Table of Contents

- [Introduction](#introduction)
- [Background Types](#background-types)
- [Square Backgrounds](#square-backgrounds)
- [Rectangular Backgrounds](#rectangular-backgrounds)
- [Styling Backgrounds](#styling-backgrounds)
- [Common Use Cases](#common-use-cases)
- [Complete Example](#complete-example)

## Introduction

Icons can have optional backgrounds to make them stand out or fit specific design patterns. Fluix supports three background modes:

- **None** (default) - Icon only, no background
- **Square** - Square background with automatic padding
- **Rectangle** - Custom width and height background

## Background Types

### No Background (Default)

```rust
Icon::new(IconName::Send)
    .medium()
    .color(rgb(0x3B82F6))
```

This is the default behavior - just the icon, no background.

### Square Background

```rust
Icon::new(IconName::Send)
    .medium()
    .color(rgb(0xFFFFFF))
    .with_square_bg(rgb(0x3B82F6))
```

Automatically adds 25% padding around the icon.

### Rectangular Background

```rust
Icon::new(IconName::Send)
    .medium()
    .color(rgb(0x3B82F6))
    .with_rect_bg(px(80.), px(40.), rgb(0xEFF6FF))
```

Custom width and height for the background.

## Square Backgrounds

Square backgrounds are perfect for buttons and circular badges.

### Basic Square Background

```rust
Icon::new(IconName::Check)
    .medium()
    .color(rgb(0xFFFFFF))
    .with_square_bg(rgb(0x10B981))  // Green background
```

### With Border Radius

```rust
Icon::new(IconName::Send)
    .medium()
    .color(rgb(0xFFFFFF))
    .with_square_bg(rgb(0x3B82F6))
    .rounded(px(8.))  // Rounded corners
```

### Different Sizes

```rust
// Small icon button
Icon::new(IconName::Plus)
    .small()
    .color(rgb(0xFFFFFF))
    .with_square_bg(rgb(0x3B82F6))
    .rounded(px(6.))

// Large icon button
Icon::new(IconName::Settings)
    .large()
    .color(rgb(0xFFFFFF))
    .with_square_bg(rgb(0x6B7280))
    .rounded(px(12.))
```

## Rectangular Backgrounds

Rectangular backgrounds are great for badges and labels.

### Basic Rectangular Background

```rust
Icon::new(IconName::Info)
    .small()
    .color(rgb(0x3B82F6))
    .with_rect_bg(px(60.), px(28.), rgb(0xEFF6FF))
    .rounded(px(6.))
```

### Status Badge

```rust
Icon::new(IconName::Success)
    .small()
    .color(rgb(0x10B981))
    .with_rect_bg(px(80.), px(32.), rgb(0xD1FAE5))
    .rounded(px(8.))
```

### Wide Label

```rust
Icon::new(IconName::Warning)
    .medium()
    .color(rgb(0xF59E0B))
    .with_rect_bg(px(120.), px(40.), rgb(0xFEF3C7))
    .rounded(px(10.))
```

## Styling Backgrounds

### Color Combinations

**Primary (Blue)**:
```rust
Icon::new(IconName::Send)
    .medium()
    .color(rgb(0xFFFFFF))           // White icon
    .with_square_bg(rgb(0x3B82F6))  // Blue background
    .rounded(px(8.))
```

**Success (Green)**:
```rust
Icon::new(IconName::Check)
    .medium()
    .color(rgb(0xFFFFFF))           // White icon
    .with_square_bg(rgb(0x10B981))  // Green background
    .rounded(px(8.))
```

**Warning (Amber)**:
```rust
Icon::new(IconName::Warning)
    .medium()
    .color(rgb(0xFFFFFF))           // White icon
    .with_square_bg(rgb(0xF59E0B))  // Amber background
    .rounded(px(8.))
```

**Danger (Red)**:
```rust
Icon::new(IconName::Error)
    .medium()
    .color(rgb(0xFFFFFF))           // White icon
    .with_square_bg(rgb(0xEF4444))  // Red background
    .rounded(px(8.))
```

### Light Backgrounds

```rust
Icon::new(IconName::Info)
    .medium()
    .color(rgb(0x3B82F6))           // Blue icon
    .with_square_bg(rgb(0xEFF6FF))  // Light blue background
    .rounded(px(8.))
```

### Border Radius Variations

```rust
// Slightly rounded
Icon::new(IconName::Star)
    .medium()
    .color(rgb(0xFFFFFF))
    .with_square_bg(rgb(0xF59E0B))
    .rounded(px(4.))

// Medium rounded
Icon::new(IconName::Heart)
    .medium()
    .color(rgb(0xFFFFFF))
    .with_square_bg(rgb(0xEF4444))
    .rounded(px(8.))

// Fully rounded (circle)
Icon::new(IconName::User)
    .medium()
    .color(rgb(0xFFFFFF))
    .with_square_bg(rgb(0x6B7280))
    .rounded(px(999.))  // Large radius = circle
```

## Common Use Cases

### 1. Icon Buttons

```rust
div()
    .flex()
    .gap_2()
    .child(
        Icon::new(IconName::Plus)
            .small()
            .color(rgb(0xFFFFFF))
            .with_square_bg(rgb(0x3B82F6))
            .rounded(px(6.))
    )
    .child(
        Icon::new(IconName::Minus)
            .small()
            .color(rgb(0xFFFFFF))
            .with_square_bg(rgb(0xEF4444))
            .rounded(px(6.))
    )
```

### 2. Status Indicators

```rust
div()
    .flex()
    .flex_col()
    .gap_2()
    .child(
        div()
            .flex()
            .items_center()
            .gap_2()
            .child(
                Icon::new(IconName::Success)
                    .small()
                    .color(rgb(0x10B981))
                    .with_square_bg(rgb(0xD1FAE5))
                    .rounded(px(4.))
            )
            .child("Success")
    )
    .child(
        div()
            .flex()
            .items_center()
            .gap_2()
            .child(
                Icon::new(IconName::Warning)
                    .small()
                    .color(rgb(0xF59E0B))
                    .with_square_bg(rgb(0xFEF3C7))
                    .rounded(px(4.))
            )
            .child("Warning")
    )
```

### 3. Toolbar Icons

```rust
div()
    .flex()
    .gap_2()
    .p_2()
    .bg(rgb(0xF9FAFB))
    .rounded(px(8.))
    .child(
        Icon::new(IconName::Home)
            .small()
            .color(rgb(0x3B82F6))
            .with_square_bg(rgb(0xEFF6FF))
            .rounded(px(6.))
    )
    .child(
        Icon::new(IconName::Search)
            .small()
            .color(rgb(0x666666))
            .with_square_bg(rgb(0xF3F4F6))
            .rounded(px(6.))
    )
    .child(
        Icon::new(IconName::Settings)
            .small()
            .color(rgb(0x666666))
            .with_square_bg(rgb(0xF3F4F6))
            .rounded(px(6.))
    )
```

## Complete Example

Here's a complete example showing various icon backgrounds:

```rust
use fluix::*;
use gpui::*;

struct IconBackgroundDemo {
    scroll_handle: ScrollHandle,
}

impl IconBackgroundDemo {
    fn new(_window: &mut Window, _cx: &mut Context<Self>) -> Self {
        Self {
            scroll_handle: ScrollHandle::new(),
        }
    }
}

impl Render for IconBackgroundDemo {
    fn render(&mut self, _: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .size_full()
            .overflow_hidden()
            .child(
                div()
                    .id("scroll-container")
                    .size_full()
                    .overflow_y_scroll()
                    .track_scroll(&self.scroll_handle)
                    .child(
                        div()
                            .flex()
                            .flex_col()
                            .p_8()
                            .gap_8()
                            .child(
                                // Icon Buttons
                                div()
                                    .flex()
                                    .gap_4()
                                    .child(
                                        Icon::new(IconName::Send)
                                            .medium()
                                            .color(rgb(0xFFFFFF))
                                            .with_square_bg(rgb(0x3B82F6))
                                            .rounded(px(8.))
                                    )
                                    .child(
                                        Icon::new(IconName::Check)
                                            .medium()
                                            .color(rgb(0xFFFFFF))
                                            .with_square_bg(rgb(0x10B981))
                                            .rounded(px(8.))
                                    )
                            )
                    )
            )
    }
}
```

## Next Steps

- Learn about [Select Component Advanced Features](05-SELECT-ADVANCED.md)
- Explore [Component Reference](../COMPONENT-REFERENCE.md)
- Check out [Icon Reference](../ICON_REFERENCE.md)

