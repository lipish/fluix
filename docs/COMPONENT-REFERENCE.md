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

SVG icon component with 22 built-in icons.

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

### Icon Names

**Navigation** (6):
- `ArrowLeft`, `ArrowRight`, `ArrowUp`, `ArrowDown`
- `ChevronUpDown`, `UnfoldMore`

**Actions** (5):
- `Check`, `Close`, `Plus`, `Minus`, `Search`

**UI** (7):
- `Settings`, `Home`, `User`, `Bell`
- `Star`, `Heart`, `Menu`

**Status** (4):
- `Info`, `Warning`, `Error`, `Success`

### Example

```rust
// Basic icon
Icon::new(IconName::Star)
    .large()
    .color(rgb(0xF59E0B))

// Custom size
Icon::new(IconName::Search)
    .size(IconSize::Custom(48.0))
    .color(rgb(0x666666))
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
| `.text_color()` | `Rgba` | Set custom text color ⭐ NEW |
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

// Dark theme with custom text color (NEW!)
let dark_theme_select = cx.new(|cx| {
    Select::new(cx)
        .placeholder("Dark theme")
        .bg_color(rgb(0x1F2937))          // Dark gray background
        .text_color(rgb(0xFFFFFF))        // White text
        .options(vec![...])
});
```

---

## TextInput

Text input field component.

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

