# Fluix Quick Start

Get started with Fluix in 5 minutes!

## 🚀 Installation

### Create New Project

```bash
cargo new my-app
cd my-app
```

### Add Dependencies

Edit `Cargo.toml`:

```toml
[dependencies]
fluix = "0.1"
gpui = "0.2"
env_logger = "0.11"
```

## 📝 First Application

### 1. Create main.rs

```rust
use gpui::*;
use fluix::prelude::*;

fn main() {
    env_logger::init();
    let app = Application::new();
    
    app.run(move |cx| {
        let window_options = WindowOptions {
            window_bounds: Some(WindowBounds::Windowed(Bounds {
                origin: point(px(100.), px(100.)),
                size: size(px(600.), px(400.)),
            })),
            titlebar: Some(TitlebarOptions {
                title: Some("My First Fluix App".into()),
                ..Default::default()
            }),
            ..Default::default()
        };

        cx.open_window(window_options, |window, cx| {
            cx.new(|cx| MyApp::new(window, cx))
        }).unwrap();
    });
}

struct MyApp {
    counter: usize,
    button: Entity<Button>,
}

impl MyApp {
    fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        let button = cx.new(|_cx| {
            Button::new("Click Me!")
                .variant(ButtonVariant::Primary)
                .size(ComponentSize::Large)
        });
        
        cx.subscribe_in(&button, window, Self::on_button_click).detach();
        
        Self {
            counter: 0,
            button,
        }
    }
    
    fn on_button_click(
        &mut self,
        _button: &Entity<Button>,
        _event: &ButtonEvent,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.counter += 1;
        println!("Button clicked {} times!", self.counter);
        cx.notify();
    }
}

impl Render for MyApp {
    fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
        div()
            .flex()
            .flex_col()
            .items_center()
            .justify_center()
            .size_full()
            .gap_4()
            .child(
                div()
                    .text_2xl()
                    .text_color(rgb(0x333333))
                    .child(format!("Counter: {}", self.counter))
            )
            .child(self.button.clone())
    }
}
```

### 2. Run

```bash
cargo run
```

You should see a window with a button and counter!

## 🎨 Using More Components

### TextInput Example

```rust
use fluix::prelude::*;

// Create text input
let input = cx.new(|cx| {
    TextInput::new(cx)
        .placeholder("Enter your name...")
        .max_length(50)
});

// Subscribe to events
cx.subscribe_in(&input, window, |this, _input, event: &TextInputEvent, _, cx| {
    match event {
        TextInputEvent::Change(value) => {
            println!("Input changed: {}", value);
        }
        TextInputEvent::Submit(value) => {
            println!("Input submitted: {}", value);
        }
        _ => {}
    }
}).detach();

// Use in render
div().child(input.clone())
```

### TextArea Example

```rust
// Create multi-line text editor
let textarea = cx.new(|cx| {
    TextArea::new(cx)
        .placeholder("Type your message...")
        .min_height(100.0)
        .max_height(300.0)
});

// Subscribe to events
cx.subscribe_in(&textarea, window, |this, _ta, event: &TextAreaEvent, _, cx| {
    match event {
        TextAreaEvent::Submit(value) => {
            println!("Message sent: {}", value);
        }
        _ => {}
    }
}).detach();
```

### Button Variants

```rust
// Primary button
Button::new("Primary")
    .variant(ButtonVariant::Primary)

// Secondary button
Button::new("Secondary")
    .variant(ButtonVariant::Secondary)

// Outline button
Button::new("Outline")
    .variant(ButtonVariant::Outline)

// Text button
Button::new("Text")
    .variant(ButtonVariant::Text)

// Danger button
Button::new("Delete")
    .variant(ButtonVariant::Danger)
```

### Button Sizes

```rust
// Different sizes
Button::new("XSmall").size(ComponentSize::XSmall)
Button::new("Small").size(ComponentSize::Small)
Button::new("Medium").size(ComponentSize::Medium)  // Default
Button::new("Large").size(ComponentSize::Large)
Button::new("XLarge").size(ComponentSize::XLarge)

// Full width button
Button::new("Full Width")
    .full_width(true)

// Disabled button
Button::new("Disabled")
    .disabled(true)

// Loading state
Button::new("Loading...")
    .loading(true)
```

## 🎨 Using Theme

```rust
use fluix::theme::*;

// Get theme
let theme = Theme::default();

// Use colors
div().bg(theme.colors.primary)
div().text_color(theme.colors.text)

// Use spacing
div().p(px(Spacing::MD))
div().gap(px(Spacing::SM))

// Use border radius
div().rounded(px(BorderRadius::MD))
```

## 📚 Complete Examples

View the `examples/` directory for more complete examples:

```bash
# View Button example
cargo run --example button_demo

# View TextInput example
cargo run --example text_input_demo
```

## 🔍 Learning Resources

- [README.md](../README.md) - Complete documentation
- [ROADMAP.md](ROADMAP.md) - Component list and development plan
- [CONTRIBUTING.md](../CONTRIBUTING.md) - Component development guide
- [examples/](../examples/) - Example code

## 💡 Tips

1. **Event Subscription**: Use `cx.subscribe_in()` to subscribe to component events
2. **State Management**: Store component `Entity<T>` in struct fields
3. **Styling**: Use GPUI's chained API to set styles
4. **Theme**: Use `Theme` for consistent design

## 🐛 Common Issues

### Q: Window doesn't show?
A: Make sure `app.run()` is called and window is created correctly.

### Q: Component doesn't respond to clicks?
A: Check if events are subscribed correctly and `.detach()` is called.

### Q: Styles don't work?
A: Make sure you're using correct GPUI APIs like `.bg()`, `.text_color()`, etc.

### Q: How to update component state?
A: After modifying state, call `cx.notify()` to trigger re-rendering.

## 🚀 Next Steps

- Explore more components
- Customize themes
- Create complex layouts
- Contribute to Fluix!

---

**Need help?** Check [GitHub Issues](https://github.com/lipish/fluix/issues)
