# Fluix Component

Rust UI components for building fantastic cross-platform desktop applications by using GPUI 0.2.

[Get Started](#hello-world) | [Components](#components)

---

### 46+ Components

Comprehensive library of cross-platform desktop UI components for building feature-rich applications.

### High Performance

GPU-accelerated rendering powered by GPUI for smooth user experience.

### Type Safe

Leverage Rust's type system for compile-time type safety guarantees.

### Themeable

Built-in theme system with flexible theme and style customization support.

### Easy to Use

Simple and consistent API design for quick onboarding.

### Well Documented

Comprehensive API documentation, tutorials, and example code.

## Simple and Intuitive API

Get started with just a few lines of code. Stateless components make it easy to build complex UIs.

```rust
Button::new("Click Me")
    .variant(ButtonVariant::Primary)
    .size(ComponentSize::Medium)
    .on_click(|_, _, _| println!("Button clicked!"))
```

## Install Fluix Component

Add the following to your `Cargo.toml`:

```toml
[dependencies]
fluix = "0.1.20"
gpui = "0.2"
```

## Hello World

The following `src/main.rs` is a simple "Hello, World!" application:

```rust
use gpui::*;
use fluix::*;

pub struct HelloWorld;

impl Render for HelloWorld {
    fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
        div()
            .v_flex()
            .gap_2()
            .size_full()
            .items_center()
            .justify_center()
            .child("Hello, World!")
            .child(
                Button::new("click_me")
                    .variant(ButtonVariant::Primary)
                    .size(ComponentSize::Medium)
            )
    }
}

fn main() {
    let app = Application::new()
        .with_assets(fluix::Assets);  // ← Important! Load SVG icons

    app.run(move |cx| {
        cx.spawn(async move |cx| {
            cx.open_window(WindowOptions::default(), |window, cx| {
                cx.new(|_| HelloWorld)
            })?;

            Ok::<_, anyhow::Error>(())
        })
        .detach();
    });
}
```

Run the program:

```bash
$ cargo run
```

> ⚠️ **Work in Progress**: Fluix is currently in early development, APIs may change.  
> 💡 **Important**: When using Fluix, you must call `.with_assets(fluix::Assets)` at application startup to load SVG icon resources!

## Documentation & Tutorials

### 📚 Tutorials

**New to Fluix?** Follow our step-by-step tutorials:

- **[Getting Started](docs/tutorials/01-GETTING-STARTED.md)** ⭐ - Your first Fluix app (30 min)
- **[Working with Components](docs/tutorials/02-COMPONENTS.md)** - All components explained (45 min)
- **[Styling and Theming](docs/tutorials/03-STYLING.md)** - Make it beautiful (30 min)

[View All Tutorials →](docs/tutorials/README.md) | [Documentation Index →](docs/DOCUMENTATION-INDEX.md)

### 📖 API Reference

- **[Component Reference](docs/COMPONENT-REFERENCE.md)** - Complete API reference for all components
- **[Icon Reference](docs/ICON_REFERENCE.md)** - All 31 icons with examples and usage guide
- **[FAQ](docs/FAQ.md)** - Common questions answered
- **[Asset Loading Guide](docs/ASSET_LOADING_GUIDE.md)** - How SVG loading works

## Components

### ✅ Implemented Components

**Basic Components**: Button, Icon  
**Form Components**: TextInput, TextArea, Checkbox, Radio, Select, Combobox  
**Layout Components**: Tabs, Breadcrumb

### 🔄 In Development

See [ROADMAP.md](docs/ROADMAP.md) for detailed development progress and component list.

## Examples

Run example projects:

```bash
# Button component example
cargo run --example button_demo

# Icon component example  
cargo run --example icon_demo

# TextInput and TextArea examples
cargo run --example text_input_demo

# Tabs component example
cargo run --example tabs_demo
```

View more examples: [examples/](examples/)

## Contributing

Contributions are welcome! Please check [ROADMAP.md](docs/ROADMAP.md) for current progress and components to implement.

## Related Links

- [GPUI](https://github.com/zed-industries/zed) - Underlying UI framework
- [API Documentation](https://docs.rs/fluix) - Complete API documentation
- [Example Code](examples/) - More examples

## About

Fluix is a modern Rust UI component library built on top of GPUI 0.2, designed to simplify the development of cross-platform desktop applications. With GPU-accelerated rendering, a comprehensive component set, and type-safe APIs, Fluix empowers developers to build beautiful and performant desktop applications with ease.

### Key Features

- **GPU-Accelerated**: Built on GPUI for smooth, high-performance rendering
- **Type-Safe**: Leverages Rust's type system for compile-time safety
- **Cross-Platform**: Works on macOS, Windows, and Linux
- **Comprehensive**: 46+ components covering basic UI, forms, and layouts
- **Well-Documented**: Extensive tutorials, API documentation, and examples
- **Themeable**: Flexible theming system for customizing application appearance

### Why Fluix?

Fluix bridges the gap between low-level GPUI APIs and high-level component development, providing a consistent and intuitive API for building modern desktop applications. Whether you're building a simple utility or a complex application, Fluix provides the building blocks you need.

## License

MIT License
