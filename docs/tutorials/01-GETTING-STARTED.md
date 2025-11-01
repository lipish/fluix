# Getting Started with Fluix

Welcome to Fluix! This tutorial will guide you through creating your first GPUI application with Fluix components.

## 📋 Prerequisites

- Rust 1.70 or later
- Basic knowledge of Rust
- Familiarity with UI concepts

## 🚀 Installation

### Step 1: Create a New Project

```bash
cargo new my-fluix-app
cd my-fluix-app
```

### Step 2: Add Dependencies

Add Fluix to your `Cargo.toml`:

```toml
[dependencies]
fluix = "0.1.7"
gpui = "0.2"
```

### Step 3: Set Up Your Main File

Replace the contents of `src/main.rs`:

```rust
use fluix::*;
use gpui::*;

fn main() {
    // Initialize logging (optional but helpful)
    env_logger::init();

    // Create application with embedded assets
    let app = Application::new()
        .with_assets(fluix::Assets);  // ← IMPORTANT: Load SVG icons!
    
    // Run the application
    app.run(|cx| {
        // Define window options
        let window_options = WindowOptions {
            window_bounds: Some(WindowBounds::Windowed(Bounds {
                origin: point(px(100.), px(100.)),
                size: size(px(800.), px(600.)),
            })),
            titlebar: Some(TitlebarOptions {
                title: Some("My Fluix App".into()),
                appears_transparent: false,
                ..Default::default()
            }),
            ..Default::default()
        };

        // Open a window
        cx.open_window(window_options, |window, cx| {
            cx.new(|cx| MyApp::new(window, cx))
        }).unwrap();
    });
}

// Your main application view
struct MyApp;

impl MyApp {
    fn new(_window: &mut Window, _cx: &mut Context<Self>) -> Self {
        Self
    }
}

impl Render for MyApp {
    fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
        div()
            .size_full()
            .flex()
            .items_center()
            .justify_center()
            .bg(rgb(0xF5F5F5))
            .child(
                div()
                    .text_2xl()
                    .font_weight(FontWeight::BOLD)
                    .text_color(rgb(0x333333))
                    .child("Hello, Fluix!")
            )
    }
}
```

### Step 4: Run Your App

```bash
cargo run
```

You should see a window with "Hello, Fluix!" in the center! 🎉

## 🎯 Understanding the Code

### The Application Setup

```rust
let app = Application::new()
    .with_assets(fluix::Assets);  // Load embedded SVG icons
```

**Important**: Always call `.with_assets(fluix::Assets)` to enable SVG icons!

### The Window Options

```rust
let window_options = WindowOptions {
    window_bounds: Some(WindowBounds::Windowed(Bounds {
        origin: point(px(100.), px(100.)),  // Position on screen
        size: size(px(800.), px(600.)),     // Window size
    })),
    titlebar: Some(TitlebarOptions {
        title: Some("My Fluix App".into()),  // Window title
        appears_transparent: false,
        ..Default::default()
    }),
    ..Default::default()
};
```

### The View Component

```rust
struct MyApp;

impl Render for MyApp {
    fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
        // Your UI goes here
    }
}
```

Every view in GPUI implements the `Render` trait.

## 🎨 Adding Your First Component

Let's add a button! Update your `MyApp`:

```rust
use fluix::*;
use gpui::*;

struct MyApp {
    button: Entity<Button>,
}

impl MyApp {
    fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        // Create a button
        let button = cx.new(|_| {
            Button::new("Click Me!")
                .variant(ButtonVariant::Primary)
                .size(ComponentSize::Large)
        });

        // Subscribe to button events
        cx.subscribe_in(&button, window, Self::on_button_click).detach();

        Self { button }
    }

    fn on_button_click(
        &mut self,
        _button: &Entity<Button>,
        _event: &ButtonEvent,
        _window: &mut Window,
        _cx: &mut Context<Self>,
    ) {
        println!("Button clicked! 🎉");
    }
}

impl Render for MyApp {
    fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
        div()
            .size_full()
            .flex()
            .items_center()
            .justify_center()
            .bg(rgb(0xF5F5F5))
            .child(self.button.clone())
    }
}
```

Run it again and click the button - you'll see "Button clicked! 🎉" in the console!

## 📚 What's Next?

Now that you have a basic app running, explore these tutorials:

1. [Working with Components](./02-COMPONENTS.md) - Learn about all available components
2. [Styling and Theming](./03-STYLING.md) - Customize the look and feel
3. [Event Handling](./04-EVENTS.md) - Handle user interactions
4. [Building Forms](./05-FORMS.md) - Create interactive forms

## 💡 Common Issues

### Icons Not Showing?

Make sure you called `.with_assets(fluix::Assets)`:

```rust
// ❌ Wrong
Application::new().run(|cx| { ... });

// ✅ Correct
Application::new()
    .with_assets(fluix::Assets)
    .run(|cx| { ... });
```

### Window Not Appearing?

Check that you're calling `cx.open_window()` inside the `app.run()` closure.

### Compilation Errors?

Make sure you have the correct GPUI version:

```toml
[dependencies]
gpui = "0.2"
fluix = "0.1.7"
```

## 🎓 Learning Resources

- [GPUI Documentation](https://www.gpui.rs/)
- [Fluix Examples](../../examples/)
- [Component Reference](../COMPONENT-REFERENCE.md)

## 🤝 Need Help?

- Check the [FAQ](../FAQ.md)
- Browse [Examples](../../examples/)
- Open an issue on [GitHub](https://github.com/lipish/fluix/issues)

---

**Next**: [Working with Components →](./02-COMPONENTS.md)

