# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

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

