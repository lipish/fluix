# Combobox Enter 键自动选择优化

## 功能描述

为 Combobox 组件添加了 Enter 键自动选择功能，提升用户体验。

当用户按 Enter 键选择选项后：
- ✅ 自动选择匹配的选项
- ✅ 下拉菜单自动关闭
- ✅ Combobox 自动失去焦点

## 使用场景

### 场景 1：完全匹配

用户输入的文本与某个选项完全匹配（不区分大小写）时，按 Enter 键自动选择该选项。

**示例**：
- 输入 "GPT-4" 或 "gpt-4" → 按 Enter → 自动选择 "GPT-4"
- 输入 "Claude 2" → 按 Enter → 自动选择 "Claude 2"

### 场景 2：唯一过滤结果

用户输入的文本过滤后只剩下一个选项时，按 Enter 键自动选择该选项。

**示例**：
- 输入 "gemini" → 只匹配 "Gemini Pro" → 按 Enter → 自动选择
- 输入 "llama" → 只匹配 "Llama 2" → 按 Enter → 自动选择

### 场景 3：多个匹配

如果有多个匹配结果，按 Enter 键不会自动选择，用户需要继续输入或手动选择。

**示例**：
- 输入 "claude" → 匹配 "Claude 3 Opus", "Claude 3 Sonnet", "Claude 2" → 按 Enter → 不自动选择
- 输入 "gpt" → 匹配 "GPT-4", "GPT-3.5" → 按 Enter → 不自动选择

## 实现细节

### 事件处理

监听 `TextInputEvent::Submit` 事件（用户按 Enter 键时触发）：

```rust
TextInputEvent::Submit(value) => {
    if !value.is_empty() {
        let all_options = this.all_options();
        
        // 1. 首先尝试完全匹配（不区分大小写）
        let exact_match = all_options.iter().find(|opt| 
            opt.label.eq_ignore_ascii_case(value) || 
            opt.value.eq_ignore_ascii_case(value)
        );
        
        if let Some(matched_option) = exact_match {
            // 找到完全匹配，选择它
            this.select_option(matched_option.value.clone(), cx);

            // 移除焦点，让 combobox 失去焦点
            _window.blur();
        } else {
            // 2. 没有完全匹配，检查是否只有一个过滤结果
            let filtered = all_options.iter().filter(|opt|
                opt.label.to_lowercase().contains(&value.to_lowercase()) ||
                opt.value.to_lowercase().contains(&value.to_lowercase())
            ).collect::<Vec<_>>();

            if filtered.len() == 1 {
                // 只有一个匹配，选择它
                this.select_option(filtered[0].value.clone(), cx);

                // 移除焦点，让 combobox 失去焦点
                _window.blur();
            }
            // 如果有多个匹配或没有匹配，不做任何操作
        }
    }
}
```

### 匹配逻辑

1. **完全匹配优先**：
   - 使用 `eq_ignore_ascii_case()` 进行不区分大小写的完全匹配
   - 同时检查 `label` 和 `value` 字段
   - 如果找到完全匹配，立即选择

2. **唯一过滤结果**：
   - 如果没有完全匹配，使用 `contains()` 进行模糊匹配
   - 统计过滤结果数量
   - 只有当结果数量为 1 时才自动选择

3. **多个匹配或无匹配**：
   - 不执行任何操作
   - 保持下拉菜单打开状态
   - 用户可以继续输入或手动选择

## 用户体验提升

### 提升点 1：快速选择

用户可以通过键盘快速完成选择，无需使用鼠标：
1. 输入选项名称
2. 按 Enter 键
3. 完成选择
4. Combobox 自动失去焦点，可以继续其他操作

### 提升点 2：智能匹配

系统会智能判断用户意图：
- 完全匹配时立即选择
- 唯一结果时自动选择
- 多个结果时等待用户进一步操作

### 提升点 3：容错性

不区分大小写的匹配提高了容错性：
- 用户不需要记住准确的大小写
- "gpt-4", "GPT-4", "Gpt-4" 都能正确匹配

## 测试方法

运行测试程序：
```bash
cargo run --example combobox_test
```

测试用例：

1. **完全匹配测试**：
   - 输入 "GPT-4" → 按 Enter → 应该选择 "GPT-4"
   - 输入 "claude 2" → 按 Enter → 应该选择 "Claude 2"

2. **唯一过滤结果测试**：
   - 输入 "gemini" → 按 Enter → 应该选择 "Gemini Pro"
   - 输入 "mistral" → 按 Enter → 应该选择 "Mistral"

3. **多个匹配测试**：
   - 输入 "claude" → 按 Enter → 不应该自动选择
   - 输入 "gpt" → 按 Enter → 不应该自动选择

4. **大小写测试**：
   - 输入 "gpt-4" → 按 Enter → 应该选择 "GPT-4"
   - 输入 "CLAUDE 2" → 按 Enter → 应该选择 "Claude 2"

## 兼容性

此优化完全向后兼容：
- 不影响现有的鼠标点击选择功能
- 不影响现有的过滤功能
- 只是增加了一个额外的快捷方式

## 代码位置

文件：`src/components/form/combobox.rs`
位置：第 594-621 行
事件：`TextInputEvent::Submit`

