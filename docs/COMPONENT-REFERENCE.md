# Component Reference

Complete API reference for all Fluix components.

## Table of Contents

- [Button](#button)
- [Icon](#icon)
- [Select](#select)
- [TextInput](#textinput)
- [Checkbox](#checkbox)
- [Common Types](#common-types)

---

## Button

Interactive button component with multiple variants and sizes.

### Constructor

```rust
Button::new(label: impl Into<SharedString>) -> Self
```

### Methods

| Method | Type | Description |
|--------|------|-------------|
| `.variant()` | `ButtonVariant` | Set button variant (Primary, Secondary, etc.) |
| `.size()` | `ComponentSize` | Set button size (XSmall to XLarge) |
| `.disabled()` | `bool` | Enable/disable the button |
| `.loading()` | `bool` | Show loading state |
| `.full_width()` | `bool` | Make button full width |
| `.icon()` | `IconName` | Add an icon to the button |

### Variants

```rust
pub enum ButtonVariant {
    Primary,    // Blue background, white text
    Secondary,  // Gray background, dark text
    Outline,    // Transparent with border
    Ghost,      // Transparent, no border
    Danger,     // Red background, white text
}
```

### Events

```rust
pub enum ButtonEvent {
    Click,
}
```

### Example

```rust
let button = cx.new(|_| {
    Button::new("Save")
        .variant(ButtonVariant::Primary)
        .size(ComponentSize::Large)
        .icon(IconName::Check)
});

cx.subscribe_in(&button, window, |_, _, event, _, _| {
    match event {
        ButtonEvent::Click => println!("Clicked!"),
    }
}).detach();
```

---

## Icon

SVG icon component with 23 built-in icons. **Icons support square and rectangular backgrounds**.

### Constructor

```rust
Icon::new(name: IconName) -> Self
```

### Methods

| Method | Type | Description |
|--------|------|-------------|
| `.xsmall()` | - | Set size to 12px |
| `.small()` | - | Set size to 16px |
| `.medium()` | - | Set size to 20px (default) |
| `.large()` | - | Set size to 24px |
| `.xlarge()` | - | Set size to 32px |
| `.size()` | `IconSize` | Set custom size |
| `.color()` | `Rgba` | Set icon color |
| `.with_square_bg()` | `Rgba` | Add square background ⭐ NEW |
| `.with_rect_bg()` | `Pixels, Pixels, Rgba` | Add rectangular background ⭐ NEW |
| `.bg_color()` | `Rgba` | Set background color ⭐ NEW |
| `.rounded()` | `Pixels` | Set border radius ⭐ NEW |

### Icon Names

**Navigation** (6):
- `ArrowLeft`, `ArrowRight`, `ArrowUp`, `ArrowDown`
- `ChevronUpDown`, `UnfoldMore`

**Actions** (6):
- `Check`, `Close`, `Plus`, `Minus`, `Search`, `Send` ⭐ NEW

**UI** (7):
- `Settings`, `Home`, `User`, `Bell`
- `Star`, `Heart`, `Menu`

**Status** (4):
- `Info`, `Warning`, `Error`, `Success`

### Example

```rust
// Basic icon (no background)
Icon::new(IconName::Star)
    .large()
    .color(rgb(0xF59E0B))

// Send icon with square background (NEW!)
Icon::new(IconName::Send)
    .medium()
    .color(rgb(0xFFFFFF))
    .with_square_bg(rgb(0x3B82F6))
    .rounded(px(8.))

// Icon with rectangular background (NEW!)
Icon::new(IconName::Send)
    .medium()
    .color(rgb(0x3B82F6))
    .with_rect_bg(px(80.), px(40.), rgb(0xEFF6FF))
    .rounded(px(8.))
```

---

## Select

Dropdown selection component with single/multiple selection support.

### Constructor

```rust
Select::new(id: impl Into<SharedString>) -> Self
```

### Methods

| Method | Type | Description |
|--------|------|-------------|
| `.placeholder()` | `impl Into<SharedString>` | Set placeholder text |
| `.size()` | `ComponentSize` | Set component size |
| `.font_size()` | `Pixels` | Set custom font size (independent of component size) |
| `.bg_color()` | `Rgba` | Set custom background color |
| `.text_color()` | `Rgba` | Set custom text color |
| `.border_color()` | `Rgba` | Set custom border color ⭐ NEW |
| `.variant()` | `SelectVariant` | Set visual variant (Default, Ghost, Outline) |
| `.dropdown_direction()` | `DropdownDirection` | Set dropdown direction (Down, Up, Auto) |
| `.dropdown_alignment()` | `DropdownAlignment` | Set dropdown alignment (Left, Right, Center) |
| `.align_left()` | - | Align dropdown to left (convenience method) |
| `.align_right()` | - | Align dropdown to right (convenience method) |
| `.align_center()` | - | Center align dropdown (convenience method) |
| `.dropdown_width()` | `DropdownWidth` | Set dropdown width ⭐ NEW |
| `.fixed_width()` | `Pixels` | Set fixed dropdown width (convenience method) ⭐ NEW |
| `.min_width()` | `Pixels` | Set minimum dropdown width (convenience method) ⭐ NEW |
| `.max_width()` | `Pixels` | Set maximum dropdown width (convenience method) ⭐ NEW |
| `.no_border()` | - | Remove border (convenience method) |
| `.no_shadow()` | - | Remove shadow (convenience method) |
| `.transparent()` | - | Make background transparent (convenience method) |
| `.clean()` | - | Clean style: no border, no shadow, transparent (convenience method) |
| `.compact()` | - | Use compact spacing for dropdown items (convenience method) ⭐ NEW |
| `.multiple()` | `bool` | Enable multiple selection |
| `.options()` | `Vec<SelectOption>` | Set options |
| `.option_groups()` | `Vec<SelectOptionGroup>` | Set grouped options |
| `.value()` | `impl Into<String>` | Set selected value (single) |
| `.values()` | `Vec<String>` | Set selected values (multiple) |
| `.disabled()` | `bool` | Enable/disable the select |

### Types

```rust
pub struct SelectOption {
    pub value: String,
    pub label: String,
}

impl SelectOption {
    pub fn new(value: impl Into<String>, label: impl Into<String>) -> Self
}

pub struct SelectOptionGroup {
    pub label: String,
    pub options: Vec<SelectOption>,
}

impl SelectOptionGroup {
    pub fn new(label: impl Into<String>) -> Self
    pub fn option(mut self, option: SelectOption) -> Self
}
```

### Events

```rust
pub enum SelectEvent {
    Change(String),           // Single selection
    MultiChange(Vec<String>), // Multiple selection
}
```

### Example

```rust
// Single selection
let select = cx.new(|_| {
    Select::new("framework")
        .placeholder("Choose a framework")
        .options(vec![
            SelectOption::new("react", "React"),
            SelectOption::new("vue", "Vue"),
        ])
});

// Multiple selection
let multi_select = cx.new(|_| {
    Select::new("languages")
        .placeholder("Select languages")
        .multiple(true)
        .options(vec![
            SelectOption::new("rust", "Rust"),
            SelectOption::new("go", "Go"),
        ])
});

// Grouped options
let grouped_select = cx.new(|cx| {
    Select::new(cx)
        .option_groups(vec![
            SelectOptionGroup::new("Frontend")
                .option(SelectOption::new("react", "React"))
                .option(SelectOption::new("vue", "Vue")),
            SelectOptionGroup::new("Backend")
                .option(SelectOption::new("rust", "Rust"))
                .option(SelectOption::new("go", "Go")),
        ])
});

// Custom font size (NEW!)
// Change font size without changing component height
let custom_font_select = cx.new(|cx| {
    Select::new(cx)
        .placeholder("Select option")
        .font_size(px(12.))  // 12px font, but keeps 36px height
        .options(vec![...])
});

// Combine size and custom font
let large_with_small_font = cx.new(|cx| {
    Select::new(cx)
        .size(ComponentSize::Large)  // 44px height
        .font_size(px(12.))           // But 12px font
        .options(vec![...])
});

// Custom background color (NEW!)
let blue_bg_select = cx.new(|cx| {
    Select::new(cx)
        .placeholder("Select option")
        .bg_color(rgb(0xEFF6FF))  // Light blue background
        .options(vec![...])
});

// Combine all customizations
let fully_custom_select = cx.new(|cx| {
    Select::new(cx)
        .placeholder("Fully customized")
        .size(ComponentSize::Large)      // Custom size
        .font_size(px(12.))               // Custom font size
        .bg_color(rgb(0xF0FDF4))          // Light green background
        .options(vec![...])
});

// Dark theme with custom text color
let dark_theme_select = cx.new(|cx| {
    Select::new(cx)
        .placeholder("Dark theme")
        .bg_color(rgb(0x1F2937))          // Dark gray background
        .text_color(rgb(0xFFFFFF))        // White text
        .options(vec![...])
});

// Ghost variant (NEW!)
let ghost_select = cx.new(|cx| {
    Select::new(cx)
        .placeholder("Ghost variant")
        .variant(SelectVariant::Ghost)    // No border, transparent
        .text_color(rgb(0x999999))
        .options(vec![...])
});

// Outline variant (NEW!)
let outline_select = cx.new(|cx| {
    Select::new(cx)
        .placeholder("Outline variant")
        .variant(SelectVariant::Outline)  // Border only
        .options(vec![...])
});

// Dropdown expands upward (NEW!)
let up_select = cx.new(|cx| {
    Select::new(cx)
        .placeholder("Expands upward")
        .dropdown_direction(DropdownDirection::Up)
        .options(vec![...])
});

// Convenience methods (NEW!)
let simple_select = cx.new(|cx| {
    Select::new(cx)
        .no_border()                      // Remove border
        .no_shadow()                      // Remove shadow
        .transparent()                    // Transparent background
        .options(vec![...])
});

// Clean style - all-in-one
let clean_select = cx.new(|cx| {
    Select::new(cx)
        .clean()                          // No border, no shadow, transparent
        .text_color(rgb(0x999999))
        .options(vec![...])
});

// Compact spacing - less padding
let compact_select = cx.new(|cx| {
    Select::new(cx)
        .compact()                        // Compact spacing for dropdown items
        .options(vec![...])
});

// Right aligned dropdown (NEW!)
let right_aligned = cx.new(|cx| {
    Select::new(cx)
        .align_right()                    // Align dropdown to right edge
        .options(vec![...])
});

// Center aligned + Up direction
let center_up = cx.new(|cx| {
    Select::new(cx)
        .align_center()                   // Center align
        .dropdown_direction(DropdownDirection::Up)
        .options(vec![...])
});

// Fixed width dropdown (NEW!)
let narrow_select = cx.new(|cx| {
    Select::new(cx)
        .fixed_width(px(120.))            // Fixed 120px width
        .compact()
        .align_right()
        .options(vec![...])
});

// Min/Max width (NEW!)
let constrained_select = cx.new(|cx| {
    Select::new(cx)
        .min_width(px(200.))              // At least 200px
        .max_width(px(400.))              // At most 400px
        .options(vec![...])
});
```

---

## TextInput

A powerful single-line text input component with full editing capabilities.

### Constructor

```rust
TextInput::new(cx: &mut Context<Self>) -> Self
```

### Methods

| Method | Type | Description |
|--------|------|-------------|
| `.placeholder()` | `impl Into<SharedString>` | Set placeholder text |
| `.value()` | `impl Into<String>` | Set initial value |
| `.password()` | `bool` | Enable password mode (masked) |
| `.disabled()` | `bool` | Enable/disable the input |
| `.max_length()` | `usize` | Set maximum length |
| `.validator()` | `fn(&str) -> bool` | Set validation function |

### Features

**✨ IME Support** - Full support for Chinese, Japanese, Korean input with proper multi-byte character handling

**🖱️ Mouse Selection** - Click to position cursor, drag to select, Shift+Click to extend

**⌨️ Keyboard Shortcuts**
- `Cmd+A` / `Ctrl+A` - Select all
- `Shift+Arrow` - Extend selection
- `Shift+Home/End` - Extend to start/end
- `Backspace` / `Delete` - Delete character or selection

**🎨 Visual Features**
- Cursor blinking animation
- Smooth selection highlighting
- No width jitter during selection

### Events

```rust
pub enum TextInputEvent {
    Change(String),  // Value changed
    Submit(String),  // Enter key pressed
    Focus,           // Input focused
    Blur,            // Input blurred
}
```

### Example

```rust
let input = cx.new(|cx| {
    TextInput::new(cx)
        .placeholder("Enter your email")
        .validator(|value| value.contains('@'))
        .max_length(100)
});

cx.subscribe_in(&input, window, |_, _, event, _, _| {
    match event {
        TextInputEvent::Change(value) => {
            println!("Value: {}", value);
        }
        TextInputEvent::Submit(value) => {
            println!("Submitted: {}", value);
        }
        _ => {}
    }
}).detach();
```

---

## TextArea

A multi-line text area component with full editing capabilities.

### Constructor

```rust
TextArea::new(cx: &mut Context<Self>) -> Self
```

### Methods

| Method | Type | Description |
|--------|------|-------------|
| `.placeholder()` | `impl Into<SharedString>` | Set placeholder text |
| `.value()` | `impl Into<String>` | Set initial value |
| `.disabled()` | `bool` | Enable/disable the textarea |
| `.max_length()` | `usize` | Set maximum length |
| `.min_height()` | `f32` | Set minimum height in pixels |
| `.max_height()` | `f32` | Set maximum height in pixels |
| `.bg_color()` | `Rgba` | Set custom background color |
| `.border_color()` | `Rgba` | Set custom border color |
| `.focus_border_color()` | `Rgba` | Set custom focus border color |
| `.no_border()` | - | Remove border |

### Features

**🖱️ Mouse Selection** - Click to position cursor in multi-line text, drag to select across lines, Shift+Click to extend, double-click to select all

**⌨️ Keyboard Shortcuts**
- `Cmd+A` / `Ctrl+A` - Select all
- `Shift+Arrow` - Extend selection
- `Shift+Enter` - Insert newline
- `Enter` - Submit (without Shift)
- `Backspace` / `Delete` - Delete character or selection

**🎨 Visual Features**
- Automatic height adjustment based on content
- Cursor blinking animation
- Smooth selection highlighting
- No width jitter during selection

### Events

```rust
pub enum TextAreaEvent {
    Change(String),  // Value changed
    Submit(String),  // Enter key pressed (without Shift)
    Focus,           // Textarea focused
    Blur,            // Textarea blurred
}
```

### Example

```rust
let textarea = cx.new(|cx| {
    TextArea::new(cx)
        .placeholder("Type your message...")
        .min_height(80.0)
        .max_height(200.0)
        .bg_color(rgb(0xF0F9FF))
        .border_color(rgb(0x3B82F6))
});

cx.subscribe_in(&textarea, window, |_, _, event, _, _| {
    match event {
        TextAreaEvent::Change(value) => {
            println!("Content: {}", value);
        }
        TextAreaEvent::Submit(value) => {
            println!("Submitted: {}", value);
        }
        _ => {}
    }
}).detach();
```

---

## Checkbox

Checkbox component for boolean values.

### Constructor

```rust
Checkbox::new(id: impl Into<SharedString>) -> Self
```

### Methods

| Method | Type | Description |
|--------|------|-------------|
| `.label()` | `impl Into<SharedString>` | Set checkbox label |
| `.checked()` | `bool` | Set checked state |
| `.disabled()` | `bool` | Enable/disable the checkbox |

### Events

```rust
pub enum CheckboxEvent {
    Change(bool),  // Checked state changed
}
```

### Example

```rust
let checkbox = cx.new(|_| {
    Checkbox::new("agree")
        .label("I agree to the terms")
        .checked(false)
});

cx.subscribe_in(&checkbox, window, |_, _, event, _, _| {
    match event {
        CheckboxEvent::Change(checked) => {
            println!("Checked: {}", checked);
        }
    }
}).detach();
```

---

## Common Types

### ComponentSize

```rust
pub enum ComponentSize {
    XSmall,   // 11px font, 20px height
    Small,    // 13px font, 28px height
    Medium,   // 14px font, 36px height (default)
    Large,    // 16px font, 44px height
    XLarge,   // 18px font, 52px height
}
```

### IconSize

```rust
pub enum IconSize {
    XSmall,           // 12px
    Small,            // 16px
    Medium,           // 20px (default)
    Large,            // 24px
    XLarge,           // 32px
    Custom(f32),      // Custom size in pixels
}
```

### Theme Colors

```rust
// Access via Theme::default().colors

primary: Hsla       // #696FC7 (Purple)
secondary: Hsla     // #6B7280 (Gray)
success: Hsla       // #22C55E (Green)
warning: Hsla       // #F59E0B (Orange)
error: Hsla         // #EF4444 (Red)
info: Hsla          // #3B82F6 (Blue)
background: Hsla    // #FFFFFF (White)
surface: Hsla       // #F9FAFB (Light Gray)
border: Hsla        // #E5E7EB (Border Gray)
text: Hsla          // #111827 (Dark Gray)
text_muted: Hsla    // #6B7280 (Muted Gray)
```

---

## See Also

- [Tutorials](tutorials/README.md) - Step-by-step guides
- [Icon Reference](ICON_REFERENCE.md) - All icons with examples
- [Examples](../examples/) - Complete working examples

