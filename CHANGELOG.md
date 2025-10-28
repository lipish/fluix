# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.1.14] - 2025-10-28

### Added - Select Component
- **Dropdown Width Control** 📏
  - Match trigger width: Default behavior
  - Fixed width: `.fixed_width(px(n))`
  - Minimum width: `.min_width(px(n))`
  - Maximum width: `.max_width(px(n))`
  - Full control with `.dropdown_width(DropdownWidth)`
  - Four width modes: MatchTrigger, Fixed, MinWidth, MaxWidth

### Examples
- Add `select_width_demo.rs` - Dropdown width demonstration
  - Shows all width modes
  - Demonstrates combinations with compact and alignment

### Documentation
- Update [SELECT-IMPROVEMENTS.md](docs/SELECT-IMPROVEMENTS.md)
  - Add dropdown width section
  - Add usage examples
- Update [COMPONENT-REFERENCE.md](docs/COMPONENT-REFERENCE.md)
  - Add new width methods
  - Add width examples

### Technical Details
- Select: Add `DropdownWidth` enum (MatchTrigger, Fixed, MinWidth, MaxWidth)
- Select: Add `dropdown_width: DropdownWidth` field
- Select: Add `.dropdown_width()`, `.fixed_width()`, `.min_width()`, `.max_width()` methods
- Select: Width-based rendering logic for dropdown menu

### Use Cases
- Narrow action menus: `.fixed_width(px(100.))`
- Wide detailed options: `.fixed_width(px(400.))`
- Responsive design: `.min_width(px(200.))`
- Constrained layouts: `.max_width(px(300.))`
- Combined: `.fixed_width(px(100.)).compact().align_right()`

## [0.1.13] - 2025-10-28

### Added - Icon Component
- **Icon Background Support** 📦
  - Square backgrounds: `.with_square_bg(color)`
  - Rectangular backgrounds: `.with_rect_bg(width, height, color)`
  - Border radius support: `.rounded(radius)`
  - Background color customization: `.bg_color(color)`
  - Three background modes: None (default), Square, Rectangle

### Added - Select Component
- **Compact Mode** 📏
  - `.compact()` method for tighter spacing
  - 50% less vertical space (4px vs 8px padding)
  - Perfect for long lists and grouped options
  - Applies to both items and group labels

- **Dropdown Alignment** 📐
  - Left alignment: `.align_left()` (default)
  - Right alignment: `.align_right()`
  - Center alignment: `.align_center()`
  - Full control with `.dropdown_alignment(DropdownAlignment)`
  - Combine with direction (Up/Down) for complete positioning

### Examples
- Add `select_compact_demo.rs` - Compact spacing demonstration
- Add `select_alignment_demo.rs` - Alignment options demonstration
- Update `icon_send_demo.rs` - Show background features

### Documentation
- Update [SELECT-IMPROVEMENTS.md](docs/SELECT-IMPROVEMENTS.md)
  - Add compact mode section
  - Add alignment control section
- Update [ICON_REFERENCE.md](docs/ICON_REFERENCE.md)
  - Add background support documentation
  - Add usage examples for backgrounds
- Update [COMPONENT-REFERENCE.md](docs/COMPONENT-REFERENCE.md)
  - Add new Icon methods
  - Add new Select methods

### Technical Details
- Icon: Add `IconBackground` enum (None, Square, Rectangle)
- Icon: Add background rendering logic with padding
- Select: Add `compact: bool` field for spacing control
- Select: Add `DropdownAlignment` enum (Left, Right, Center)
- Select: Add `dropdown_alignment` field
- Select: Dynamic padding calculation based on compact mode
- Select: Alignment-based positioning for dropdown overlay

### Use Cases
- Icon backgrounds: Buttons, badges, status indicators
- Compact mode: Long lists (>10 items), grouped options, space-constrained UIs
- Alignment: Right-side toolbars, bottom menus, table actions, centered dialogs

## [0.1.12] - 2025-10-28

### Added
- **Send Icon** 📤
  - New `IconName::Send` icon for send/submit actions
  - Perfect for message sending, form submission, email
  - Available in all sizes (XSmall to XLarge)
  - SVG asset: `assets/icons/send.svg`

### Fixed
- **Demo Scrolling** 📜
  - Fixed `icon_send_demo.rs` - now supports scrolling
  - All content is accessible regardless of window size
  - Verified all 9 main demos have scrolling support

### Documentation
- **Icon Reference** 📚
  - Add [ICON_REFERENCE.md](docs/ICON_REFERENCE.md) - Complete icon reference
  - All 23 icons documented with examples
  - Usage guidelines and color palette
  - Technical details and customization guide

- **Demo Best Practices** 📋
  - Add [DEMO-BEST-PRACTICES.md](docs/DEMO-BEST-PRACTICES.md)
  - Scrolling pattern guidelines
  - Complete demo template
  - Common mistakes and fixes
  - Testing checklist

- **Component Reference Updates**
  - Update icon count: 22 → 23 icons
  - Add Send icon to Actions category
  - Clarify icons are square by default

### Examples
- Add `icon_send_demo.rs` - Send icon demonstration with scrolling

### Technical Details
- Icons are square by default (equal width and height)
- `.size()` method sets both width and height
- All demos follow three-layer scrolling pattern

### Quality Improvements
- All main demos verified for scrolling support
- Consistent demo structure across examples
- Better documentation for contributors

## [0.1.11] - 2025-10-28

### Added
- **Select Component Variant System** 🎨
  - New `SelectVariant` enum: Default, Ghost, Outline
  - Ghost variant: no border, transparent background
  - Outline variant: border only, transparent background
  - `.variant()` method to set visual variant

- **Dropdown Direction Control** ⬆️⬇️
  - New `DropdownDirection` enum: Down, Up, Auto
  - `.dropdown_direction()` method to control expansion direction
  - Support for upward expansion (perfect for bottom toolbars)

- **Enhanced Convenience Methods** 🛠️
  - `.no_border()` - Remove border
  - `.no_shadow()` - Remove shadow ⭐ NEW
  - `.transparent()` - Transparent background
  - `.clean()` - All-in-one: no border + no shadow + transparent ⭐ NEW
  - `.border_color()` - Custom border color

### Examples
- Add `select_variants_demo.rs` - Comprehensive variants and directions demo
- Add `select_clean_demo.rs` - Demonstrates no_border + no_shadow combinations

### Documentation
- Add [SELECT-IMPROVEMENTS.md](docs/SELECT-IMPROVEMENTS.md) - Complete improvement guide
- Update Component Reference with all new methods
- Update tutorials with variant examples

### Technical Details
- Select: Add `variant: SelectVariant` field
- Select: Add `dropdown_direction: DropdownDirection` field
- Select: Add `show_shadow: bool` field
- Select: Add `custom_border_color: Option<Rgba>` field
- Conditional rendering for borders and shadows
- Upward dropdown positioning support

### Use Cases
- Embedded selects in Settings views (Ghost variant)
- Bottom toolbar selects (Upward expansion)
- Flat design (no_shadow)
- Minimal design (clean method)

## [0.1.10] - 2025-10-28

### Added
- **Text Color Customization for Select Component** 🎨
  - New `.text_color(color)` method for Select component
  - Customize text color with any RGB/RGBA color
  - Perfect for dark themes and semantic colors
  - Works seamlessly with `.bg_color()` for complete theming
  - Fully backward compatible

### Fixed
- **Scrolling Support in All Demos** 📜
  - Add scrolling to `select_bg_color_demo.rs`
  - Add scrolling to `select_font_size_demo.rs`
  - All components now visible even in smaller windows
  - Consistent scroll behavior across all demos

### Changed
- Update `select_bg_color_demo.rs` to showcase dark theme with white text
- Improve demo user experience with proper scroll containers

### Technical Details
- Select component: Added `custom_text_color: Option<Rgba>` field
- Select component: Added `text_color(color: Rgba)` method
- Rendering updated to use custom text color for placeholder and selected values
- All demos now use `ScrollHandle` and `.overflow_y_scroll()` pattern

### Examples
- Enhanced `select_bg_color_demo.rs` with dark theme example
- All demos now support scrolling for better UX

## [0.1.9] - 2025-10-28

### Added
- **Background Color Customization for Select Component** 🎨
  - New `.bg_color(color)` method for Select component
  - Customize background color with any RGB/RGBA color
  - Perfect for semantic colors (success, warning, error, info)
  - Fully backward compatible

### Examples
- Add `select_bg_color_demo.rs` - Demonstrates background color customization

### Technical Details
- Select component: Added `custom_bg_color: Option<Rgba>` field
- Select component: Added `bg_color(color: Rgba)` method
- Rendering updated to use custom background color when provided

## [0.1.8] - 2025-10-28

### Added
- **Independent Font Size Control for Select Component** 🎉
  - New `.font_size(px)` method for Select component
  - Change font size without changing component height
  - Perfect for matching TextInput font size (12px)
  - Fully backward compatible

- **Comprehensive Tutorial System**
  - 3 complete tutorials (Getting Started, Components, Styling)
  - Component Reference with complete API documentation
  - Icon Reference with all 22 icons
  - FAQ with 40+ questions answered
  - Documentation Index for easy navigation
  - 100+ code examples across all docs

- **New Example**
  - `select_font_size_demo.rs` - Demonstrates independent font size control

### Changed
- Select component now supports custom font size via `.font_size()`
- Updated all documentation with new font size feature
- Improved documentation structure and navigation

### Documentation
- Added [Getting Started Tutorial](docs/tutorials/01-GETTING-STARTED.md)
- Added [Working with Components Tutorial](docs/tutorials/02-COMPONENTS.md)
- Added [Styling and Theming Tutorial](docs/tutorials/03-STYLING.md)
- Added [Component Reference](docs/COMPONENT-REFERENCE.md)
- Added [Icon Reference](docs/ICON_REFERENCE.md)
- Added [FAQ](docs/FAQ.md)
- Added [Documentation Index](docs/DOCUMENTATION-INDEX.md)
- Added [Select Font Size Feature Guide](docs/SELECT-FONT-SIZE-FEATURE.md)

### Technical Details
- Select component internal: Added `custom_font_size: Option<Pixels>` field
- Rendering logic updated to use custom font size when provided
- ~2,000 lines of new documentation
- 9 new documentation files

## [0.1.7] - 2025-10-28

### Added
- **Icon Component with SVG Support** 🎉
  - 22 SVG icons from Heroicons
  - Asset loading using `rust-embed`
  - Customizable size (Small, Medium, Large, XLarge, Custom)
  - Customizable color
  - Icons: ArrowLeft, ArrowRight, ArrowUp, ArrowDown, Check, ChevronUpDown, Close, Plus, Minus, Search, Settings, Home, User, Bell, Star, Heart, Menu, Info, Warning, Error, Success, UnfoldMore

- **Asset Loading System**
  - `AssetSource` implementation for loading embedded resources
  - `fluix::Assets` for automatic SVG embedding
  - Comprehensive asset loading documentation

- **Select Component Improvements**
  - ChevronUpDown icon for dropdown indicator
  - Better visual feedback
  - Improved icon sizing

- **Documentation**
  - `ASSET_LOADING_GUIDE.md` - Complete guide for loading assets in GPUI
  - `ICON_REFERENCE.md` - Complete icon reference with examples
  - `ICON_SOLUTION_SUMMARY.md` - SVG loading solution explanation
  - `ZED_ICON_PATTERN.md` - Learning from Zed's implementation
  - `FINAL_SUMMARY.md` - Project summary and achievements
  - Updated README with asset loading instructions

- **Examples**
  - `icon_demo.rs` - Showcase all 22 icons with scrollable view
  - Updated `select_demo.rs` with new ChevronUpDown icon

### Changed
- Select component now uses ChevronUpDown icon instead of ArrowDown
- Icon demo now scrollable to view all icons
- Updated all examples to use `.with_assets(fluix::Assets)`

### Fixed
- SVG icons now display correctly using embedded assets
- Scrolling in icon demo for better UX

### Technical Details
- Added `rust-embed = "8"` dependency
- Created `assets/icons/` directory with 22 SVG files
- Implemented `AssetSource` trait for resource loading
- All icons embedded at compile time for zero-dependency deployment

## [0.1.6] - 2025-10-27

### Added
- Select component with multiple selection support
- Select component with grouped options
- Checkbox component for multi-select
- Tag component for selected items display

### Changed
- Improved Select component styling
- Better dropdown positioning

## [0.1.5] - 2025-10-26

### Added
- Initial Select component implementation
- Basic form components

## [0.1.0] - 2025-10-25

### Added
- Initial release
- Button component
- Basic layout components
- Theme system
- Component documentation

[0.1.7]: https://github.com/lipish/fluix/compare/v0.1.6...v0.1.7
[0.1.6]: https://github.com/lipish/fluix/compare/v0.1.5...v0.1.6
[0.1.5]: https://github.com/lipish/fluix/compare/v0.1.0...v0.1.5
[0.1.0]: https://github.com/lipish/fluix/releases/tag/v0.1.0

