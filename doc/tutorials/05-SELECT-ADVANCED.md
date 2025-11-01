# Tutorial 5: Select Component Advanced Features

Master the advanced features of the Select component including compact mode, alignment, and variants.

## Table of Contents

- [Introduction](#introduction)
- [Compact Mode](#compact-mode)
- [Dropdown Alignment](#dropdown-alignment)
- [Visual Variants](#visual-variants)
- [Dropdown Direction](#dropdown-direction)
- [Combining Features](#combining-features)
- [Real-World Examples](#real-world-examples)

## Introduction

The Select component offers powerful customization options for different use cases:

- **Compact Mode** - Tighter spacing for long lists
- **Alignment** - Left, right, or center alignment
- **Variants** - Default, Ghost, or Outline styles
- **Direction** - Expand up or down

## Compact Mode

Compact mode reduces vertical spacing by 50%, perfect for long lists.

### Basic Compact Mode

```rust
Select::new(cx)
    .placeholder("Select framework")
    .compact()  // Enable compact mode
    .options(vec![
        SelectOption::new("react", "React"),
        SelectOption::new("vue", "Vue"),
        SelectOption::new("angular", "Angular"),
        // ... many more options
    ])
```

### When to Use Compact Mode

✅ **Use compact mode when**:
- You have more than 10 options
- You have grouped options with multiple groups
- Space is limited (embedded selects)
- You want to minimize scrolling

❌ **Don't use compact mode when**:
- You have fewer than 5 options
- You need larger touch targets
- You want to emphasize each option

### Spacing Comparison

| Mode | Vertical Padding | Horizontal Padding |
|------|-----------------|-------------------|
| Normal | 8px | 12px |
| Compact | 4px | 8px |

### Example: Long List

```rust
// Country selector with 200+ options
Select::new(cx)
    .placeholder("Select country")
    .compact()  // Compact mode for long list
    .options(countries)
```

## Dropdown Alignment

Control where the dropdown menu appears relative to the trigger.

### Left Alignment (Default)

```rust
Select::new(cx)
    .placeholder("Left aligned")
    .align_left()  // Default behavior
    .options(vec![...])
```

The dropdown's left edge aligns with the trigger's left edge.

### Right Alignment

```rust
Select::new(cx)
    .placeholder("Right aligned")
    .align_right()  // Align to right edge
    .options(vec![...])
```

Perfect for right-side toolbars and user menus.

### Center Alignment

```rust
Select::new(cx)
    .placeholder("Center aligned")
    .align_center()  // Center align
    .options(vec![...])
```

Great for centered dialogs and modals.

### Alignment Use Cases

**Left Alignment**:
- Default for most cases
- Left-side navigation
- Form fields

**Right Alignment**:
- Right-side toolbars
- User profile menus
- Table action columns

**Center Alignment**:
- Dialog boxes
- Centered page elements
- Symmetrical layouts

## Visual Variants

Three visual styles for different design needs.

### Default Variant

```rust
Select::new(cx)
    .variant(SelectVariant::Default)
    .options(vec![...])
```

Standard style with border and background.

### Ghost Variant

```rust
Select::new(cx)
    .variant(SelectVariant::Ghost)
    .options(vec![...])
```

No border, transparent background - blends into the UI.

### Outline Variant

```rust
Select::new(cx)
    .variant(SelectVariant::Outline)
    .options(vec![...])
```

Border only, transparent background.

### Variant Comparison

| Variant | Border | Background | Use Case |
|---------|--------|------------|----------|
| Default | ✓ | Solid | Standard forms |
| Ghost | ✗ | Transparent | Embedded UI |
| Outline | ✓ | Transparent | Minimal design |

## Dropdown Direction

Control whether the dropdown expands up or down.

### Expand Down (Default)

```rust
Select::new(cx)
    .dropdown_direction(DropdownDirection::Down)
    .options(vec![...])
```

### Expand Up

```rust
Select::new(cx)
    .dropdown_direction(DropdownDirection::Up)
    .options(vec![...])
```

Perfect for bottom toolbars and fixed-bottom elements.

### Direction Use Cases

**Expand Down**:
- Top navigation
- Form fields
- Most standard cases

**Expand Up**:
- Bottom toolbars
- Fixed bottom menus
- When space below is limited

## Combining Features

The real power comes from combining features.

### Compact + Ghost

```rust
Select::new(cx)
    .variant(SelectVariant::Ghost)
    .compact()
    .size(ComponentSize::Small)
    .text_color(rgb(0x999999))
    .options(providers)
```

Perfect for embedded selects in Settings views.

### Right Align + Up

```rust
Select::new(cx)
    .align_right()
    .dropdown_direction(DropdownDirection::Up)
    .options(vec![
        SelectOption::new("profile", "Profile"),
        SelectOption::new("settings", "Settings"),
        SelectOption::new("logout", "Logout"),
    ])
```

Ideal for user menus in the top-right corner.

### Compact + Center + Up

```rust
Select::new(cx)
    .compact()
    .align_center()
    .dropdown_direction(DropdownDirection::Up)
    .options(languages)
```

Great for language selectors in bottom center.

### Full Customization

```rust
Select::new(cx)
    .variant(SelectVariant::Ghost)
    .compact()
    .align_right()
    .dropdown_direction(DropdownDirection::Up)
    .size(ComponentSize::Small)
    .text_color(rgb(0x666666))
    .no_shadow()
    .options(vec![...])
```

## Real-World Examples

### Example 1: Settings Provider Select

```rust
// Embedded in Settings view
Select::new(cx)
    .placeholder("Select provider")
    .variant(SelectVariant::Ghost)
    .compact()
    .size(ComponentSize::Small)
    .text_color(rgb(0x999999))
    .options(vec![
        SelectOption::new("openai", "OpenAI"),
        SelectOption::new("anthropic", "Anthropic"),
        SelectOption::new("google", "Google"),
    ])
```

**Why this combination?**
- Ghost: Blends into settings UI
- Compact: Saves space
- Small: Matches settings text size
- Gray text: Subtle appearance

### Example 2: User Menu (Top Right)

```rust
Select::new(cx)
    .placeholder("User")
    .align_right()
    .dropdown_direction(DropdownDirection::Down)
    .variant(SelectVariant::Ghost)
    .size(ComponentSize::Small)
    .options(vec![
        SelectOption::new("profile", "👤 Profile"),
        SelectOption::new("settings", "⚙️ Settings"),
        SelectOption::new("logout", "🚪 Logout"),
    ])
```

**Why this combination?**
- Right align: Menu in top-right corner
- Down: Standard dropdown direction
- Ghost: Clean appearance
- Small: Compact menu

### Example 3: Table Action Column

```rust
Select::new(cx)
    .placeholder("Actions")
    .align_right()
    .compact()
    .variant(SelectVariant::Ghost)
    .size(ComponentSize::Small)
    .options(vec![
        SelectOption::new("edit", "Edit"),
        SelectOption::new("duplicate", "Duplicate"),
        SelectOption::new("delete", "Delete"),
    ])
```

**Why this combination?**
- Right align: Last column alignment
- Compact: Fits in table row
- Ghost: Minimal visual weight
- Small: Matches table text

### Example 4: Bottom Language Selector

```rust
Select::new(cx)
    .placeholder("Language")
    .align_center()
    .dropdown_direction(DropdownDirection::Up)
    .compact()
    .variant(SelectVariant::Outline)
    .options(vec![
        SelectOption::new("en", "🇺🇸 English"),
        SelectOption::new("zh", "🇨🇳 中文"),
        SelectOption::new("ja", "🇯🇵 日本語"),
    ])
```

**Why this combination?**
- Center align: Centered in footer
- Up: Expands upward from bottom
- Compact: More language options visible
- Outline: Clear but minimal

### Example 5: Long Country List

```rust
Select::new(cx)
    .placeholder("Select country")
    .compact()
    .options(vec![
        SelectOption::new("us", "United States"),
        SelectOption::new("uk", "United Kingdom"),
        SelectOption::new("ca", "Canada"),
        // ... 200+ countries
    ])
```

**Why this combination?**
- Compact: Fits more countries without scrolling
- Default alignment: Standard form behavior
- Default variant: Clear selection UI

## Best Practices

### 1. Choose the Right Variant

```rust
// Forms - use Default
Select::new(cx)
    .variant(SelectVariant::Default)

// Embedded UI - use Ghost
Select::new(cx)
    .variant(SelectVariant::Ghost)

// Minimal design - use Outline
Select::new(cx)
    .variant(SelectVariant::Outline)
```

### 2. Use Compact for Long Lists

```rust
// Short list (< 10 items) - normal spacing
Select::new(cx)
    .options(short_list)

// Long list (> 10 items) - compact spacing
Select::new(cx)
    .compact()
    .options(long_list)
```

### 3. Align Based on Position

```rust
// Left side - left align
Select::new(cx).align_left()

// Right side - right align
Select::new(cx).align_right()

// Center - center align
Select::new(cx).align_center()
```

### 4. Direction Based on Space

```rust
// Top half of screen - expand down
Select::new(cx)
    .dropdown_direction(DropdownDirection::Down)

// Bottom half of screen - expand up
Select::new(cx)
    .dropdown_direction(DropdownDirection::Up)
```

## Quick Reference

### Feature Matrix

| Feature | Method | Options |
|---------|--------|---------|
| Compact | `.compact()` | On/Off |
| Alignment | `.align_left/right/center()` | Left, Right, Center |
| Variant | `.variant()` | Default, Ghost, Outline |
| Direction | `.dropdown_direction()` | Down, Up, Auto |

### Common Combinations

```rust
// Settings embed
.variant(SelectVariant::Ghost).compact().size(ComponentSize::Small)

// User menu
.align_right().dropdown_direction(DropdownDirection::Down).variant(SelectVariant::Ghost)

// Table actions
.align_right().compact().variant(SelectVariant::Ghost).size(ComponentSize::Small)

// Bottom menu
.align_center().dropdown_direction(DropdownDirection::Up).compact()
```

## Next Steps

- Explore [Component Reference](../COMPONENT-REFERENCE.md)
- Check out [Select Improvements](../SELECT-IMPROVEMENTS.md)
- Run demos: `cargo run --example select_compact_demo`

