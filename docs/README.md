# RUI - Rust UI Component Library

[![Rust](https://img.shields.io/badge/rust-1.75%2B-orange.svg)](https://www.rust-lang.org/)
[![GPUI](https://img.shields.io/badge/GPUI-0.2-blue.svg)](https://github.com/zed-industries/zed)
[![License](https://img.shields.io/badge/license-MIT-green.svg)](LICENSE)
[![Crates.io](https://img.shields.io/crates/v/rui.svg)](https://crates.io/crates/rui)
[![Documentation](https://docs.rs/rui/badge.svg)](https://docs.rs/rui)

一个基于 GPUI 0.2 的现代化 Rust UI 组件库，提供完整的、易用的组件集合。

> ⚠️ **开发中**: RUI 目前处于早期开发阶段，API 可能会有变化。

## ✨ 特性

- 🎨 **丰富的组件** - 46+ 个精心设计的 UI 组件
- 🚀 **高性能** - 基于 GPUI 的 GPU 加速渲染
- 🎯 **类型安全** - 充分利用 Rust 的类型系统
- 🛠️ **可定制** - 灵活的主题系统和样式定制
- 📚 **完善文档** - 详细的 API 文档和示例代码
- 🔧 **易于使用** - 简洁一致的 API 设计

## 📦 安装

### 从 crates.io (推荐)

```toml
[dependencies]
rui = "0.1"
gpui = "0.2"
```

### 从 Git 仓库

```toml
[dependencies]
rui = { git = "https://github.com/yourusername/rui" }
gpui = "0.2"
```

### 本地开发

```toml
[dependencies]
rui = { path = "../rui" }
gpui = "0.2"
```

## 🚀 快速开始

```rust
use rui::prelude::*;

fn main() {
    App::new().run(|window, cx| {
        let input = cx.new(|cx| {
            TextInput::new(cx)
                .placeholder("输入文本...")
        });

        window.open_window(cx, |window, cx| {
            cx.new(|cx| MyView { input })
        });
    });
}

struct MyView {
    input: Entity<TextInput>,
}

impl Render for MyView {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .p_4()
            .child(self.input.clone())
    }
}
```

## 📚 组件列表

### ✅ 已实现 (2/46)

#### 表单组件
- ✅ **TextInput** - 单行文本输入
- ✅ **TextArea** - 多行文本编辑器

### 🔄 开发中

#### 基础组件 (19)
- [ ] Button - 按钮
- [ ] Icon - 图标
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

RUI 提供了灵活的主题系统：

```rust
use rui::theme::*;

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
# 运行 TextInput 示例
cargo run --example text_input_demo

# 未来会添加更多示例
cargo run --example button_demo
cargo run --example form_demo
cargo run --example showcase
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
- [文档](https://docs.rs/rui) - API 文档
- [示例](examples/) - 示例代码

## 🙏 致谢

- [GPUI](https://github.com/zed-industries/zed) - 提供强大的 GPU 加速 UI 框架
- [gpui-component](https://github.com/longbridge/gpui-component) - 设计灵感来源

---

**当前版本**: v0.1.0-dev  
**已实现组件**: 2/46 (4.3%)  
**最后更新**: 2025-10-25
