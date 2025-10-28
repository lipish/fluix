# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

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

