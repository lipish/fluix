# Known Limitations

本文档记录 Fluix 组件库在使用 GPUI 框架时遇到的已知限制。

## TextArea 组件

### 鼠标拖拽选择文本的限制

**问题描述：**
无法实现精确的鼠标拖拽选择文本功能。

**技术原因：**
GPUI 0.2.2 的 `MouseDownEvent` 和 `MouseMoveEvent` 中的 `position` 字段类型为 `Point<Pixels>`，但 `Pixels` 结构体的内部字段是私有的：

```rust
pub struct Pixels(f32);  // 字段 `0` 是私有的
```

这导致无法直接访问鼠标的 x/y 坐标值来计算点击位置对应的字符索引。

**尝试的代码：**
```rust
// ❌ 编译错误：field `0` of struct `gpui::Pixels` is private
let x = event.position.x.0;
let y = event.position.y.0;
```

**当前解决方案：**
实现了以下替代交互方式：
- ✅ 单击：清除选择并聚焦
- ✅ 双击：选择全部文本
- ✅ Cmd/Ctrl+A：键盘快捷键全选
- ✅ 选中高亮：蓝色背景显示

**未来展望：**
如果 GPUI 未来版本提供以下任一方式，即可实现完整的鼠标选择功能：
1. 将 `Pixels` 的字段设为公开
2. 添加 `pub fn value(&self) -> f32` 方法
3. 实现 `Deref<Target = f32>` trait
4. 提供专门的坐标访问 API

**相关链接：**
- GPUI GitHub: https://github.com/zed-industries/zed
- GPUI 版本: 0.2.2

**记录时间：** 2025-10-25

---

## 如何贡献

如果你发现此限制已被解决或有其他解决方案，欢迎提交 Issue 或 Pull Request！
