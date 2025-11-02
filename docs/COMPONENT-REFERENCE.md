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

#### Builder Methods (Chainable)

| Method | Parameters | Returns | Description |
|--------|------------|---------|-------------|
| `.variant()` | `ButtonVariant` | `Self` | Set button variant (Primary, Secondary, Outline, Text, Danger). See [Variants](#variants) below |
| `.size()` | `ComponentSize` | `Self` | Set button size (XSmall to XLarge). Affects padding, font size, and height |
| `.disabled()` | `bool` | `Self` | Enable/disable the button. Disabled buttons cannot be clicked and have reduced opacity |
| `.loading()` | `bool` | `Self` | Show loading state. When `true`, displays "⏳ " prefix and disables interaction |
| `.full_width()` | `bool` | `Self` | Make button full width of its container. When `true`, button takes 100% width |
| `.icon()` | `IconName` | `Self` | Add an icon to the button. Icon appears before the label text |

### Variants

```rust
pub enum ButtonVariant {
    Primary,    // Blue background (#696FC7), white text. Best for primary actions
    Secondary,  // Gray background (#F9FAFB), dark text. Best for secondary actions
    Outline,    // Transparent background with border. Best for less prominent actions
    Text,       // Transparent background, no border. Best for subtle actions
    Danger,     // Red background (#EF4444), white text. Best for destructive actions
}
```

**Visual Details:**
- **Primary**: Filled background with shadow, highest visual prominence
- **Secondary**: Light gray background, subtle shadow
- **Outline**: Border only, transparent background, hover effect
- **Text**: No background or border, hover effect only
- **Danger**: Red background, similar to Primary but with error color

**Usage Guidelines:**
- Use **Primary** for the main action (e.g., "Save", "Submit")
- Use **Secondary** for supporting actions (e.g., "Cancel", "Back")
- Use **Outline** for less important actions (e.g., "Learn More")
- Use **Text** for minimal UI footprint (e.g., "View All")
- Use **Danger** for destructive actions (e.g., "Delete", "Remove")

### Events

```rust
pub enum ButtonEvent {
    Click,
}
```

### Examples

#### Basic Usage

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

#### All Variants

```rust
let primary = cx.new(|_| {
    Button::new("Primary")
        .variant(ButtonVariant::Primary)
});

let secondary = cx.new(|_| {
    Button::new("Secondary")
        .variant(ButtonVariant::Secondary)
});

let outline = cx.new(|_| {
    Button::new("Outline")
        .variant(ButtonVariant::Outline)
});

let text = cx.new(|_| {
    Button::new("Text")
        .variant(ButtonVariant::Text)
});

let danger = cx.new(|_| {
    Button::new("Delete")
        .variant(ButtonVariant::Danger)
});
```

#### Sizes

```rust
let xsmall = cx.new(|_| {
    Button::new("XSmall")
        .size(ComponentSize::XSmall)  // 11px font, 20px height
});

let small = cx.new(|_| {
    Button::new("Small")
        .size(ComponentSize::Small)   // 13px font, 28px height
});

let medium = cx.new(|_| {
    Button::new("Medium")
        .size(ComponentSize::Medium)  // 14px font, 36px height (default)
});

let large = cx.new(|_| {
    Button::new("Large")
        .size(ComponentSize::Large)    // 16px font, 44px height
});

let xlarge = cx.new(|_| {
    Button::new("XLarge")
        .size(ComponentSize::XLarge)   // 18px font, 52px height
});
```

#### With Icons

```rust
// Icon before text
let save_button = cx.new(|_| {
    Button::new("Save")
        .variant(ButtonVariant::Primary)
        .icon(IconName::Check)
});

let delete_button = cx.new(|_| {
    Button::new("Delete")
        .variant(ButtonVariant::Danger)
        .icon(IconName::Close)
});
```

#### States

```rust
// Disabled button
let disabled = cx.new(|_| {
    Button::new("Disabled")
        .disabled(true)
});

// Loading button
let loading = cx.new(|_| {
    Button::new("Loading")
        .loading(true)
});

// Full width button
let full_width = cx.new(|_| {
    Button::new("Full Width")
        .full_width(true)
});
```

### Notes

- **Hover Effects**: Buttons automatically show hover effects (darken/lighten) when not disabled or loading
- **Click Events**: Buttons emit `ButtonEvent::Click` when clicked (if not disabled or loading)
- **Icon Positioning**: Icons appear before the label text with appropriate spacing
- **Loading State**: Loading buttons show "⏳ " prefix and are automatically disabled
- **Full Width**: Full width buttons take 100% of their container width
- **Disabled State**: Disabled buttons have reduced opacity (64%) and cannot be clicked

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

### Examples

#### Basic Usage

```rust
// Basic icon (no background)
Icon::new(IconName::Star)
    .large()
    .color(rgb(0xF59E0B))
```

#### With Square Background

```rust
// Icon with square background
Icon::new(IconName::Send)
    .medium()
    .color(rgb(0xFFFFFF))
    .with_square_bg(rgb(0x3B82F6))
    .rounded(px(8.))
```

#### With Rectangular Background

```rust
// Icon with rectangular background
Icon::new(IconName::Send)
    .medium()
    .color(rgb(0x3B82F6))
    .with_rect_bg(px(80.), px(40.), rgb(0xEFF6FF))
    .rounded(px(8.))
```

#### Custom Sizes

```rust
// Using convenience methods
let small_icon = Icon::new(IconName::Star).small();
let medium_icon = Icon::new(IconName::Star).medium();
let large_icon = Icon::new(IconName::Star).large();

// Using IconSize enum
let custom_icon = Icon::new(IconName::Star)
    .size(IconSize::Custom(18.0));  // 18px custom size
```

#### Color Variations

```rust
// Theme colors
let primary_icon = Icon::new(IconName::Check)
    .color(rgb(0x696FC7));  // Primary color

let success_icon = Icon::new(IconName::Check)
    .color(rgb(0x22C55E));  // Success color

let error_icon = Icon::new(IconName::Error)
    .color(rgb(0xEF4444));  // Error color
```

### Notes

- **Default Size**: Icons default to 20px (Medium) if no size is specified
- **Background Sizing**: Square backgrounds match icon size. Rectangular backgrounds use specified dimensions
- **Border Radius**: Applied to background, not the icon itself
- **Color Inheritance**: Icons inherit theme text color if no color is specified
- **SVG Rendering**: All icons are rendered as SVG for crisp display at any size

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

#### Builder Methods (Chainable)

| Method | Parameters | Returns | Description |
|--------|------------|---------|-------------|
| `.placeholder()` | `impl Into<String>` | `Self` | Set placeholder text displayed when input is empty |
| `.value()` | `impl Into<String>` | `Self` | Set initial value and move cursor to end |
| `.password()` | `bool` | `Self` | Enable password mode (masks characters). When `true`, characters are masked based on `password_mask_mode` |
| `.show_password()` | `bool` | `Self` | Set password visibility. Only works when password mode is enabled. `true` = show characters, `false` = mask characters |
| `.password_mask_mode()` | `PasswordMaskMode` | `Self` | Set password masking mode. See [Password Mask Modes](#password-mask-modes) below |
| `.disabled()` | `bool` | `Self` | Enable/disable the input. Disabled inputs cannot be focused or edited |
| `.max_length()` | `usize` | `Self` | Set maximum character length. Input will reject characters beyond this limit |
| `.validator()` | `fn(&str) -> bool + 'static` | `Self` | Set validation function. Called on every change; input is rejected if function returns `false` |
| `.no_border()` | - | `Self` | Hide border (useful for embedded use cases like combobox) |
| `.bg_color()` | `Rgba` | `Self` | Set custom background color |
| `.border_color()` | `Rgba` | `Self` | Set custom border color |
| `.transparent()` | - | `Self` | Make background transparent (equivalent to `.bg_color(rgba(0x00000000))`) |

#### Instance Methods

| Method | Parameters | Returns | Description |
|--------|------------|---------|-------------|
| `.get_value()` | - | `&str` | Get the current input value (read-only) |
| `.set_value()` | `value: String, cx: &mut Context<Self>` | - | Set value programmatically. Triggers validation and emits `Change` event |
| `.clear()` | `cx: &mut Context<Self>` | - | Clear all text and reset cursor. Emits `Change` event |
| `.focus()` | `window: &mut Window` | - | Focus the input programmatically |
| `.select_all()` | `cx: &mut Context<Self>` | - | Select all text in the input |
| `.toggle_password_visibility()` | `cx: &mut Context<Self>` | - | Toggle password visibility (only works when password mode is enabled) |

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

**🔐 Password Features**
- Password visibility toggle
- Two masking modes:
  - `PasswordMaskMode::All` - Mask all characters with bullets (•)
  - `PasswordMaskMode::Partial` - Show first and last few characters, mask the middle (e.g., `f2••••••••44u9`)

### Password Mask Modes

```rust
use fluix::components::form::text_input::PasswordMaskMode;

// Full mask mode (default)
let password_input = cx.new(|cx| {
    TextInput::new(cx)
        .password(true)
        .password_mask_mode(PasswordMaskMode::All)
});

// Partial mask mode - show first 2 and last 2 characters
let partial_mask_input = cx.new(|cx| {
    TextInput::new(cx)
        .password(true)
        .password_mask_mode(PasswordMaskMode::Partial {
            prefix_len: 2,
            suffix_len: 2,
        })
});

// Toggle password visibility programmatically
password_input.update(cx, |input, cx| {
    input.toggle_password_visibility(cx);
});
```

### Events

```rust
pub enum TextInputEvent {
    Change(String),  // Value changed
    Submit(String),  // Enter key pressed
    Focus,           // Input focused
    Blur,            // Input blurred
}
```

### Examples

#### Basic Usage

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

#### Getting and Setting Values Programmatically

```rust
let input = cx.new(|cx| {
    TextInput::new(cx)
        .placeholder("Enter text")
});

// Get current value
let current_value = input.read(cx).get_value();
println!("Current value: {}", current_value);

// Set value programmatically
input.update(cx, |input, cx| {
    input.set_value("New value".to_string(), cx);
});

// Clear the input
input.update(cx, |input, cx| {
    input.clear(cx);
});

// Focus the input
input.read(cx).focus(window);
```

#### Custom Styling

```rust
// Custom background and border colors
let styled_input = cx.new(|cx| {
    TextInput::new(cx)
        .placeholder("Styled input")
        .bg_color(rgb(0xF0F9FF))      // Light blue background
        .border_color(rgb(0x3B82F6))    // Blue border
});

// Transparent background (no border)
let borderless_input = cx.new(|cx| {
    TextInput::new(cx)
        .placeholder("No border")
        .no_border()
        .transparent()
});
```

#### Password Input with Validation

```rust
let password_input = cx.new(|cx| {
    TextInput::new(cx)
        .placeholder("Enter password")
        .password(true)
        .password_mask_mode(PasswordMaskMode::Partial {
            prefix_len: 2,
            suffix_len: 2,
        })
        .validator(|value| {
            // Password must be at least 8 characters
            value.len() >= 8
        })
        .max_length(128)
});

// Toggle visibility programmatically
password_input.update(cx, |input, cx| {
    input.toggle_password_visibility(cx);
});
```

### Notes

- **Cursor Position**: After calling `.value()` or `.set_value()`, the cursor is automatically positioned at the end of the text
- **Validation**: The validator function is called on every change. If it returns `false`, the change is rejected
- **Password Masking**: When password mode is enabled and `password_visible` is `false`, characters are masked according to `password_mask_mode`
- **IME Support**: Full support for Chinese, Japanese, and Korean input methods with proper multi-byte character handling
- **Selection**: Use `select_all()` to programmatically select all text, or let users select with mouse/keyboard
- **Focus Management**: Use `focus()` to programmatically focus the input, useful for form navigation

---

## TextArea

A multi-line text area component with full editing capabilities.

### Constructor

```rust
TextArea::new(cx: &mut Context<Self>) -> Self
```

### Methods

#### Builder Methods (Chainable)

| Method | Parameters | Returns | Description |
|--------|------------|---------|-------------|
| `.placeholder()` | `impl Into<String>` | `Self` | Set placeholder text displayed when textarea is empty |
| `.value()` | `impl Into<String>` | `Self` | Set initial value and move cursor to end |
| `.disabled()` | `bool` | `Self` | Enable/disable the textarea. Disabled textareas cannot be focused or edited |
| `.max_length()` | `usize` | `Self` | Set maximum character length. Textarea will reject characters beyond this limit |
| `.min_height()` | `f32` | `Self` | Set minimum height in pixels. Textarea will not shrink below this height |
| `.max_height()` | `f32` | `Self` | Set maximum height in pixels. When content exceeds this height, scrollbar appears |
| `.bg_color()` | `Rgba` | `Self` | Set custom background color |
| `.border_color()` | `Rgba` | `Self` | Set custom border color (applies when not focused) |
| `.focus_border_color()` | `Rgba` | `Self` | Set custom border color when textarea is focused |
| `.no_border()` | - | `Self` | Remove border completely |

#### Instance Methods

| Method | Parameters | Returns | Description |
|--------|------------|---------|-------------|
| `.get_value()` | - | `&str` | Get the current textarea value (read-only) |
| `.set_value()` | `value: String, cx: &mut Context<Self>` | - | Set value programmatically. Emits `Change` event |
| `.clear()` | `cx: &mut Context<Self>` | - | Clear all text and reset cursor. Emits `Change` event |
| `.focus()` | `window: &mut Window` | - | Focus the textarea programmatically |
| `.select_all()` | `cx: &mut Context<Self>` | - | Select all text in the textarea |

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

### Examples

#### Basic Usage

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

#### Getting and Setting Values Programmatically

```rust
let textarea = cx.new(|cx| {
    TextArea::new(cx)
        .placeholder("Enter text")
});

// Get current value
let current_value = textarea.read(cx).get_value();
println!("Current value: {}", current_value);

// Set value programmatically
textarea.update(cx, |textarea, cx| {
    textarea.set_value("New value\nLine 2".to_string(), cx);
});

// Clear the textarea
textarea.update(cx, |textarea, cx| {
    textarea.clear(cx);
});

// Focus the textarea
textarea.read(cx).focus(window);
```

#### Height Management

```rust
// Fixed height (no scrolling)
let fixed_height = cx.new(|cx| {
    TextArea::new(cx)
        .min_height(100.0)
        .max_height(100.0)  // Same as min = fixed height
});

// Auto-expanding with max limit
let auto_expanding = cx.new(|cx| {
    TextArea::new(cx)
        .min_height(40.0)    // Start at 40px
        .max_height(200.0)   // Expand up to 200px, then scroll
});
```

#### Custom Styling

```rust
// Custom colors
let styled_textarea = cx.new(|cx| {
    TextArea::new(cx)
        .placeholder("Styled textarea")
        .bg_color(rgb(0xF0F9FF))           // Light blue background
        .border_color(rgb(0x3B82F6))       // Blue border
        .focus_border_color(rgb(0x2563EB)) // Darker blue when focused
});

// Borderless
let borderless_textarea = cx.new(|cx| {
    TextArea::new(cx)
        .placeholder("No border")
        .no_border()
        .bg_color(rgb(0xFAFAFA))
});
```

### Notes

- **Auto-Height**: Textarea automatically adjusts height based on content between `min_height` and `max_height`
- **Scrolling**: When content exceeds `max_height`, a vertical scrollbar appears
- **Line Breaks**: Users can insert line breaks with `Shift+Enter`. `Enter` alone triggers `Submit` event
- **Selection**: Use `select_all()` to programmatically select all text, or let users select with mouse/keyboard
- **Focus Management**: Use `focus()` to programmatically focus the textarea, useful for form navigation

---

## Checkbox

Checkbox component for boolean values.

### Constructor

```rust
Checkbox::new(cx: &mut Context<Self>) -> Self
```

### Methods

#### Builder Methods (Chainable)

| Method | Parameters | Returns | Description |
|--------|------------|---------|-------------|
| `.label()` | `impl Into<String>` | `Self` | Set checkbox label text displayed next to the checkbox |
| `.checked()` | `bool` | `Self` | Set initial checked state. `true` = checked, `false` = unchecked |
| `.disabled()` | `bool` | `Self` | Enable/disable the checkbox. Disabled checkboxes cannot be clicked and have reduced opacity |
| `.size()` | `ComponentSize` | `Self` | Set checkbox size (XSmall to XLarge). Affects checkbox box size and label font size |
| `.text_color()` | `Rgba` | `Self` | Set custom text color for the label |

### Events

```rust
pub enum CheckboxEvent {
    Changed(bool),  // Checked state changed. Value is the new checked state
}
```

### Examples

#### Basic Usage

```rust
let checkbox = cx.new(|cx| {
    Checkbox::new(cx)
        .label("I agree to the terms")
        .checked(false)
});

cx.subscribe_in(&checkbox, window, |_, _, event, _, _| {
    match event {
        CheckboxEvent::Changed(checked) => {
            println!("Checked: {}", checked);
        }
    }
}).detach();
```

#### Getting Current State

```rust
let checkbox = cx.new(|cx| {
    Checkbox::new(cx)
        .label("Enable feature")
        .checked(true)
});

// Note: Checkbox state is managed internally.
// Use events to track state changes.
```

#### Disabled State

```rust
let disabled_checkbox = cx.new(|cx| {
    Checkbox::new(cx)
        .label("This is disabled")
        .checked(false)
        .disabled(true)
});
```

#### Custom Styling

```rust
let styled_checkbox = cx.new(|cx| {
    Checkbox::new(cx)
        .label("Custom styled checkbox")
        .size(ComponentSize::Large)
        .text_color(rgb(0x3B82F6))  // Blue text
});
```

### Notes

- **ID Requirement**: Each checkbox must have a unique ID for proper event handling
- **State Management**: Checkbox state is managed internally. Use events to track changes
- **Visual Feedback**: Checkboxes show visual feedback on hover (when not disabled)
- **Accessibility**: Checkboxes are keyboard accessible (Space key toggles state)
- **Size**: Checkbox box size scales with `ComponentSize` (14px to 22px)
- **Event Name**: The event is `CheckboxEvent::Changed(bool)`, not `Change`

---

## Common Types

### ComponentSize

Standard size enum used by buttons and other components.

```rust
pub enum ComponentSize {
    XSmall,   // 11px font, 20px height, 4px padding
    Small,    // 13px font, 28px height, 6px padding
    Medium,   // 14px font, 36px height, 8px padding (default)
    Large,    // 16px font, 44px height, 10px padding
    XLarge,   // 18px font, 52px height, 12px padding
}
```

**Size Details:**
- **XSmall**: Compact buttons, dense UIs
- **Small**: Secondary actions, compact forms
- **Medium**: Standard buttons, most common use case (default)
- **Large**: Primary actions, prominent buttons
- **XLarge**: Hero buttons, very prominent actions

**Usage:**
```rust
use fluix::ComponentSize;

Button::new("Click me")
    .size(ComponentSize::Large)
```

### IconSize

Icon size enum with predefined sizes and custom option.

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

**Usage:**
```rust
use fluix::IconSize;

Icon::new(IconName::Star)
    .size(IconSize::Large)

// Custom size
Icon::new(IconName::Star)
    .size(IconSize::Custom(18.0))  // 18px
```

### PasswordMaskMode

Password masking mode for TextInput component.

```rust
pub enum PasswordMaskMode {
    /// Mask all characters with bullets (•)
    All,
    /// Show first and last few characters, mask the middle
    Partial {
        /// Number of characters to show at the start
        prefix_len: usize,
        /// Number of characters to show at the end
        suffix_len: usize,
    },
}
```

**Usage:**
```rust
use fluix::components::form::text_input::PasswordMaskMode;

// Full mask (default)
TextInput::new(cx)
    .password(true)
    .password_mask_mode(PasswordMaskMode::All)

// Partial mask - show first 2 and last 2 characters
TextInput::new(cx)
    .password(true)
    .password_mask_mode(PasswordMaskMode::Partial {
        prefix_len: 2,
        suffix_len: 2,
    })
```

**Example Output:**
- `PasswordMaskMode::All`: Password `f26612345678944u9` → `•••••••••••••••••`
- `PasswordMaskMode::Partial { prefix_len: 2, suffix_len: 2 }`: Password `f26612345678944u9` → `f2••••••••••••••44u9`

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

