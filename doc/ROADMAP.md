# Fluix Component Library Roadmap

Fluix is a Rust UI component library based on GPUI 0.2, aiming to provide a complete and easy-to-use component collection.

## Project Goals

- Provide a complete component library
- Maintain simple and high-performance implementation
- Support theme customization and style extension
- Provide detailed examples and documentation

## Component Categories and Implementation Status

### ✅ Implemented Components (2)

#### Form Components
- [x] **TextInput** - Single-line text input field
  - Supports placeholder, password mode, validation, max length
  - File: `src/text_input.rs`
  
- [x] **TextArea** - Multi-line text editor
  - Supports auto-expanding height, Shift+Enter for newline, Enter to submit
  - Cursor position tracking and correct display
  - File: `src/text_area.rs`

---

### 📋 Components To Implement (44)

#### Basic Components - 19 components

- [ ] **Accordion** - Collapsible content panel
- [ ] **Alert** - Alert messages with different variants
- [ ] **Avatar** - User avatar (with fallback text)
- [ ] **Badge** - Count badge and indicator
- [ ] **Button** - Interactive buttons with multiple variants
  - Primary, Secondary, Outline, Text
  - Supports icons, loading state, disabled state
- [ ] **Checkbox** - Checkbox control
- [ ] **Icon** - Icon display component
  - Integrated Lucide icons
- [ ] **Image** - Image display (with fallback)
- [ ] **Indicator** - Loading and status indicators
- [ ] **Kbd** - Keyboard shortcut display
- [ ] **Label** - Form element text label
- [ ] **Progress** - Progress bar
- [ ] **Radio** - Radio button (single selection)
- [ ] **Skeleton** - Loading placeholder
- [ ] **Slider** - Range selection slider
- [ ] **Switch** - Toggle switch control
- [ ] **Tag** - Tags and categories
- [ ] **Toggle** - Toggle button state
- [ ] **Tooltip** - Hover tooltip

#### Form Components - 6 components

- [ ] **ColorPicker** - Color selection interface
- [ ] **DatePicker** - Date selection (with calendar)
- [ ] **Dropdown** - Dropdown selection
- [ ] **Form** - Form container and layout
- [ ] **NumberInput** - Number input (with increment/decrement)
- [ ] **OtpInput** - One-time password input
- [ ] **Editor** - Code editor

#### Layout Components - 9 components

- [ ] **DescriptionList** - Key-value pair display
- [ ] **Drawer** - Panel that slides in from edge
- [ ] **GroupBox** - Grouped content with border
- [ ] **Modal** - Dialog and modal window
- [ ] **Notification** - Toast notification
- [ ] **Popover** - Floating content display
- [ ] **Resizable** - Resizable panel
- [ ] **Scrollable** - Scrollable container
- [ ] **Sidebar** - Navigation sidebar

#### Advanced Components - 10 components

- [ ] **Calendar** - Calendar display and navigation
- [ ] **Chart** - Data visualization charts
  - Line, Bar, Area, Pie
- [ ] **List** - List display
- [ ] **PopupMenu** - Menu and context menu
- [ ] **Table** - High-performance data table
- [ ] **Tabs** - Tab interface
- [ ] **Tree** - Hierarchical tree data display
- [ ] **VirtualList** - Virtualized list (large datasets)
- [ ] **WebView** - Embedded web browser

---

## Implementation Plan

### Phase 1: Core Basic Components (Priority: High)

**Goal**: Provide the most commonly used basic UI elements

1. **Button** - Various button styles
2. **Icon** - Icon support
3. **Label** - Text label
4. **Checkbox** - Checkbox
5. **Radio** - Radio button
6. **Switch** - Toggle switch
7. **Badge** - Badge
8. **Tag** - Tag

**Estimated Time**: 2-3 weeks

### Phase 2: Form Components (Priority: High)

**Goal**: Complete form input capabilities

1. **Dropdown** - Dropdown selection
2. **Form** - Form container
3. **NumberInput** - Number input
4. **ColorPicker** - Color picker
5. **DatePicker** - Date picker

**Estimated Time**: 2-3 weeks

### Phase 3: Feedback Components (Priority: Medium)

**Goal**: User feedback and interaction hints

1. **Alert** - Alert message
2. **Tooltip** - Tooltip
3. **Modal** - Modal dialog
4. **Notification** - Notification
5. **Progress** - Progress bar
6. **Indicator** - Loading indicator
7. **Skeleton** - Skeleton screen

**Estimated Time**: 2 weeks

### Phase 4: Layout Components (Priority: Medium)

**Goal**: Page layout and containers

1. **Drawer** - Drawer
2. **Sidebar** - Sidebar
3. **Tabs** - Tabs
4. **Accordion** - Accordion
5. **GroupBox** - Group box
6. **Resizable** - Resizable
7. **Scrollable** - Scrollable container

**Estimated Time**: 2-3 weeks

### Phase 5: Data Display Components (Priority: Medium-Low)

**Goal**: Complex data display

1. **Table** - Data table
2. **List** - List
3. **VirtualList** - Virtual list
4. **Tree** - Tree component
5. **Calendar** - Calendar
6. **DescriptionList** - Description list

**Estimated Time**: 3-4 weeks

### Phase 6: Advanced Components (Priority: Low)

**Goal**: Advanced feature components

1. **Chart** - Chart component
2. **PopupMenu** - Popup menu
3. **Popover** - Popover card
4. **WebView** - Web view
5. **Editor** - Code editor
6. **OtpInput** - OTP input

**Estimated Time**: 4-5 weeks

---

## Directory Structure Plan

```
crates/fluix/
├── src/
│   ├── lib.rs                    # Library entry point
│   ├── theme.rs                  # Theme system
│   ├── components/
│   │   ├── mod.rs                # Component module
│   │   ├── basic/                # Basic components
│   │   │   ├── mod.rs
│   │   │   ├── button.rs
│   │   │   ├── icon.rs
│   │   │   ├── badge.rs
│   │   │   ├── checkbox.rs
│   │   │   ├── radio.rs
│   │   │   ├── switch.rs
│   │   │   ├── tag.rs
│   │   │   ├── label.rs
│   │   │   ├── avatar.rs
│   │   │   ├── kbd.rs
│   │   │   ├── progress.rs
│   │   │   ├── slider.rs
│   │   │   ├── skeleton.rs
│   │   │   ├── tooltip.rs
│   │   │   ├── toggle.rs
│   │   │   ├── image.rs
│   │   │   ├── indicator.rs
│   │   │   ├── alert.rs
│   │   │   └── accordion.rs
│   │   ├── form/                 # Form components
│   │   │   ├── mod.rs
│   │   │   ├── text_input.rs     # ✅ Implemented
│   │   │   ├── text_area.rs      # ✅ Implemented
│   │   │   ├── dropdown.rs
│   │   │   ├── form.rs
│   │   │   ├── number_input.rs
│   │   │   ├── color_picker.rs
│   │   │   ├── date_picker.rs
│   │   │   ├── otp_input.rs
│   │   │   └── editor.rs
│   │   ├── layout/               # Layout components
│   │   │   ├── mod.rs
│   │   │   ├── drawer.rs
│   │   │   ├── modal.rs
│   │   │   ├── sidebar.rs
│   │   │   ├── tabs.rs
│   │   │   ├── group_box.rs
│   │   │   ├── resizable.rs
│   │   │   ├── scrollable.rs
│   │   │   ├── popover.rs
│   │   │   ├── notification.rs
│   │   │   └── description_list.rs
│   │   └── advanced/             # Advanced components
│   │       ├── mod.rs
│   │       ├── table.rs
│   │       ├── list.rs
│   │       ├── virtual_list.rs
│   │       ├── tree.rs
│   │       ├── calendar.rs
│   │       ├── chart.rs
│   │       ├── popup_menu.rs
│   │       └── webview.rs
│   └── utils/                    # Utility functions
│       ├── mod.rs
│       ├── colors.rs             # Color utilities
│       └── icons.rs              # Icon utilities
├── examples/
│   ├── text_input_demo.rs        # ✅ Existing
│   ├── button_demo.rs
│   ├── form_demo.rs
│   ├── layout_demo.rs
│   └── showcase.rs               # All components showcase
├── Cargo.toml
├── README.md
└── ROADMAP.md                    # This file

```

---

## Design Principles

1. **Consistency**: All components use unified API design patterns
2. **Extensibility**: Support custom styles and themes
3. **High Performance**: Fully leverage GPUI's GPU acceleration capabilities
4. **Type Safety**: Fully leverage Rust's type system
5. **Well Documented**: Each component has detailed examples and documentation

## API Design Standards

### Component Creation Pattern
```rust
// Builder pattern
let button = Button::new("Click me")
    .variant(ButtonVariant::Primary)
    .size(Size::Medium)
    .disabled(false)
    .on_click(|cx| { /* handler */ });
```

### Event Handling Pattern
```rust
// Using EventEmitter trait
pub enum ButtonEvent {
    Click,
    DoubleClick,
}

impl EventEmitter<ButtonEvent> for Button {}
```

### Style Customization Pattern
```rust
// Using theme system
button.theme(|theme| {
    theme.primary_color(rgb(0x696FC7))
         .border_radius(px(6.))
})
```

---

## Next Steps

### Immediate Actions (This Week)

1. **Refactor Existing Components**
   - Move `text_input.rs` to `src/components/form/`
   - Move `text_area.rs` to `src/components/form/`
   - Create new directory structure

2. **Create Theme System**
   - Define unified color scheme
   - Create size standards (Size enum)
   - Create spacing standards

3. **Implement Button Component**
   - As the first new component
   - Establish standard component implementation pattern
   - Create complete examples

### Short-term Goals (1-2 weeks)

- Complete first 3 components of Phase 1 (Button, Icon, Label)
- Create unified showcase example
- Write component development guide documentation

### Medium-term Goals (1-2 months)

- Complete all components in Phase 1 and Phase 2
- Provide complete form solution
- Release v0.1.0 version

### Long-term Goals (3-6 months)

- Complete all 46 components
- Reach production-ready status
- Release v1.0.0 version

---

## Contributing Guidelines

Contributions are welcome! When implementing new components, please follow:

1. Keep API design simple and consistent
2. Maintain consistent code style
3. Write detailed examples
4. Add unit tests (if applicable)
5. Update this ROADMAP document

---

**Last Updated**: 2025-10-25
**Current Version**: v0.1.0-dev
**Implemented Components**: 2/46 (4.3%)
