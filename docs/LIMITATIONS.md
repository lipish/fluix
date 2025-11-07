# Known Limitations

This document records known limitations encountered when using the Fluix component library with the GPUI framework.

## TextArea Component

### Mouse Drag Selection Limitation

**Issue Description:**
Unable to implement precise mouse drag text selection functionality.

**Technical Reason:**
In GPUI 0.2.2, the `position` field in `MouseDownEvent` and `MouseMoveEvent` is of type `Point<Pixels>`, but the internal field of the `Pixels` struct is private:

```rust
pub struct Pixels(f32);  // Field `0` is private
```

This prevents direct access to the mouse's x/y coordinate values to calculate the character index corresponding to the click position.

**Attempted Code:**
```rust
// ❌ Compilation error: field `0` of struct `gpui::Pixels` is private
let x = event.position.x.0;
let y = event.position.y.0;
```

**Current Workaround:**
Implemented the following alternative interaction methods:
- ✅ Click: Clear selection and focus
- ✅ Double-click: Select all text
- ✅ Cmd/Ctrl+A: Keyboard shortcut to select all
- ✅ Selection highlight: Blue background display

**Future Outlook:**
If future versions of GPUI provide any of the following methods, complete mouse selection functionality can be implemented:
1. Make the `Pixels` field public
2. Add `pub fn value(&self) -> f32` method
3. Implement `Deref<Target = f32>` trait
4. Provide dedicated coordinate access API

**Related Links:**
- GPUI GitHub: https://github.com/zed-industries/zed
- GPUI Version: 0.2.2

**Recorded Date:** 2025-10-25

---

## Contributing

If you discover that this limitation has been resolved or have other solutions, please submit an Issue or Pull Request!
