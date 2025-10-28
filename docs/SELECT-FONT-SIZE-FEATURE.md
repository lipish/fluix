# Select Component - Independent Font Size Control

## 🎉 New Feature Added!

You can now change the **font size independently** from the component size in the Select component!

## ❌ The Problem

Previously, Fluix Select component had a limitation:

```rust
// ComponentSize controls BOTH font size AND component height together
Select::new(cx)
    .size(ComponentSize::Small)   // 13px font + 28px height
    .size(ComponentSize::Medium)  // 14px font + 36px height
    .size(ComponentSize::Large)   // 16px font + 44px height
```

**You couldn't change font size without changing the component height!**

This was problematic when you wanted to:
- Match TextInput font size (usually 12px)
- Keep consistent component heights
- Have smaller text in a larger component

## ✅ The Solution

Added a new `.font_size()` method that allows independent font size control:

```rust
Select::new(cx)
    .font_size(px(12.))  // Change ONLY font size
    .options(vec![...])
```

## 📖 How to Use

### Basic Usage

```rust
use fluix::*;
use gpui::*;

// Default: 14px font, 36px height
let default_select = cx.new(|cx| {
    Select::new(cx)
        .placeholder("Default")
        .options(vec![...])
});

// Custom font: 12px font, 36px height (same height!)
let custom_font_select = cx.new(|cx| {
    Select::new(cx)
        .placeholder("Custom font")
        .font_size(px(12.))  // ← NEW!
        .options(vec![...])
});
```

### Matching TextInput Font Size

Perfect for forms where you want Select and TextInput to have the same font size:

```rust
// TextInput typically uses ~12px font
let input = cx.new(|cx| {
    TextInput::new(cx)
        .placeholder("Enter text")
});

// Match the font size in Select
let select = cx.new(|cx| {
    Select::new(cx)
        .placeholder("Select option")
        .font_size(px(12.))  // ← Matches TextInput!
        .options(vec![...])
});
```

### Combining with Component Size

You can use both `.size()` and `.font_size()`:

```rust
// Large component (44px height) with small font (11px)
Select::new(cx)
    .size(ComponentSize::Large)  // 44px height
    .font_size(px(11.))           // But 11px font
    .options(vec![...])

// Small component (28px height) with large font (16px)
Select::new(cx)
    .size(ComponentSize::Small)  // 28px height
    .font_size(px(16.))           // But 16px font
    .options(vec![...])
```

## 🎯 Common Use Cases

### 1. Form Consistency

```rust
// All form fields with 12px font
div()
    .flex()
    .flex_col()
    .gap_4()
    .child(
        TextInput::new(cx)
            .placeholder("Name")
    )
    .child(
        Select::new(cx)
            .placeholder("Country")
            .font_size(px(12.))  // Matches TextInput
            .options(vec![...])
    )
```

### 2. Compact UI

```rust
// Smaller font for dense layouts
Select::new(cx)
    .font_size(px(11.))
    .options(vec![...])
```

### 3. Large Touch Targets with Readable Text

```rust
// Large component for easy clicking, but normal font size
Select::new(cx)
    .size(ComponentSize::Large)  // 44px - easy to click
    .font_size(px(14.))           // Normal readable size
    .options(vec![...])
```

## 📊 Font Size Reference

| Font Size | Use Case |
|-----------|----------|
| 11px | Compact UIs, dense layouts |
| 12px | **Recommended** - Matches TextInput |
| 13px | Small but readable |
| 14px | Default - Good for most cases |
| 16px | Large, prominent |
| 18px | Extra large, hero sections |

## 🔧 Implementation Details

### What Changed

**Added to Select struct**:
```rust
pub struct Select {
    // ... other fields
    custom_font_size: Option<Pixels>,  // ← NEW!
}
```

**New method**:
```rust
impl Select {
    pub fn font_size(mut self, size: Pixels) -> Self {
        self.custom_font_size = Some(size);
        self
    }
}
```

**Updated rendering**:
```rust
// Before
.text_size(self.size.font_size())

// After
.text_size(self.custom_font_size.unwrap_or(self.size.font_size()))
```

### Backward Compatibility

✅ **Fully backward compatible!**

Existing code continues to work without changes:

```rust
// This still works exactly as before
Select::new(cx)
    .size(ComponentSize::Medium)
    .options(vec![...])
```

The `.font_size()` method is **optional**. If not used, the component uses the default font size from `ComponentSize`.

## 📝 Examples

### Example 1: Form with Consistent Font Size

```rust
struct MyForm {
    name_input: Entity<TextInput>,
    country_select: Entity<Select>,
}

impl MyForm {
    fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        let name_input = cx.new(|cx| {
            TextInput::new(cx).placeholder("Your name")
        });

        let country_select = cx.new(|cx| {
            Select::new(cx)
                .placeholder("Select country")
                .font_size(px(12.))  // Matches TextInput
                .options(vec![
                    SelectOption::new("us", "United States"),
                    SelectOption::new("uk", "United Kingdom"),
                ])
        });

        Self { name_input, country_select }
    }
}
```

### Example 2: Different Font Sizes

```rust
// Small font for secondary info
let secondary_select = cx.new(|cx| {
    Select::new(cx)
        .placeholder("Filter by...")
        .font_size(px(11.))
        .options(vec![...])
});

// Large font for primary action
let primary_select = cx.new(|cx| {
    Select::new(cx)
        .placeholder("Choose your plan")
        .font_size(px(18.))
        .options(vec![...])
});
```

## 🎨 Demo

Run the demo to see it in action:

```bash
cargo run --example select_font_size_demo
```

The demo shows:
- Default font size (14px)
- Small font (11px) with same height
- Large font (18px) with same height
- Custom font (12px) - perfect for matching TextInput

## 📚 Documentation Updates

Updated documentation:
- ✅ [Component Tutorial](tutorials/02-COMPONENTS.md#custom-font-size-new)
- ✅ [Component Reference](COMPONENT-REFERENCE.md#select)
- ✅ [FAQ](FAQ.md#how-do-i-change-the-font-size-in-components)

## 🚀 Version

This feature is available in **Fluix v0.1.8+**

## 💡 Tips

1. **Use 12px for form consistency** - Matches most TextInput implementations
2. **Don't go below 11px** - Readability suffers
3. **Test on different screens** - Font size perception varies
4. **Consider accessibility** - Larger fonts are easier to read

## 🎊 Summary

**Before**:
- ❌ Font size tied to component size
- ❌ Couldn't match TextInput font size
- ❌ Had to choose between font size and component height

**After**:
- ✅ Independent font size control
- ✅ Can match TextInput perfectly
- ✅ Full flexibility for any design
- ✅ Backward compatible

---

**Start using it**: Just add `.font_size(px(12.))` to your Select components!

