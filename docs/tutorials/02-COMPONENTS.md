# Working with Components

This tutorial covers all available Fluix components and how to use them effectively.

## 📦 Available Components

Fluix currently provides:
- **Button** - Interactive buttons with variants
- **Icon** - 22 SVG icons
- **Select** - Dropdown selection (single/multiple)
- **TextInput** - Text input fields
- **Checkbox** - Checkboxes for boolean values

## 🔘 Button Component

### Basic Usage

```rust
use fluix::*;
use gpui::*;

// Simple button
Button::new("Click Me")

// With variant
Button::new("Primary")
    .variant(ButtonVariant::Primary)

Button::new("Secondary")
    .variant(ButtonVariant::Secondary)

Button::new("Danger")
    .variant(ButtonVariant::Danger)
```

### Button Variants

```rust
pub enum ButtonVariant {
    Primary,    // Blue background
    Secondary,  // Gray background
    Outline,    // Transparent with border
    Ghost,      // Transparent, no border
    Danger,     // Red background
}
```

### Button Sizes

```rust
Button::new("Extra Small")
    .size(ComponentSize::XSmall)  // 11px font, 20px height

Button::new("Small")
    .size(ComponentSize::Small)   // 13px font, 28px height

Button::new("Medium")
    .size(ComponentSize::Medium)  // 14px font, 36px height (default)

Button::new("Large")
    .size(ComponentSize::Large)   // 16px font, 44px height

Button::new("Extra Large")
    .size(ComponentSize::XLarge)  // 18px font, 52px height
```

### Button States

```rust
// Disabled button
Button::new("Disabled")
    .disabled(true)

// Loading button
Button::new("Loading...")
    .loading(true)

// Full width button
Button::new("Full Width")
    .full_width(true)
```

### Handling Button Events

```rust
struct MyView {
    button: Entity<Button>,
}

impl MyView {
    fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        let button = cx.new(|_| {
            Button::new("Click Me")
                .variant(ButtonVariant::Primary)
        });

        // Subscribe to button events
        cx.subscribe_in(&button, window, Self::on_click).detach();

        Self { button }
    }

    fn on_click(
        &mut self,
        _: &Entity<Button>,
        event: &ButtonEvent,
        _: &mut Window,
        _: &mut Context<Self>,
    ) {
        match event {
            ButtonEvent::Click => {
                println!("Button clicked!");
            }
        }
    }
}
```

## 🎨 Icon Component

### Available Icons (22 total)

**Navigation**: ArrowLeft, ArrowRight, ArrowUp, ArrowDown, ChevronUpDown, UnfoldMore  
**Actions**: Check, Close, Plus, Minus, Search  
**UI**: Settings, Home, User, Bell, Star, Heart, Menu  
**Status**: Info, Warning, Error, Success

### Basic Usage

```rust
use fluix::*;
use gpui::*;

// Simple icon
Icon::new(IconName::Star)

// With size
Icon::new(IconName::Search)
    .medium()  // or .small(), .large(), .xlarge()

// With color
Icon::new(IconName::Heart)
    .large()
    .color(rgb(0xFF0000))  // Red heart

// Custom size
Icon::new(IconName::Settings)
    .size(IconSize::Custom(48.0))
    .color(rgb(0x666666))
```

### Icon Sizes

```rust
Icon::new(IconName::Star).xsmall()  // 12px
Icon::new(IconName::Star).small()   // 16px
Icon::new(IconName::Star).medium()  // 20px (default)
Icon::new(IconName::Star).large()   // 24px
Icon::new(IconName::Star).xlarge()  // 32px
```

### Semantic Colors

```rust
// Info - Blue
Icon::new(IconName::Info).color(rgb(0x3B82F6))

// Success - Green
Icon::new(IconName::Success).color(rgb(0x22C55E))

// Warning - Orange
Icon::new(IconName::Warning).color(rgb(0xF59E0B))

// Error - Red
Icon::new(IconName::Error).color(rgb(0xEF4444))
```

### Using Icons in Layouts

```rust
div()
    .flex()
    .items_center()
    .gap_2()
    .child(Icon::new(IconName::Search).medium())
    .child("Search")
```

## 📋 Select Component

### Single Selection

```rust
use fluix::*;

struct MyView {
    select: Entity<Select>,
}

impl MyView {
    fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        let select = cx.new(|_| {
            Select::new("framework")
                .placeholder("Choose a framework")
                .options(vec![
                    SelectOption::new("react", "React"),
                    SelectOption::new("vue", "Vue"),
                    SelectOption::new("angular", "Angular"),
                ])
        });

        cx.subscribe_in(&select, window, Self::on_select).detach();

        Self { select }
    }

    fn on_select(
        &mut self,
        _: &Entity<Select>,
        event: &SelectEvent,
        _: &mut Window,
        _: &mut Context<Self>,
    ) {
        match event {
            SelectEvent::Change(value) => {
                println!("Selected: {}", value);
            }
        }
    }
}
```

### Multiple Selection

```rust
Select::new("languages")
    .placeholder("Select languages")
    .multiple(true)  // Enable multi-select
    .options(vec![
        SelectOption::new("rust", "Rust"),
        SelectOption::new("go", "Go"),
        SelectOption::new("python", "Python"),
        SelectOption::new("javascript", "JavaScript"),
    ])
```

### Grouped Options

```rust
Select::new("tech")
    .placeholder("Select technology")
    .option_groups(vec![
        SelectOptionGroup::new("Frontend")
            .option(SelectOption::new("react", "React"))
            .option(SelectOption::new("vue", "Vue"))
            .option(SelectOption::new("svelte", "Svelte")),
        SelectOptionGroup::new("Backend")
            .option(SelectOption::new("rust", "Rust"))
            .option(SelectOption::new("go", "Go"))
            .option(SelectOption::new("node", "Node.js")),
    ])
```

### Select Sizes

```rust
// Small select (13px font, 28px height)
Select::new(cx)
    .size(ComponentSize::Small)
    .options(vec![...])

// Large select (16px font, 44px height)
Select::new(cx)
    .size(ComponentSize::Large)
    .options(vec![...])
```

### Custom Font Size (New!)

You can now change the font size **independently** from the component size:

```rust
// Medium component size (36px height) but small font (11px)
Select::new(cx)
    .font_size(px(11.))
    .options(vec![...])

// Medium component size (36px height) but custom font (12px)
// Perfect for matching TextInput!
Select::new(cx)
    .font_size(px(12.))
    .options(vec![...])

// You can combine with .size() too
Select::new(cx)
    .size(ComponentSize::Large)  // 44px height
    .font_size(px(12.))           // But 12px font
    .options(vec![...])
```

### Custom Background Color (New!)

You can now customize the background color:

```rust
// Light blue background
Select::new(cx)
    .placeholder("Choose option")
    .bg_color(rgb(0xEFF6FF))  // Light blue
    .options(vec![...])

// Light green background (success theme)
Select::new(cx)
    .placeholder("Status")
    .bg_color(rgb(0xDCFCE7))  // Light green
    .options(vec![...])

// Light yellow background (warning theme)
Select::new(cx)
    .placeholder("Priority")
    .bg_color(rgb(0xFEFCE8))  // Light yellow
    .options(vec![...])

// Combine all customizations
Select::new(cx)
    .placeholder("Fully customized")
    .size(ComponentSize::Large)      // Custom size
    .font_size(px(12.))               // Custom font
    .bg_color(rgb(0xEFF6FF))          // Custom background
    .options(vec![...])

// Dark theme (NEW!)
Select::new(cx)
    .placeholder("Dark theme")
    .bg_color(rgb(0x1F2937))          // Dark background
    .text_color(rgb(0xFFFFFF))        // White text
    .options(vec![...])
```

### Pre-selected Values

```rust
// Single select
Select::new("framework")
    .value("react")  // Pre-select React
    .options(vec![...])

// Multiple select
Select::new("languages")
    .multiple(true)
    .values(vec!["rust".to_string(), "go".to_string()])  // Pre-select multiple
    .options(vec![...])
```

## 📝 TextInput Component

A powerful single-line text input component with full editing capabilities.

### Basic Usage

```rust
struct MyView {
    input: Entity<TextInput>,
}

impl MyView {
    fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        let input = cx.new(|cx| {
            TextInput::new(cx)
                .placeholder("Enter your name")
        });

        cx.subscribe_in(&input, window, Self::on_input).detach();

        Self { input }
    }

    fn on_input(
        &mut self,
        _: &Entity<TextInput>,
        event: &TextInputEvent,
        _: &mut Window,
        _: &mut Context<Self>,
    ) {
        match event {
            TextInputEvent::Change(value) => {
                println!("Input changed: {}", value);
            }
            TextInputEvent::Submit(value) => {
                println!("Input submitted: {}", value);
            }
            _ => {}
        }
    }
}
```

### Features

TextInput includes many advanced features:

**✨ IME Support**
- Full support for Chinese, Japanese, Korean input
- Proper handling of multi-byte characters
- Accurate cursor positioning

**🖱️ Mouse Selection**
- Click to position cursor precisely
- Drag to select text
- Shift+Click to extend selection
- Double-click to select word (upcoming)

**⌨️ Keyboard Shortcuts**
- `Cmd+A` / `Ctrl+A` - Select all text
- `Shift+Left/Right` - Extend selection
- `Shift+Home/End` - Extend selection to start/end
- `Left/Right` - Move cursor
- `Home/End` - Jump to start/end
- `Backspace` - Delete character or selection
- `Delete` - Delete character at cursor
- `Enter` - Submit input

**🎨 Visual Features**
- Cursor blinking animation (530ms interval)
- Cursor hides when text is selected
- Smooth text selection highlighting
- No width jitter during selection

### Password Input

TextInput supports password mode with flexible masking options:

```rust
use fluix::components::form::text_input::PasswordMaskMode;

// Basic password input (full mask)
let password_input = cx.new(|cx| {
    TextInput::new(cx)
        .placeholder("Enter password")
        .password(true)  // Mask all characters with bullets (•)
});

// Partial mask mode - show first 2 and last 2 characters
// Example: password "f26612345678944u9" displays as "f2••••••••••••••44u9"
let partial_mask_input = cx.new(|cx| {
    TextInput::new(cx)
        .placeholder("Enter password")
        .password(true)
        .password_mask_mode(PasswordMaskMode::Partial {
            prefix_len: 2,  // Show first 2 characters
            suffix_len: 2,  // Show last 2 characters
        })
});

// Toggle password visibility programmatically
password_input.update(cx, |input, cx| {
    input.toggle_password_visibility(cx);
});

// Set initial visibility state
let visible_password = cx.new(|cx| {
    TextInput::new(cx)
        .password(true)
        .show_password(true)  // Start with password visible
});
```

**Password Mask Modes:**
- `PasswordMaskMode::All` - All characters masked with bullets (default)
- `PasswordMaskMode::Partial { prefix_len, suffix_len }` - Show prefix and suffix, mask middle

### Input Validation

```rust
TextInput::new(cx)
    .placeholder("Enter email")
    .validator(|value| {
        value.contains('@')  // Simple email validation
    })
```

### Max Length

```rust
TextInput::new(cx)
    .placeholder("Username (max 20 chars)")
    .max_length(20)
```

### Pre-filled Value

```rust
TextInput::new(cx)
    .value("Initial text")
    .placeholder("Enter text...")
```

### Disabled State

```rust
TextInput::new(cx)
    .value("Cannot edit this")
    .disabled(true)
```

## 📄 TextArea Component

A multi-line text area component with full editing capabilities.

### Basic Usage

```rust
struct MyView {
    textarea: Entity<TextArea>,
}

impl MyView {
    fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        let textarea = cx.new(|cx| {
            TextArea::new(cx)
                .placeholder("Type your message...")
                .min_height(80.0)
                .max_height(200.0)
        });

        cx.subscribe_in(&textarea, window, Self::on_textarea).detach();

        Self { textarea }
    }

    fn on_textarea(
        &mut self,
        _: &Entity<TextArea>,
        event: &TextAreaEvent,
        _: &mut Window,
        _: &mut Context<Self>,
    ) {
        match event {
            TextAreaEvent::Change(value) => {
                println!("Content changed: {}", value);
            }
            TextAreaEvent::Submit(value) => {
                println!("Submitted: {}", value);
            }
            _ => {}
        }
    }
}
```

### Features

TextArea shares many features with TextInput:

**🖱️ Mouse Selection**
- Click to position cursor in multi-line text
- Drag to select text across multiple lines
- Shift+Click to extend selection
- Double-click to select all text

**⌨️ Keyboard Shortcuts**
- `Cmd+A` / `Ctrl+A` - Select all text
- `Shift+Left/Right` - Extend selection
- `Shift+Home/End` - Extend selection to start/end
- `Shift+Enter` - Insert newline
- `Enter` - Submit (without Shift)
- `Backspace` / `Delete` - Delete character or selection

**🎨 Visual Features**
- Automatic height adjustment based on content
- Cursor blinking animation
- Smooth text selection highlighting
- No width jitter during selection

### Custom Styling

```rust
TextArea::new(cx)
    .placeholder("Styled textarea...")
    .min_height(60.0)
    .max_height(200.0)
    .bg_color(rgb(0xF0F9FF))          // Light blue background
    .border_color(rgb(0x3B82F6))       // Blue border
    .focus_border_color(rgb(0x2563EB)) // Darker blue on focus
```

### Borderless TextArea

```rust
TextArea::new(cx)
    .placeholder("No border...")
    .bg_color(rgb(0xFAFAFA))
    .no_border()
```

### Height Constraints

```rust
// Fixed height
TextArea::new(cx)
    .min_height(100.0)
    .max_height(100.0)

// Flexible height
TextArea::new(cx)
    .min_height(80.0)
    // No max_height = unlimited

// Auto-growing
TextArea::new(cx)
    .min_height(50.0)
    // Height grows with content
```

## ✅ Checkbox Component

### Basic Usage

```rust
struct MyView {
    checkbox: Entity<Checkbox>,
}

impl MyView {
    fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        let checkbox = cx.new(|_| {
            Checkbox::new("agree")
                .label("I agree to the terms")
        });

        cx.subscribe_in(&checkbox, window, Self::on_check).detach();

        Self { checkbox }
    }

    fn on_check(
        &mut self,
        _: &Entity<Checkbox>,
        event: &CheckboxEvent,
        _: &mut Window,
        _: &mut Context<Self>,
    ) {
        match event {
            CheckboxEvent::Change(checked) => {
                println!("Checkbox: {}", if *checked { "checked" } else { "unchecked" });
            }
        }
    }
}
```

## 🎯 Complete Example

Here's a complete example using multiple components:

```rust
use fluix::*;
use gpui::*;

struct ContactForm {
    name_input: Entity<TextInput>,
    email_input: Entity<TextInput>,
    framework_select: Entity<Select>,
    newsletter_checkbox: Entity<Checkbox>,
    submit_button: Entity<Button>,
}

impl ContactForm {
    fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        let name_input = cx.new(|cx| {
            TextInput::new(cx).placeholder("Your name")
        });

        let email_input = cx.new(|cx| {
            TextInput::new(cx)
                .placeholder("Your email")
                .validator(|v| v.contains('@'))
        });

        let framework_select = cx.new(|_| {
            Select::new("framework")
                .placeholder("Favorite framework")
                .options(vec![
                    SelectOption::new("react", "React"),
                    SelectOption::new("vue", "Vue"),
                    SelectOption::new("svelte", "Svelte"),
                ])
        });

        let newsletter_checkbox = cx.new(|_| {
            Checkbox::new("newsletter")
                .label("Subscribe to newsletter")
        });

        let submit_button = cx.new(|_| {
            Button::new("Submit")
                .variant(ButtonVariant::Primary)
                .size(ComponentSize::Large)
        });

        cx.subscribe_in(&submit_button, window, Self::on_submit).detach();

        Self {
            name_input,
            email_input,
            framework_select,
            newsletter_checkbox,
            submit_button,
        }
    }

    fn on_submit(
        &mut self,
        _: &Entity<Button>,
        _: &ButtonEvent,
        _: &mut Window,
        _: &mut Context<Self>,
    ) {
        println!("Form submitted!");
    }
}

impl Render for ContactForm {
    fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
        div()
            .flex()
            .flex_col()
            .gap_4()
            .p_8()
            .max_w(px(400.))
            .child(self.name_input.clone())
            .child(self.email_input.clone())
            .child(self.framework_select.clone())
            .child(self.newsletter_checkbox.clone())
            .child(self.submit_button.clone())
    }
}
```

---

**Next**: [Styling and Theming →](./03-STYLING.md)

