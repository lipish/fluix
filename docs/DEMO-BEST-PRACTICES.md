# Demo Best Practices

Guidelines for creating demos and examples in Fluix.

## ⚠️ Critical: Always Add Scrolling Support

**All demos MUST support scrolling** to ensure all content is visible, regardless of window size.

### ❌ Bad: No Scrolling

```rust
impl Render for MyDemo {
    fn render(&mut self, _: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .size_full()
            .flex()
            .flex_col()
            .p_8()
            .gap_8()
            .child(/* content */)
            .child(/* more content */)
            .child(/* even more content */)
            // ❌ Content may be cut off if window is too small!
    }
}
```

### ✅ Good: With Scrolling

```rust
struct MyDemo {
    scroll_handle: ScrollHandle,  // ✅ Add scroll handle
}

impl MyDemo {
    fn new(_window: &mut Window, _cx: &mut Context<Self>) -> Self {
        Self {
            scroll_handle: ScrollHandle::new(),  // ✅ Initialize
        }
    }
}

impl Render for MyDemo {
    fn render(&mut self, _: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .size_full()
            .overflow_hidden()  // ✅ Hide overflow on outer container
            .child(
                div()
                    .id("scroll-container")  // ✅ Unique ID
                    .size_full()
                    .overflow_y_scroll()  // ✅ Enable vertical scrolling
                    .track_scroll(&self.scroll_handle)  // ✅ Track scroll position
                    .child(
                        div()
                            .flex()
                            .flex_col()
                            .p_8()
                            .gap_8()
                            .child(/* content */)
                            .child(/* more content */)
                            .child(/* even more content */)
                            // ✅ All content is accessible via scrolling!
                    )
            )
    }
}
```

## Scrolling Pattern Breakdown

### 1. Add ScrollHandle Field

```rust
struct MyDemo {
    scroll_handle: ScrollHandle,  // Required for scroll tracking
}
```

### 2. Initialize in Constructor

```rust
impl MyDemo {
    fn new(_window: &mut Window, _cx: &mut Context<Self>) -> Self {
        Self {
            scroll_handle: ScrollHandle::new(),
        }
    }
}
```

### 3. Three-Layer Structure

```rust
div()
    .size_full()
    .overflow_hidden()              // Layer 1: Outer container (hides overflow)
    .child(
        div()
            .id("scroll-container")  // Layer 2: Scroll container (unique ID)
            .size_full()
            .overflow_y_scroll()     // Enable scrolling
            .track_scroll(&self.scroll_handle)
            .child(
                div()
                    .flex()          // Layer 3: Content container
                    .flex_col()
                    .p_8()
                    .gap_8()
                    .child(/* your content here */)
            )
    )
```

## When to Add Scrolling

### ✅ Always Add Scrolling For:

- **Component demos** with multiple examples
- **Feature showcases** with many sections
- **Documentation demos** with explanations
- **Comparison demos** showing different variants
- **Any demo with 3+ sections**

### ⚠️ Optional For:

- **Simple tests** with minimal content (e.g., `simple_button.rs`)
- **Technical tests** (e.g., `svg_test.rs`, `color_test.rs`)
- **Single-component tests** that fit in a small window

## Complete Example Template

```rust
use fluix::*;
use gpui::*;

fn main() {
    env_logger::init();

    let app = Application::new().with_assets(fluix::Assets);

    app.run(move |cx| {
        let window_options = WindowOptions {
            window_bounds: Some(WindowBounds::Windowed(Bounds {
                origin: point(px(100.), px(100.)),
                size: size(px(800.), px(600.)),
            })),
            titlebar: Some(TitlebarOptions {
                title: Some("My Demo".into()),
                appears_transparent: false,
                ..Default::default()
            }),
            ..Default::default()
        };

        cx.open_window(window_options, |window, cx| {
            cx.new(|cx| MyDemo::new(window, cx))
        })
        .unwrap();
    });
}

struct MyDemo {
    scroll_handle: ScrollHandle,  // ✅ Required
    // ... other fields
}

impl MyDemo {
    fn new(_window: &mut Window, cx: &mut Context<Self>) -> Self {
        Self {
            scroll_handle: ScrollHandle::new(),  // ✅ Initialize
            // ... initialize other fields
        }
    }
}

impl Render for MyDemo {
    fn render(&mut self, _: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .size_full()
            .overflow_hidden()  // ✅ Layer 1
            .child(
                div()
                    .id("scroll-container")  // ✅ Layer 2
                    .size_full()
                    .overflow_y_scroll()
                    .track_scroll(&self.scroll_handle)
                    .child(
                        div()  // ✅ Layer 3
                            .flex()
                            .flex_col()
                            .bg(rgb(0xF5F5F5))
                            .p_8()
                            .gap_8()
                            .child(
                                // Header
                                div()
                                    .flex()
                                    .flex_col()
                                    .gap_2()
                                    .child(
                                        div()
                                            .text_2xl()
                                            .font_weight(FontWeight::BOLD)
                                            .text_color(rgb(0x111827))
                                            .child("My Demo Title")
                                    )
                                    .child(
                                        div()
                                            .text_sm()
                                            .text_color(rgb(0x6B7280))
                                            .child("Demo description")
                                    )
                            )
                            .child(
                                // Section 1
                                div()
                                    .flex()
                                    .flex_col()
                                    .w_full()
                                    .max_w(px(700.))
                                    .p_6()
                                    .bg(rgb(0xFFFFFF))
                                    .border_1()
                                    .border_color(rgb(0xE5E7EB))
                                    .rounded(px(12.))
                                    .gap_6()
                                    .child(/* section content */)
                            )
                            // ... more sections
                    )
            )
    }
}
```

## Checklist for New Demos

Before submitting a new demo, verify:

- [ ] `ScrollHandle` field added to struct
- [ ] `ScrollHandle::new()` called in constructor
- [ ] Three-layer structure implemented:
  - [ ] Outer: `.size_full().overflow_hidden()`
  - [ ] Middle: `.id("scroll-container").size_full().overflow_y_scroll().track_scroll()`
  - [ ] Inner: Content container with `.flex().flex_col()`
- [ ] Tested with small window size (e.g., 600×400)
- [ ] All content is accessible via scrolling

## Current Demo Status

### ✅ Demos with Scrolling

- `button_demo.rs`
- `icon_demo.rs`
- `icon_send_demo.rs` ⭐ Fixed
- `select_bg_color_demo.rs`
- `select_clean_demo.rs`
- `select_demo.rs`
- `select_font_size_demo.rs`
- `select_variants_demo.rs`
- `text_input_demo.rs`

### ⚠️ Test Files (Scrolling Optional)

- `simple_button.rs` - Simple test, minimal content
- `color_test.rs` - Technical test
- `svg_test.rs` - Technical test
- `svg_path_test.rs` - Technical test

## Common Mistakes

### ❌ Mistake 1: Forgetting ScrollHandle

```rust
struct MyDemo {
    // ❌ Missing scroll_handle field
}
```

### ❌ Mistake 2: Missing overflow_hidden

```rust
div()
    .size_full()
    // ❌ Missing .overflow_hidden()
    .child(
        div()
            .overflow_y_scroll()
            // ...
    )
```

### ❌ Mistake 3: Missing ID

```rust
div()
    .size_full()
    // ❌ Missing .id("scroll-container")
    .overflow_y_scroll()
    .track_scroll(&self.scroll_handle)
```

### ❌ Mistake 4: Wrong Layer Order

```rust
div()
    .overflow_y_scroll()  // ❌ Wrong: scroll on outer layer
    .child(
        div()
            .size_full()
            .overflow_hidden()  // ❌ Wrong: hidden on inner layer
            // ...
    )
```

## Testing Scrolling

### Manual Test

1. Run the demo
2. Resize window to small size (e.g., 400×300)
3. Verify all content is accessible via scrolling
4. Check scroll bar appears when needed

### Visual Indicators

Add a footer to verify scrolling works:

```rust
.child(
    div()
        .p_4()
        .bg(rgb(0xDCFCE7))
        .border_1()
        .border_color(rgb(0x86EFAC))
        .rounded(px(8.))
        .child(
            div()
                .text_sm()
                .text_color(rgb(0x166534))
                .child("✅ If you can see this, scrolling works!")
        )
)
```

## See Also

- [Component Reference](COMPONENT-REFERENCE.md)
- [Examples](../examples/)
- [Tutorials](tutorials/README.md)

