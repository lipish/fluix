# Icon 组件实现说明

## 当前实现

Icon 组件使用 **SVG 文件** 来显示图标，遵循 Zed 编辑器的实现模式。

### 支持的图标

| 图标名称 | Unicode 符号 | 说明 |
|---------|-------------|------|
| ArrowLeft | ← | 向左箭头 |
| ArrowRight | → | 向右箭头 |
| ArrowUp | ↑ | 向上箭头 |
| ArrowDown | ↓ | 向下箭头 |
| Check | ✓ | 勾选标记 |
| Close | ✕ | 关闭/删除 |
| Plus | + | 加号 |
| Minus | − | 减号 |
| Search | 🔍 | 搜索 |
| Settings | ⚙ | 设置 |
| Home | 🏠 | 主页 |
| User | 👤 | 用户 |
| Bell | 🔔 | 通知 |
| Star | ★ | 星标 |
| Heart | ♥ | 喜欢 |
| Menu | ☰ | 菜单 |
| Info | ℹ | 信息 |
| Warning | ⚠ | 警告 |
| Error | ⊗ | 错误 |
| Success | ✓ | 成功 |
| UnfoldMore | ⇅ | 展开/折叠 |

## 实现原理（基于 Zed 的模式）

### 核心思路

参考 Zed 编辑器的实现：
1. **图标枚举**：定义 `IconName` 枚举表示所有可用图标
2. **路径映射**：`IconName::path()` 返回对应的 SVG 文件路径
3. **SVG 渲染**：使用 GPUI 的 `svg().path()` 加载并渲染 SVG

### 代码结构

```rust
// 1. 定义图标枚举
pub enum IconName {
    ArrowDown,
    Check,
    Search,
    // ...
}

// 2. 实现路径映射
impl IconName {
    pub fn path(self) -> &'static str {
        match self {
            Self::ArrowDown => "icons/arrow-down.svg",
            Self::Check => "icons/check.svg",
            // ...
        }
    }
}

// 3. Icon 组件渲染
impl RenderOnce for Icon {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let size = self.size.px();
        let color = self.color.unwrap_or(rgb(0x333333));
        let path = self.name.path();

        svg()
            .path(path)  // 从 assets 加载 SVG
            .size(size)
            .text_color(color)
            .flex_none()
    }
}
```

### 使用方法

```rust
use fluix::{Icon, IconName};

// 创建图标
let icon = Icon::new(IconName::ArrowDown)
    .medium()
    .color(rgb(0x666666));

// 在组件中使用
div().child(icon)
```

### 文件组织

```
fluix/
├── icons/              # SVG 图标文件
│   ├── arrow-down.svg
│   ├── check.svg
│   ├── search.svg
│   └── ...
└── src/
    └── components/
        └── basic/
            └── icon.rs  # Icon 组件实现
```

## 限制

1. **不是真正的 SVG**：使用 Unicode 符号而不是矢量图形
2. **样式限制**：无法自定义描边宽度、填充等 SVG 属性
3. **跨平台一致性**：不同操作系统的 Unicode 符号渲染可能不同
4. **图标库有限**：只能使用有对应 Unicode 符号的图标

## 未来改进方向

### 方案 1：GPUI SVG 支持（推荐）

等待 GPUI 添加对内联 SVG 或 SVG 字符串的支持。

**优点**：
- 真正的矢量图形
- 完全可定制
- 高质量渲染

**缺点**：
- 需要等待 GPUI 更新

### 方案 2：Canvas 绘制

使用 GPUI 的 Canvas API 手动绘制图标路径。

**优点**：
- 不依赖外部资源
- 完全控制渲染

**缺点**：
- 实现复杂
- 需要手动转换 SVG 路径到 Canvas 命令
- 性能可能不如原生 SVG

### 方案 3：图标字体

使用 Web 字体（如 Font Awesome、Material Icons）。

**优点**：
- 易于实现
- 图标库丰富

**缺点**：
- 需要加载字体文件
- 样式定制受限

### 方案 4：预渲染位图

将 SVG 预渲染为不同尺寸的 PNG。

**优点**：
- 兼容性好
- 渲染快速

**缺点**：
- 不是真正的矢量图形
- 文件体积大
- 缩放质量差

## 贡献

如果你有更好的解决方案，欢迎提交 PR！

### 理想的实现

```rust
// 理想情况下，Icon 组件应该这样工作：
impl RenderOnce for Icon {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let size = self.size.px();
        let color = self.color.unwrap_or(rgb(0x333333));
        
        // 使用 Heroicons 或 Lucide 的 SVG 内容
        let svg_content = self.name.svg_content();
        
        svg()
            .with_raw_svg(svg_content)  // 需要 GPUI 支持
            .size(size)
            .text_color(color)
            .flex_none()
    }
}
```

## 参考资源

- [Heroicons](https://heroicons.com/) - Tailwind CSS 官方图标库
- [Lucide](https://lucide.dev/) - 开源图标库
- [GPUI 文档](https://github.com/zed-industries/zed) - Zed 编辑器的 UI 框架

