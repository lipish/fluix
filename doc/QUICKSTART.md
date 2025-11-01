# Fluix 快速开始

5 分钟内开始使用 Fluix！

## 🚀 安装

### 创建新项目

```bash
cargo new my-app
cd my-app
```

### 添加依赖

编辑 `Cargo.toml`:

```toml
[dependencies]
fluix = "0.1"
gpui = "0.2"
env_logger = "0.11"
```

## 📝 第一个应用

### 1. 创建 main.rs

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

### 2. 运行

```bash
cargo run
```

你应该看到一个窗口，里面有一个按钮和计数器！

## 🎨 使用更多组件

### TextInput 示例

```rust
use fluix::prelude::*;

// 创建文本输入
let input = cx.new(|cx| {
    TextInput::new(cx)
        .placeholder("Enter your name...")
        .max_length(50)
});

// 订阅事件
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

// 在 render 中使用
div().child(input.clone())
```

### TextArea 示例

```rust
// 创建多行文本编辑器
let textarea = cx.new(|cx| {
    TextArea::new(cx)
        .placeholder("Type your message...")
        .min_height(100.0)
        .max_height(300.0)
});

// 订阅事件
cx.subscribe_in(&textarea, window, |this, _ta, event: &TextAreaEvent, _, cx| {
    match event {
        TextAreaEvent::Submit(value) => {
            println!("Message sent: {}", value);
        }
        _ => {}
    }
}).detach();
```

### Button 变体

```rust
// Primary 按钮
Button::new("Primary")
    .variant(ButtonVariant::Primary)

// Secondary 按钮
Button::new("Secondary")
    .variant(ButtonVariant::Secondary)

// Outline 按钮
Button::new("Outline")
    .variant(ButtonVariant::Outline)

// Text 按钮
Button::new("Text")
    .variant(ButtonVariant::Text)

// Danger 按钮
Button::new("Delete")
    .variant(ButtonVariant::Danger)
```

### Button 尺寸

```rust
// 不同尺寸
Button::new("XSmall").size(ComponentSize::XSmall)
Button::new("Small").size(ComponentSize::Small)
Button::new("Medium").size(ComponentSize::Medium)  // 默认
Button::new("Large").size(ComponentSize::Large)
Button::new("XLarge").size(ComponentSize::XLarge)

// 全宽按钮
Button::new("Full Width")
    .full_width(true)

// 禁用按钮
Button::new("Disabled")
    .disabled(true)

// 加载状态
Button::new("Loading...")
    .loading(true)
```

## 🎨 使用主题

```rust
use fluix::theme::*;

// 获取主题
let theme = Theme::default();

// 使用颜色
div().bg(theme.colors.primary)
div().text_color(theme.colors.text)

// 使用间距
div().p(px(Spacing::MD))
div().gap(px(Spacing::SM))

// 使用圆角
div().rounded(px(BorderRadius::MD))
```

## 📚 完整示例

查看 `examples/` 目录获取更多完整示例：

```bash
# 查看 Button 示例
cargo run --example button_demo

# 查看 TextInput 示例
cargo run --example text_input_demo
```

## 🔍 学习资源

- [README.md](README.md) - 完整文档
- [ROADMAP.md](ROADMAP.md) - 组件列表和开发计划
- [CONTRIBUTING.md](CONTRIBUTING.md) - 组件开发指南
- [examples/](examples/) - 示例代码

## 💡 提示

1. **事件订阅**: 使用 `cx.subscribe_in()` 订阅组件事件
2. **状态管理**: 在结构体中保存组件的 `Entity<T>`
3. **样式**: 使用 GPUI 的链式 API 设置样式
4. **主题**: 使用 `Theme` 保持一致的设计

## 🐛 常见问题

### Q: 窗口不显示？
A: 确保调用了 `app.run()` 并正确创建了窗口。

### Q: 组件不响应点击？
A: 检查是否正确订阅了事件并调用了 `.detach()`。

### Q: 样式不生效？
A: 确保使用了正确的 GPUI API，如 `.bg()`, `.text_color()` 等。

### Q: 如何更新组件状态？
A: 修改状态后调用 `cx.notify()` 触发重新渲染。

## 🚀 下一步

- 探索更多组件
- 自定义主题
- 创建复杂布局
- 为 Fluix 贡献代码！

---

**需要帮助?** 查看 [GitHub Issues](https://github.com/yourusername/fluix/issues)
