# Fluix Component

基于 GPUI 0.2 的 Rust UI 组件库，用于构建现代化的跨平台桌面应用。

[快速开始](#hello-world) | [组件文档](#组件列表)

---

### 46+ 组件

丰富的跨平台桌面 UI 组件库，提供完整的组件集合用于构建功能丰富的应用程序。

### 高性能

基于 GPUI 的 GPU 加速渲染，提供流畅的用户体验。

### 类型安全

充分利用 Rust 的类型系统，编译时保证类型安全。

### 灵活定制

内置主题系统，支持灵活的主题和样式定制。

### 易于使用

简洁一致的 API 设计，让你快速上手。

### 完善文档

详细的 API 文档、教程和示例代码。

## 简洁直观的 API

只需几行代码即可开始使用。无状态组件设计让构建复杂 UI 变得简单。

```rust
Button::new("Click Me")
    .variant(ButtonVariant::Primary)
    .size(ComponentSize::Medium)
    .on_click(|_, _, _| println!("Button clicked!"))
```

## 安装 Fluix Component

在 `Cargo.toml` 中添加以下依赖：

```toml
[dependencies]
fluix = "0.1.20"
gpui = "0.2"
```

## Hello World

以下是一个简单的 "Hello, World!" 应用示例：

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
        .with_assets(fluix::Assets);  // ← 重要！加载 SVG 图标

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

运行程序：

```bash
$ cargo run
```

> ⚠️ **开发中**: Fluix 目前处于早期开发阶段，API 可能会有变化。  
> 💡 **重要**: 使用 Fluix 时，必须在应用启动时调用 `.with_assets(fluix::Assets)` 来加载 SVG 图标资源！

## 文档和教程

### 📚 教程

**新手入门？** 跟随我们的分步教程：

- **[快速开始](docs/tutorials/01-GETTING-STARTED.md)** ⭐ - 你的第一个 Fluix 应用 (30 分钟)
- **[使用组件](docs/tutorials/02-COMPONENTS.md)** - 所有组件详解 (45 分钟)
- **[样式和主题](docs/tutorials/03-STYLING.md)** - 美化你的应用 (30 分钟)

[查看所有教程 →](docs/tutorials/README.md) | [文档索引 →](docs/DOCUMENTATION-INDEX.md)

### 📖 API 参考

- **[组件参考](docs/COMPONENT-REFERENCE.md)** - 所有组件的完整 API 参考
- **[图标参考](docs/ICON_REFERENCE.md)** - 所有 31 个图标的使用示例
- **[常见问题](docs/FAQ.md)** - 常见问题解答
- **[资源加载指南](docs/ASSET_LOADING_GUIDE.md)** - SVG 加载工作原理

## 组件列表

### ✅ 已实现组件

**基础组件**: Button, Icon  
**表单组件**: TextInput, TextArea, Checkbox, Radio, Select, Combobox  
**布局组件**: Tabs, Breadcrumb

### 🔄 开发中

查看 [ROADMAP.md](ROADMAP.md) 了解详细的开发进度和待实现组件列表。

## 示例

运行示例项目：

```bash
# Button 组件示例
cargo run --example button_demo

# Icon 组件示例  
cargo run --example icon_demo

# TextInput 和 TextArea 示例
cargo run --example text_input_demo

# Tabs 组件示例
cargo run --example tabs_demo
```

查看更多示例：[examples/](examples/)

## 贡献

欢迎贡献！请查看 [ROADMAP.md](ROADMAP.md) 了解当前进度和待实现的组件。

## 相关链接

- [GPUI](https://github.com/zed-industries/zed) - 底层 UI 框架
- [gpui-component](https://github.com/longbridge/gpui-component) - 参考实现
- [API 文档](https://docs.rs/fluix) - 完整的 API 文档
- [示例代码](examples/) - 更多示例

## 许可证

MIT License
