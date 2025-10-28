# Frequently Asked Questions (FAQ)

Common questions and answers about Fluix.

## General Questions

### What is Fluix?

Fluix is a modern UI component library for Rust, built on top of GPUI (the UI framework used by Zed Editor). It provides ready-to-use components like buttons, inputs, selects, and icons.

### Why use Fluix?

- ✅ **Ready-to-use components** - No need to build everything from scratch
- ✅ **Consistent design** - All components follow the same design system
- ✅ **Type-safe** - Leverages Rust's type system for safety
- ✅ **GPU-accelerated** - Built on GPUI for high performance
- ✅ **Well-documented** - Comprehensive tutorials and examples

### Is Fluix production-ready?

Fluix is in active development (v0.1.7). While it's functional and used in projects, the API may change. We follow semantic versioning.

## Installation & Setup

### How do I install Fluix?

Add to your `Cargo.toml`:

```toml
[dependencies]
fluix = "0.1.7"
gpui = "0.2"
```

### Do I need to provide SVG icon files?

**No!** All 22 SVG icons are embedded in the Fluix library. Just call `.with_assets(fluix::Assets)` when creating your app.

### Why aren't my icons showing?

Make sure you call `.with_assets(fluix::Assets)`:

```rust
// ❌ Wrong - icons won't show
Application::new().run(|cx| { ... });

// ✅ Correct - icons will show
Application::new()
    .with_assets(fluix::Assets)
    .run(|cx| { ... });
```

### What version of GPUI do I need?

Fluix currently requires GPUI 0.2. Check your `Cargo.toml`:

```toml
[dependencies]
gpui = "0.2"
```

## Components

### How do I change the font size in components?

**For most components**, use the `.size()` method with `ComponentSize`:

```rust
Button::new("Click").size(ComponentSize::Large)   // 16px font
```

Available sizes: `XSmall`, `Small`, `Medium` (default), `Large`, `XLarge`

**For Select component** (NEW!), you can change font size independently:

```rust
// Change font size without changing component height
Select::new(cx)
    .font_size(px(12.))  // 12px font, but keeps 36px height
    .options(vec![...])

// Perfect for matching TextInput font size!
Select::new(cx)
    .font_size(px(12.))
    .options(vec![...])
```

### Can I customize component colors?

Yes!

**For Icons**:
```rust
Icon::new(IconName::Star).color(rgb(0xF59E0B))
```

**For Select background** (NEW!):
```rust
Select::new(cx)
    .bg_color(rgb(0xEFF6FF))  // Light blue background
    .options(vec![...])
```

**For other components**, you can wrap them in styled containers or modify the theme.

### How do I handle button clicks?

Subscribe to button events:

```rust
let button = cx.new(|_| Button::new("Click Me"));

cx.subscribe_in(&button, window, |_, _, event, _, _| {
    match event {
        ButtonEvent::Click => println!("Clicked!"),
    }
}).detach();
```

### How do I get the selected value from a Select?

Subscribe to select events:

```rust
cx.subscribe_in(&select, window, |_, _, event, _, _| {
    match event {
        SelectEvent::Change(value) => {
            println!("Selected: {}", value);
        }
        SelectEvent::MultiChange(values) => {
            println!("Selected: {:?}", values);
        }
    }
}).detach();
```

### Can Select component have multiple selections?

Yes! Use `.multiple(true)`:

```rust
Select::new("languages")
    .multiple(true)
    .options(vec![
        SelectOption::new("rust", "Rust"),
        SelectOption::new("go", "Go"),
    ])
```

### How do I group options in Select?

Use `SelectOptionGroup`:

```rust
Select::new("tech")
    .option_groups(vec![
        SelectOptionGroup::new("Frontend")
            .option(SelectOption::new("react", "React"))
            .option(SelectOption::new("vue", "Vue")),
        SelectOptionGroup::new("Backend")
            .option(SelectOption::new("rust", "Rust"))
            .option(SelectOption::new("go", "Go")),
    ])
```

## Icons

### What icons are available?

Fluix includes 22 icons from Heroicons:

**Navigation**: ArrowLeft, ArrowRight, ArrowUp, ArrowDown, ChevronUpDown, UnfoldMore  
**Actions**: Check, Close, Plus, Minus, Search  
**UI**: Settings, Home, User, Bell, Star, Heart, Menu  
**Status**: Info, Warning, Error, Success

See [Icon Reference](ICON_REFERENCE.md) for details.

### Can I add custom icons?

Currently, you need to add them to the Fluix source. We're working on making this easier. For now, you can:

1. Add your SVG to `assets/icons/`
2. Add to `IconName` enum
3. Add path mapping in `IconName::path()`
4. Rebuild

### How do I change icon size?

```rust
Icon::new(IconName::Star).small()    // 16px
Icon::new(IconName::Star).medium()   // 20px (default)
Icon::new(IconName::Star).large()    // 24px
Icon::new(IconName::Star).xlarge()   // 32px

// Custom size
Icon::new(IconName::Star).size(IconSize::Custom(48.0))
```

### How do I change icon color?

```rust
Icon::new(IconName::Heart).color(rgb(0xFF0000))  // Red
Icon::new(IconName::Star).color(rgb(0xF59E0B))   // Orange
```

## Styling

### How do I customize component appearance?

Fluix components use GPUI's styling system. Wrap components in styled containers:

```rust
div()
    .p_4()
    .bg(rgb(0xF5F5F5))
    .rounded(px(8.))
    .child(Button::new("Click"))
```

### Can I create a custom theme?

Yes! Modify the `Theme` struct:

```rust
let mut theme = Theme::default();
theme.colors.primary = hsla(0.6, 0.5, 0.6, 1.0);  // Custom purple
```

### How do I add spacing between components?

Use GPUI's layout properties:

```rust
div()
    .flex()
    .flex_col()
    .gap_4()  // 16px gap
    .child(Button::new("First"))
    .child(Button::new("Second"))
```

Or use Fluix spacing constants:

```rust
use fluix::theme::Spacing;

div().gap(px(Spacing::MD))  // 12px
```

## Performance

### Is Fluix fast?

Yes! Fluix is built on GPUI, which uses GPU acceleration for rendering. It's designed for high-performance desktop applications.

### Does embedding SVG icons increase binary size?

Yes, but minimally. All 22 icons add about ~50KB to your binary. This is a good tradeoff for zero-dependency icon support.

### Should I worry about re-rendering?

GPUI handles efficient re-rendering automatically. Only changed parts of the UI are re-rendered.

## Development

### How do I run the examples?

```bash
cargo run --example button_demo
cargo run --example icon_demo
cargo run --example select_demo
```

### How do I contribute?

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Add tests and documentation
5. Submit a pull request

See [CONTRIBUTING.md](../CONTRIBUTING.md) for details.

### Where can I get help?

- **Documentation**: [docs/tutorials/](tutorials/)
- **Examples**: [examples/](../examples/)
- **GitHub Issues**: [Report bugs or ask questions](https://github.com/lipish/fluix/issues)

## Troubleshooting

### My app won't compile

Common issues:

1. **Wrong GPUI version**: Make sure you have `gpui = "0.2"`
2. **Missing dependencies**: Run `cargo update`
3. **Outdated Fluix**: Update to latest version

### Components look broken

1. **Check window size**: Make sure your window is large enough
2. **Check styling**: Verify parent containers aren't constraining size
3. **Check GPUI version**: Ensure compatibility

### Icons are blank/missing

1. **Did you call `.with_assets()`?** This is required!
2. **Check icon name**: Make sure you're using a valid `IconName`
3. **Check color**: Icon might be same color as background

### Select dropdown doesn't close

This is a known limitation in multi-select mode. Click the trigger button to close manually. We're working on improving this.

## Roadmap

### What's coming next?

- More components (Table, Modal, Toast, etc.)
- Improved theming system
- Better form validation
- Animation support
- Accessibility features

See [ROADMAP.md](ROADMAP.md) for details.

### Can I request features?

Yes! Open an issue on [GitHub](https://github.com/lipish/fluix/issues) with the "feature request" label.

## License

### What license does Fluix use?

Fluix is licensed under the MIT License. You can use it freely in commercial and open-source projects.

### What about the icons?

The icons are from Heroicons, which is also MIT licensed.

---

**Still have questions?** 

- Check the [tutorials](tutorials/README.md)
- Browse the [examples](../examples/)
- Open an issue on [GitHub](https://github.com/lipish/fluix/issues)

