# Release v0.1.22

## 🔐 Password Masking Modes

### Added - TextInput Password Masking
- **Password Mask Modes** 🔐
  - `PasswordMaskMode::All` - Mask all characters with bullets (•) (default)
  - `PasswordMaskMode::Partial` - Show first and last few characters, mask the middle
    - Example: Password `f26612345678944u9` displays as `f2••••••••••••••44u9` with prefix_len=2, suffix_len=2
  - New method `.password_mask_mode()` to set masking mode
  - New method `.toggle_password_visibility()` to toggle password visibility programmatically
  - New method `.show_password()` to set initial password visibility state
- All password masking logic properly handles cursor positioning, text selection, and mouse interactions

## 📚 Documentation Improvements

### Improved - Documentation
- **Comprehensive Rustdoc Documentation** 📚
  - Added detailed documentation comments (all in English) for all TextInput methods
  - Added extensive examples and usage guides for each method
  - Improved documentation for `PasswordMaskMode` enum with examples
  - Enhanced `TextInputEvent` documentation with detailed explanations
  - Complete API documentation now available on [docs.rs](https://docs.rs/fluix/0.1.22/fluix/)

## 📦 Installation

```toml
[dependencies]
fluix = "0.1.22"
```

## 🔗 Links

- [Documentation](https://docs.rs/fluix/0.1.22/fluix/)
- [GitHub Repository](https://github.com/lipish/fluix)
- [Component Reference](https://github.com/lipish/fluix/blob/main/docs/COMPONENT-REFERENCE.md)

