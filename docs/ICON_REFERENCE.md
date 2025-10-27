# Icon 组件参考

## 所有可用图标（22 个）

### 导航图标

| 图标名称 | 枚举值 | 文件 | 描述 |
|---------|--------|------|------|
| ← | `IconName::ArrowLeft` | `arrow-left.svg` | 向左箭头 |
| → | `IconName::ArrowRight` | `arrow-right.svg` | 向右箭头 |
| ↑ | `IconName::ArrowUp` | `arrow-up.svg` | 向上箭头 |
| ↓ | `IconName::ArrowDown` | `arrow-down.svg` | 向下箭头 |
| ⇅ | `IconName::ChevronUpDown` | `chevron-up-down.svg` | 上下箭头（用于 Select） |
| ⇅ | `IconName::UnfoldMore` | `unfold-more.svg` | 展开/折叠 |

### 操作图标

| 图标名称 | 枚举值 | 文件 | 描述 |
|---------|--------|------|------|
| ✓ | `IconName::Check` | `check.svg` | 勾选/确认 |
| ✕ | `IconName::Close` | `close.svg` | 关闭/删除 |
| + | `IconName::Plus` | `plus.svg` | 添加/增加 |
| − | `IconName::Minus` | `minus.svg` | 减少/移除 |
| 🔍 | `IconName::Search` | `search.svg` | 搜索 |

### 界面图标

| 图标名称 | 枚举值 | 文件 | 描述 |
|---------|--------|------|------|
| ⚙ | `IconName::Settings` | `settings.svg` | 设置 |
| 🏠 | `IconName::Home` | `home.svg` | 主页 |
| 👤 | `IconName::User` | `user.svg` | 用户 |
| 🔔 | `IconName::Bell` | `bell.svg` | 通知 |
| ★ | `IconName::Star` | `star.svg` | 星标/收藏 |
| ♥ | `IconName::Heart` | `heart.svg` | 喜欢 |
| ☰ | `IconName::Menu` | `menu.svg` | 菜单 |

### 状态图标

| 图标名称 | 枚举值 | 文件 | 描述 |
|---------|--------|------|------|
| ℹ | `IconName::Info` | `info.svg` | 信息 |
| ⚠ | `IconName::Warning` | `warning.svg` | 警告 |
| ⊗ | `IconName::Error` | `error.svg` | 错误 |
| ✓ | `IconName::Success` | `success.svg` | 成功 |

## 使用示例

### 基本用法

```rust
use fluix::{Icon, IconName};
use gpui::*;

// 创建图标
Icon::new(IconName::ChevronUpDown)
```

### 设置大小

```rust
// 预定义尺寸
Icon::new(IconName::Search).small()      // 16px
Icon::new(IconName::Search).medium()     // 20px
Icon::new(IconName::Search).large()      // 24px
Icon::new(IconName::Search).xlarge()     // 32px

// 自定义尺寸
Icon::new(IconName::Search).custom_size(px(48.))
```

### 设置颜色

```rust
use gpui::rgb;

// 灰色
Icon::new(IconName::Settings).color(rgb(0x666666))

// 蓝色
Icon::new(IconName::Info).color(rgb(0x3B82F6))

// 红色
Icon::new(IconName::Error).color(rgb(0xEF4444))

// 绿色
Icon::new(IconName::Success).color(rgb(0x22C55E))

// 橙色
Icon::new(IconName::Warning).color(rgb(0xF59E0B))
```

### 链式调用

```rust
Icon::new(IconName::ChevronUpDown)
    .small()
    .color(rgb(0x666666))
```

## 在组件中使用

### Select 组件

```rust
// Select 组件自动使用 ChevronUpDown 图标
Select::new("my-select")
    .placeholder("选择选项")
    .options(vec![
        SelectOption::new("1", "选项 1"),
    ])
```

### Button 组件

```rust
Button::new("save-btn", "保存")
    .icon(IconName::Check)
    .variant(ButtonVariant::Primary)
```

### 自定义布局

```rust
div()
    .flex()
    .items_center()
    .gap_2()
    .child(Icon::new(IconName::Search).medium())
    .child("搜索")
```

## 图标尺寸对照

| 尺寸名称 | 像素值 | 使用场景 |
|---------|--------|---------|
| Small | 16px | 按钮内图标、紧凑布局 |
| Medium | 20px | 默认尺寸、表单控件 |
| Large | 24px | 标题、重要操作 |
| XLarge | 32px | 大型按钮、展示用途 |

## 颜色建议

### 语义化颜色

```rust
// 信息 - 蓝色
Icon::new(IconName::Info).color(rgb(0x3B82F6))

// 成功 - 绿色
Icon::new(IconName::Success).color(rgb(0x22C55E))

// 警告 - 橙色
Icon::new(IconName::Warning).color(rgb(0xF59E0B))

// 错误 - 红色
Icon::new(IconName::Error).color(rgb(0xEF4444))
```

### 中性颜色

```rust
// 深灰色（主要内容）
Icon::new(IconName::Menu).color(rgb(0x333333))

// 中灰色（次要内容）
Icon::new(IconName::Settings).color(rgb(0x666666))

// 浅灰色（禁用状态）
Icon::new(IconName::Close).color(rgb(0x999999))
```

## 添加新图标

### 1. 准备 SVG 文件

确保 SVG 符合以下格式：

```xml
<svg xmlns="http://www.w3.org/2000/svg" 
     fill="none" 
     viewBox="0 0 24 24" 
     stroke-width="1.5" 
     stroke="currentColor">
  <path stroke-linecap="round" 
        stroke-linejoin="round" 
        d="..." />
</svg>
```

### 2. 添加到项目

```bash
# 将 SVG 文件放到 assets/icons/
cp my-icon.svg assets/icons/
```

### 3. 更新代码

```rust
// 在 src/components/basic/icon.rs 中

// 1. 添加枚举值
pub enum IconName {
    // ...
    MyIcon,
}

// 2. 添加路径映射
impl IconName {
    pub fn path(self) -> &'static str {
        match self {
            // ...
            Self::MyIcon => "icons/my-icon.svg",
        }
    }
}
```

### 4. 重新编译

```bash
cargo build
```

rust-embed 会自动将新图标嵌入到二进制中。

## 常见问题

### Q: 图标不显示？

**A:** 确保在应用启动时注册了资源：

```rust
Application::new()
    .with_assets(fluix::Assets)  // ← 必须！
    .run(|cx| { ... });
```

### Q: 如何改变图标颜色？

**A:** SVG 使用 `stroke="currentColor"`，通过 `.color()` 方法设置：

```rust
Icon::new(IconName::Star).color(rgb(0xFFD700))
```

### Q: 可以使用自定义 SVG 吗？

**A:** 可以！按照上面的步骤添加新图标即可。

## 图标来源

所有图标来自 [Heroicons](https://heroicons.com/)，使用 MIT 许可证。

## 相关文档

- [Asset Loading Guide](./ASSET_LOADING_GUIDE.md) - 资源加载指南
- [Icon Solution Summary](./ICON_SOLUTION_SUMMARY.md) - SVG 加载解决方案
- [Final Summary](./FINAL_SUMMARY.md) - 项目总结

