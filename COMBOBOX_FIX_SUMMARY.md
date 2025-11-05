# Combobox 修复总结

## 问题描述

Combobox 组件存在三个主要问题：

1. **选择问题**：选择一个选项后，再次打开下拉菜单时只显示当前选中的选项，无法切换到其他选项
2. **过滤问题**：在输入框中输入文本时，下拉菜单无法正确过滤匹配的选项
3. **下拉菜单不关闭**：选择一个选项后，下拉菜单没有关闭，仍然显示

## 根本原因

### 问题 1：选择后无法切换

**原因**：
- `toggle_dropdown()` 方法在打开下拉菜单时，如果 `input_value` 不为空（已选择选项），会将 `is_user_typing` 设置为 `true`
- `filtered_options()` 方法检查 `is_user_typing && !input_value.is_empty()` 来决定是否过滤
- 这导致打开下拉菜单时触发过滤，只显示匹配当前选中项的选项

**修复**：
- 在 `toggle_dropdown()` 中，打开下拉菜单时总是将 `is_user_typing` 设置为 `false`
- 在 `select_option()` 中，选择后关闭下拉菜单

### 问题 2：输入文本时无法过滤

**原因**：
- TextInput 事件订阅没有正确设置
- 原代码在 `render()` 方法中每次都创建新的订阅，但订阅返回值被丢弃（`let _ =`）
- 事件处理器从未被调用

**修复**：
- 在 `render()` 方法中使用 `cx.subscribe_in()` 正确订阅 TextInput 事件
- 使用 `_subscriptions` 字段保存订阅，确保订阅不会被丢弃
- 只在第一次渲染时订阅（检查 `_subscriptions.is_empty()`）

### 问题 3：选择后下拉菜单不关闭

**原因**：
- `select_option()` 设置 `is_open = false` 关闭下拉菜单
- 然后调用 `text_input.update()` 设置 TextInput 的值
- 这会触发 TextInput 的 `Change` 事件
- `Change` 事件处理器检查 `!this.is_open`，然后重新设置 `this.is_open = true`
- 导致下拉菜单被重新打开

**修复**：
- 在 `Change` 事件处理中，检查是否是程序化的更改（选择选项导致的）
- 通过比较 `selected_value` 和 `input_value` 来判断
- 如果是程序化更改，直接返回，不处理事件

## 新增优化功能

### Enter 键自动选择

当用户输入文本时，如果满足以下条件之一，按 Enter 键会自动选择匹配的选项：

1. **完全匹配**：输入的文本与某个选项的 label 或 value 完全匹配（不区分大小写）
   - 例如：输入 "GPT-4" 或 "gpt-4" 都会匹配 "GPT-4" 选项

2. **唯一过滤结果**：输入的文本过滤后只剩下一个选项
   - 例如：输入 "gemini" 只匹配 "Gemini Pro"，按 Enter 自动选择

**实现位置**：`TextInputEvent::Submit` 事件处理（第 594-621 行）

```rust
TextInputEvent::Submit(value) => {
    if !value.is_empty() {
        let all_options = this.all_options();

        // 首先尝试完全匹配（不区分大小写）
        let exact_match = all_options.iter().find(|opt|
            opt.label.eq_ignore_ascii_case(value) || opt.value.eq_ignore_ascii_case(value)
        );

        if let Some(matched_option) = exact_match {
            // 找到完全匹配，选择它
            this.select_option(matched_option.value.clone(), cx);
        } else {
            // 没有完全匹配，检查是否只有一个过滤结果
            let filtered = all_options.iter().filter(|opt|
                opt.label.to_lowercase().contains(&value.to_lowercase()) ||
                opt.value.to_lowercase().contains(&value.to_lowercase())
            ).collect::<Vec<_>>();

            if filtered.len() == 1 {
                // 只有一个匹配，选择它
                this.select_option(filtered[0].value.clone(), cx);
            }
        }
    }
}
```

## 修复的代码位置

### 1. 添加订阅字段（第 91 行）

```rust
pub struct Combobox {
    // ... 其他字段
    /// Event subscriptions
    _subscriptions: Vec<Subscription>,
}
```

### 2. 修复 toggle_dropdown()（第 305-315 行）

```rust
fn toggle_dropdown(&mut self) {
    if !self.disabled {
        self.is_open = !self.is_open;
        // 打开时总是重置过滤标志，显示所有选项
        if self.is_open {
            self.is_user_typing = false;
        }
    }
}
```

### 3. 修复 select_option()（第 329-350 行）

```rust
fn select_option(&mut self, value: String, cx: &mut Context<Self>) {
    // ...
    self.is_user_typing = false;  // 重置过滤标志
    self.is_open = false;          // 关闭下拉菜单
    // ...
}
```

### 4. 修复事件订阅（第 543-607 行）

```rust
fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
    // ...
    // 只在第一次渲染时订阅
    if let Some(text_input_entity) = &text_input {
        if self._subscriptions.is_empty() {
            let sub = cx.subscribe_in(text_input_entity, window, |this, _input, event: &TextInputEvent, _window, cx| {
                match event {
                    TextInputEvent::Change(value) => {
                        // 检查是否是程序化更改（选择选项导致的）
                        let is_programmatic = this.selected_value.is_some() &&
                            this.all_options().iter().any(|opt|
                                Some(&opt.value) == this.selected_value.as_ref() && &opt.label == value
                            );

                        // 如果是程序化更改，直接返回，不处理
                        if is_programmatic {
                            return;
                        }

                        this.input_value = value.clone();
                        this.selected_value = None;
                        this.is_user_typing = !value.is_empty();  // 有输入时启用过滤
                        // ...
                    }
                    TextInputEvent::Focus => {
                        // 只在输入为空或匹配已选值时重置过滤标志
                        // ...
                    }
                    // ...
                }
            });
            self._subscriptions.push(sub);
        }
    }
    // ...
}
```

### 5. 修复过滤逻辑（第 239-259 行）

```rust
fn filtered_options(&self) -> Vec<SelectOption> {
    // ...
    // 只有当用户主动输入时才过滤
    if self.is_user_typing && !self.input_value.is_empty() {
        // 执行过滤
    }
    // 否则显示所有选项
}
```

## 修复后的行为

✅ **选择功能**：
- 点击下拉箭头显示所有选项
- 选择选项后下拉菜单**立即关闭**
- 重新打开显示所有选项
- 可以自由切换选择不同选项

✅ **过滤功能**：
- 输入文本时正确过滤选项
- 输入 "gpt" 显示 GPT-4 和 GPT-3.5
- 输入 "claude" 显示 Claude 3 和 Claude 2
- 清空输入显示所有选项
- 过滤状态在重新聚焦时保持

✅ **下拉菜单行为**：
- 选择选项后下拉菜单关闭
- 不会因为程序化更改而重新打开
- 只有用户主动输入或点击才会打开

✅ **Enter 键自动选择** (新增优化)：
- 输入文本完全匹配某个选项时，按 Enter 键自动选择
- 支持不区分大小写的匹配（如输入 "gpt-4" 或 "GPT-4" 都可以）
- 如果只有一个过滤结果，按 Enter 键也会自动选择
- 选择后下拉菜单自动关闭
- **Combobox 自动失去焦点**，用户可以继续其他操作

## 测试方法

运行示例程序：
```bash
# 运行完整的 demo
cargo run --example combobox_demo

# 或运行简化的测试程序（推荐）
cargo run --example combobox_test
```

测试步骤：
1. 点击 combobox 选择一个选项（如 GPT-4）
2. 再次点击 combobox - 应该显示所有选项
3. 输入 "gpt" - 应该过滤显示 GPT-4 和 GPT-3.5
4. 输入 "claude" - 应该过滤显示 Claude 相关选项
5. 清空输入 - 应该显示所有选项
6. 选择另一个选项 - 应该正常工作
7. **测试 Enter 键自动选择**：
   - 输入 "GPT-4" 然后按 Enter - 应该自动选择 GPT-4 并关闭下拉菜单
   - 输入 "gemini" 然后按 Enter - 应该自动选择 Gemini Pro（唯一匹配）
   - 输入 "claude" 然后按 Enter - 不会自动选择（有多个匹配）

## 窗口大小建议

如果窗口太小无法完整显示 combobox 和下拉选项，建议设置窗口大小至少为：
- 宽度：800px 或更大
- 高度：600px 或更大

示例代码：
```rust
let window_options = WindowOptions {
    window_bounds: Some(WindowBounds::Windowed(Bounds {
        origin: point(px(100.), px(100.)),
        size: size(px(1000.), px(800.)),  // 推荐大小
    })),
    // ...
};
```

Combobox 组件的尺寸：
- 最小高度：36px
- 下拉菜单最大高度：300px
- 建议容器宽度：400-600px

