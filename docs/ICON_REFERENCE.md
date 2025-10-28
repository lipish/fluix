# Icon Reference

Complete reference for all 23 built-in icons in Fluix.

## Features

- ✅ **23 built-in SVG icons**
- ✅ **Background support** (square or rectangular)
- ✅ **5 predefined sizes** (XSmall to XLarge)
- ✅ **Custom sizes** supported
- ✅ **Customizable colors**
- ✅ **Border radius** support
- ✅ **Embedded assets** (no external files needed)

## Quick Start

```rust
use fluix::*;
use gpui::*;

// Basic usage (no background)
Icon::new(IconName::Send)
    .large()
    .color(rgb(0x3B82F6))

// With square background
Icon::new(IconName::Send)
    .medium()
    .color(rgb(0xFFFFFF))
    .with_square_bg(rgb(0x3B82F6))
    .rounded(px(8.))

// With rectangular background
Icon::new(IconName::Send)
    .medium()
    .color(rgb(0x3B82F6))
    .with_rect_bg(px(80.), px(40.), rgb(0xEFF6FF))
    .rounded(px(8.))
```

## Icon Sizes

| Size | Pixels | Method | Use Case |
|------|--------|--------|----------|
| XSmall | 12×12 | `.xsmall()` | Inline text, badges |
| Small | 16×16 | `.small()` | Buttons, inputs |
| Medium | 20×20 | `.medium()` | Default, general use |
| Large | 24×24 | `.large()` | Headers, emphasis |
| XLarge | 32×32 | `.xlarge()` | Hero sections |
| Custom | Any | `.size(IconSize::Custom(n))` | Special cases |

## Icon Backgrounds

Icons can have optional backgrounds:

| Type | Method | Description |
|------|--------|-------------|
| None | Default | No background (icon only) |
| Square | `.with_square_bg(color)` | Square background with padding |
| Rectangle | `.with_rect_bg(w, h, color)` | Custom width and height |

### Background Examples

```rust
// Square background
Icon::new(IconName::Send)
    .medium()
    .color(rgb(0xFFFFFF))
    .with_square_bg(rgb(0x3B82F6))
    .rounded(px(8.))

// Rectangular background
Icon::new(IconName::Send)
    .medium()
    .color(rgb(0x3B82F6))
    .with_rect_bg(px(80.), px(40.), rgb(0xEFF6FF))
    .rounded(px(8.))

// No background (default)
Icon::new(IconName::Send)
    .medium()
    .color(rgb(0x3B82F6))
```

## All Icons

### Navigation Icons (6)

#### ArrowLeft
```rust
Icon::new(IconName::ArrowLeft)
    .medium()
    .color(rgb(0x666666))
```
**Use**: Back buttons, previous navigation

#### ArrowRight
```rust
Icon::new(IconName::ArrowRight)
    .medium()
    .color(rgb(0x666666))
```
**Use**: Forward buttons, next navigation

#### ArrowUp
```rust
Icon::new(IconName::ArrowUp)
    .medium()
    .color(rgb(0x666666))
```
**Use**: Scroll to top, upload

#### ArrowDown
```rust
Icon::new(IconName::ArrowDown)
    .medium()
    .color(rgb(0x666666))
```
**Use**: Scroll down, download

#### ChevronUpDown
```rust
Icon::new(IconName::ChevronUpDown)
    .small()
    .color(rgb(0x999999))
```
**Use**: Select dropdowns, sort indicators

#### UnfoldMore
```rust
Icon::new(IconName::UnfoldMore)
    .small()
    .color(rgb(0x999999))
```
**Use**: Expand/collapse, show more

### Action Icons (6)

#### Check
```rust
Icon::new(IconName::Check)
    .medium()
    .color(rgb(0x10B981))
```
**Use**: Confirm, success, selected state

#### Close
```rust
Icon::new(IconName::Close)
    .small()
    .color(rgb(0x666666))
```
**Use**: Close buttons, dismiss, remove

#### Plus
```rust
Icon::new(IconName::Plus)
    .medium()
    .color(rgb(0x3B82F6))
```
**Use**: Add, create, expand

#### Minus
```rust
Icon::new(IconName::Minus)
    .medium()
    .color(rgb(0x666666))
```
**Use**: Remove, collapse, minimize

#### Search
```rust
Icon::new(IconName::Search)
    .medium()
    .color(rgb(0x666666))
```
**Use**: Search inputs, find functionality

#### Send ⭐ NEW
```rust
Icon::new(IconName::Send)
    .medium()
    .color(rgb(0x3B82F6))
```
**Use**: Submit messages, send emails, post content

### UI Icons (7)

#### Settings
```rust
Icon::new(IconName::Settings)
    .medium()
    .color(rgb(0x666666))
```
**Use**: Settings pages, configuration

#### Home
```rust
Icon::new(IconName::Home)
    .medium()
    .color(rgb(0x666666))
```
**Use**: Home page, dashboard

#### User
```rust
Icon::new(IconName::User)
    .medium()
    .color(rgb(0x666666))
```
**Use**: Profile, account, user menu

#### Bell
```rust
Icon::new(IconName::Bell)
    .medium()
    .color(rgb(0x666666))
```
**Use**: Notifications, alerts

#### Star
```rust
Icon::new(IconName::Star)
    .medium()
    .color(rgb(0xF59E0B))
```
**Use**: Favorites, ratings, bookmarks

#### Heart
```rust
Icon::new(IconName::Heart)
    .medium()
    .color(rgb(0xEF4444))
```
**Use**: Likes, favorites, love

#### Menu
```rust
Icon::new(IconName::Menu)
    .medium()
    .color(rgb(0x666666))
```
**Use**: Hamburger menu, navigation drawer

### Status Icons (4)

#### Info
```rust
Icon::new(IconName::Info)
    .medium()
    .color(rgb(0x3B82F6))
```
**Use**: Information messages, help

#### Warning
```rust
Icon::new(IconName::Warning)
    .medium()
    .color(rgb(0xF59E0B))
```
**Use**: Warning messages, caution

#### Error
```rust
Icon::new(IconName::Error)
    .medium()
    .color(rgb(0xEF4444))
```
**Use**: Error messages, failures

#### Success
```rust
Icon::new(IconName::Success)
    .medium()
    .color(rgb(0x10B981))
```
**Use**: Success messages, confirmations

## Color Palette

Common colors for icons:

```rust
// Neutral
rgb(0x333333)  // Dark gray
rgb(0x666666)  // Medium gray
rgb(0x999999)  // Light gray

// Primary
rgb(0x3B82F6)  // Blue

// Success
rgb(0x10B981)  // Green

// Warning
rgb(0xF59E0B)  // Amber

// Danger
rgb(0xEF4444)  // Red

// Info
rgb(0x3B82F6)  // Blue

// Accent
rgb(0x8B5CF6)  // Purple
```

## Examples

### Icon in Button
```rust
Button::new("Save")
    .variant(ButtonVariant::Primary)
    .icon(IconName::Check)
```

### Icon with Custom Size
```rust
Icon::new(IconName::Send)
    .size(IconSize::Custom(28.0))  // 28×28 pixels (square)
    .color(rgb(0x3B82F6))
```

### Icon Grid
```rust
div()
    .flex()
    .gap_4()
    .child(Icon::new(IconName::Home).large())
    .child(Icon::new(IconName::Search).large())
    .child(Icon::new(IconName::Settings).large())
    .child(Icon::new(IconName::Send).large())  // NEW!
```

## Run Demo

```bash
# Icon demo with all icons
cargo run --example icon_demo

# Send icon demo
cargo run --example icon_send_demo
```

## Technical Details

- **Format**: SVG (Scalable Vector Graphics)
- **Storage**: Embedded in binary via `rust-embed`
- **Rendering**: GPUI's `svg()` component
- **Shape**: Square (width = height) by default
- **Aspect Ratio**: Preserved from original SVG
- **Color**: Applied via `text_color` (uses `currentColor` in SVG)

## Adding Custom Icons

To add your own icons:

1. Add SVG file to `assets/icons/`
2. Add variant to `IconName` enum
3. Add path mapping in `IconName::path()`
4. Rebuild the project

Example:
```rust
// In src/components/basic/icon.rs
pub enum IconName {
    // ... existing icons
    MyCustomIcon,
}

impl IconName {
    pub fn path(self) -> &'static str {
        match self {
            // ... existing paths
            Self::MyCustomIcon => "icons/my-custom-icon.svg",
        }
    }
}
```

## See Also

- [Component Reference](COMPONENT-REFERENCE.md) - All components
- [Examples](../examples/) - Working examples
- [Tutorials](tutorials/README.md) - Step-by-step guides

