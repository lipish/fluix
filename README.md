# Fluix - Rust UI Component Library

[![Rust](https://img.shields.io/badge/rust-1.75%2B-orange.svg)](https://www.rust-lang.org/)
[![GPUI](https://img.shields.io/badge/GPUI-0.2-blue.svg)](https://github.com/zed-industries/zed)
[![License](https://img.shields.io/badge/license-MIT-green.svg)](LICENSE)
[![Crates.io](https://img.shields.io/crates/v/fluix.svg)](https://crates.io/crates/fluix)
[![Documentation](https://docs.rs/fluix/badge.svg)](https://docs.rs/fluix)

一个基于 GPUI 0.2 的现代化 Rust UI 组件库，提供完整的、易用的组件集合。

> ⚠️ **开发中**: Fluix 目前处于早期开发阶段，API 可能会有变化。

## ✨ 特性

- 🎨 **丰富的组件** - 46+ 个精心设计的 UI 组件
- 🚀 **高性能** - 基于 GPUI 的 GPU 加速渲染
- 🎯 **类型安全** - 充分利用 Rust 的类型系统
- 🛠️ **可定制** - 灵活的主题系统和样式定制
- 📚 **完善文档** - 详细的 API 文档和示例代码
- 🔧 **易于使用** - 简洁一致的 API 设计

## 📦 安装

```toml
[dependencies]
fluix = "0.1"
gpui = "0.2"
```

## 🚀 快速开始

### Button Component

```rust
use gpui::*;
use fluix::{Button, ButtonVariant, ComponentSize, ButtonEvent};

fn main() {
    let app = Application::new();
    app.run(move |cx| {
        cx.open_window(window_options, |window, cx| {
            cx.new(|cx| MyView::new(window, cx))
        }).unwrap();
    });
}

struct MyView {
    button: Entity<Button>,
}

impl MyView {
    fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        let button = cx.new(|_cx| {
            Button::new("Click Me")
                .variant(ButtonVariant::Primary)
                .size(ComponentSize::Medium)
        });
        
        cx.subscribe_in(&button, window, Self::on_click).detach();
        
        Self { button }
    }
    
    fn on_click(&mut self, _: &Entity<Button>, _: &ButtonEvent, _: &mut Window, _: &mut Context<Self>) {
        println!("Button clicked!");
    }
}

impl Render for MyView {
    fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
        div()
            .p_4()
            .child(self.button.clone())
    }
}
```

### Icon Component

```rust
use gpui::*;
use fluix::{Icon, IconName, IconSize};

// Using predefined icon names
let icon = Icon::new(IconName::Star)
    .large()
    .color(rgb(0xF59E0B));

// Custom size and color
let custom_icon = Icon::new(IconName::Heart)
    .size(IconSize::Custom(28.0))
    .color(rgb(0xEF4444));

// Using custom SVG path
let custom_svg = Icon::from_path("icons/my-icon.svg")
    .medium();
```

Available icon names: `ArrowLeft`, `ArrowRight`, `ArrowUp`, `ArrowDown`, `Check`, `Close`, `Plus`, `Minus`, `Search`, `Settings`, `Home`, `User`, `Bell`, `Star`, `Heart`, `Menu`, `Info`, `Warning`, `Error`, `Success`

### TextInput Component

```rust
use gpui::*;
use fluix::{TextInput, TextInputEvent};

let input = cx.new(|cx| {
    TextInput::new(cx)
        .placeholder("Enter your name...")
        .max_length(50)
});

cx.subscribe_in(&input, window, |_, _, event: &TextInputEvent, _, _| {
    match event {
        TextInputEvent::Change(value) => println!("Value: {}", value),
        TextInputEvent::Submit(value) => println!("Submitted: {}", value),
        _ => {}
    }
}).detach();
```

### TextArea Component

```rust
use gpui::*;
use fluix::{TextArea, TextAreaEvent};

let textarea = cx.new(|cx| {
    TextArea::new(cx)
        .placeholder("Type your message...")
        .min_height(80.0)
        .max_height(200.0)
});

// Custom styling
let custom_textarea = cx.new(|cx| {
    TextArea::new(cx)
        .placeholder("Styled textarea...")
        .min_height(60.0)
        .bg_color(rgb(0xF0F9FF))          // Light blue background
        .border_color(rgb(0x3B82F6))       // Blue border
        .focus_border_color(rgb(0x2563EB)) // Darker blue on focus
});

// Borderless textarea
let borderless = cx.new(|cx| {
    TextArea::new(cx)
        .placeholder("No border...")
        .bg_color(rgb(0xFAFAFA))
        .no_border()
});

cx.subscribe_in(&textarea, window, |_, _, event: &TextAreaEvent, _, _| {
    match event {
        TextAreaEvent::Change(value) => println!("Content: {}", value),
        TextAreaEvent::Submit(value) => println!("Submitted: {}", value),
        _ => {}
    }
}).detach();
```

**Keyboard Shortcuts:**
- `Cmd+A` / `Ctrl+A` - Select all text
- `Shift+Enter` - Insert newline
- `Enter` - Submit
- `Backspace` - Delete character or selected text

**Mouse Actions:**
- `Double-click` - Select all text
- `Single-click` - Focus and position cursor

## 📚 组件列表

### ✅ 已实现 (4/46)

#### 基础组件
- ✅ **Button** - 按钮组件
- ✅ **Icon** - 图标组件

#### 表单组件
- ✅ **TextInput** - 单行文本输入
- ✅ **TextArea** - 多行文本编辑器

### 🔄 开发中

#### 基础组件 (17)
- [ ] Badge - 徽章
- [ ] Checkbox - 复选框
- [ ] Radio - 单选框
- [ ] Switch - 开关
- [ ] Tag - 标签
- [ ] Label - 标签文本
- [ ] Avatar - 头像
- [ ] Kbd - 键盘快捷键
- [ ] Progress - 进度条
- [ ] Slider - 滑块
- [ ] Skeleton - 骨架屏
- [ ] Tooltip - 工具提示
- [ ] Toggle - 切换按钮
- [ ] Image - 图片
- [ ] Indicator - 指示器
- [ ] Alert - 警告
- [ ] Accordion - 手风琴

#### 表单组件 (6)
- [ ] ColorPicker - 颜色选择器
- [ ] DatePicker - 日期选择器
- [ ] Dropdown - 下拉选择
- [ ] Form - 表单容器
- [ ] NumberInput - 数字输入
- [ ] OtpInput - OTP 输入
- [ ] Editor - 代码编辑器

#### 布局组件 (9)
- [ ] DescriptionList - 描述列表
- [ ] Drawer - 抽屉
- [ ] GroupBox - 分组框
- [ ] Modal - 模态框
- [ ] Notification - 通知
- [ ] Popover - 气泡卡片
- [ ] Resizable - 可调整大小
- [ ] Scrollable - 滚动容器
- [ ] Sidebar - 侧边栏

#### 高级组件 (10)
- [ ] Calendar - 日历
- [ ] Chart - 图表
- [ ] List - 列表
- [ ] PopupMenu - 弹出菜单
- [ ] Table - 数据表格
- [ ] Tabs - 选项卡
- [ ] Tree - 树形组件
- [ ] VirtualList - 虚拟列表
- [ ] WebView - Web 视图

## 🎨 主题系统

Fluix 提供了灵活的主题系统：

```rust
use fluix::theme::*;

let theme = Theme::new();
let colors = theme.colors;

// 使用预定义颜色
colors.primary;          // 主色
colors.success;          // 成功色
colors.error;            // 错误色

// 使用尺寸系统
Size::Small.px();        // 28.0
Size::Medium.px();       // 36.0
Size::Large.px();        // 44.0

// 使用间距系统
Spacing::SM;             // 8.0
Spacing::MD;             // 12.0
Spacing::LG;             // 16.0

// 使用圆角系统
BorderRadius::SM;        // 4.0
BorderRadius::MD;        // 6.0
BorderRadius::LG;        // 8.0
```

## 📖 示例

运行示例项目：

```bash
# Button 组件示例
cargo run --example button_demo

# Icon 组件示例  
cargo run --example icon_demo

# TextInput 和 TextArea 示例
cargo run --example text_input_demo
```

## 🗺️ 开发路线图

详见 [ROADMAP.md](ROADMAP.md)

### Phase 1: 核心基础组件 (优先级: 高)
- Button, Icon, Label, Checkbox, Radio, Switch, Badge, Tag

### Phase 2: 表单组件 (优先级: 高)
- Dropdown, Form, NumberInput, ColorPicker, DatePicker

### Phase 3: 反馈组件 (优先级: 中)
- Alert, Tooltip, Modal, Notification, Progress, Indicator, Skeleton

### Phase 4: 布局组件 (优先级: 中)
- Drawer, Sidebar, Tabs, Accordion, GroupBox, Resizable, Scrollable

### Phase 5: 数据展示组件 (优先级: 中低)
- Table, List, VirtualList, Tree, Calendar, DescriptionList

### Phase 6: 高级组件 (优先级: 低)
- Chart, PopupMenu, Popover, WebView, Editor, OtpInput

## 🤝 贡献

欢迎贡献！请查看 [ROADMAP.md](ROADMAP.md) 了解当前进度和待实现的组件。

在贡献时请遵循：
1. 保持代码风格一致
2. 编写详细的文档和示例
3. 参考 gpui-component 的 API 设计
4. 更新 ROADMAP.md 中的进度

## 📄 许可证

MIT License

## 🔗 相关链接

- [GPUI](https://github.com/zed-industries/zed) - 底层 UI 框架
- [gpui-component](https://github.com/longbridge/gpui-component) - 参考实现
- [文档](https://docs.rs/fluix) - API 文档
- [示例](examples/) - 示例代码

## 🙏 致谢

- [GPUI](https://github.com/zed-industries/zed) - 提供强大的 GPU 加速 UI 框架
- [gpui-component](https://github.com/longbridge/gpui-component) - 设计灵感来源

---

**当前版本**: v0.1.1  
**已实现组件**: 4/46 (8.7%)  
**最后更新**: 2025-10-25
