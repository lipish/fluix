# TextArea 中文输入退格键崩溃问题修复

## 问题描述

TextArea 组件在处理中文输入时存在以下严重问题：

1. **退格键崩溃**：输入中文字符后使用退格键会导致应用程序崩溃
2. **无法输入内容**：修复退格键后发现完全无法输入任何文字
3. **只显示一个字符**：能输入后发现只能显示一个字符
4. **输入内容被替换**：每次输入新内容都会替换之前的所有内容

## 错误信息

```
thread 'main' panicked at src/components/form/text_area.rs:426:31:
byte index 3 is not a char boundary; it is inside '你' (bytes 1..4) of ` 你好`
```

## 问题根源分析

### 1. 字符边界处理不当

**问题**：在 `handle_backspace` 和 `handle_delete` 方法中，直接使用字符串切片操作，没有考虑多字节字符（如中文）的边界。

```rust
// 有问题的代码
let before = self.value.chars().take(self.cursor_pos - 1).collect::<String>();
let after = self.value.chars().skip(self.cursor_pos).collect::<String>();
self.value = format!("{}{}", before, after);
```

**原因**：中文字符占用多个字节，直接的字符串操作可能在字符边界中间切断，导致无效的 UTF-8 序列。

### 2. EntityInputHandler 未正确注册

**问题**：TextArea 组件没有正确注册 `EntityInputHandler`，导致输入功能完全失效。

**原因**：缺少在 `Render` 实现中添加 `TextAreaElement` 子组件来注册输入处理器。

### 3. 索引类型混乱

**问题**：TextArea 中存在三种不同的索引类型，转换逻辑错误：
- **字符索引**：`cursor_pos`, `selection_start`, `selection_end`
- **字节索引**：`marked_range`, 字符串操作
- **UTF-16 索引**：IME 通信

### 4. 字符范围到字节范围转换错误（最关键）

**问题**：`char_range_to_byte_range` 方法在处理光标位置（空范围）时有严重bug。

```rust
// 有问题的逻辑
// 光标位置 1..1 被错误转换为 0..1
// 导致每次输入都替换从开始到光标的所有文本
```

## 解决方案

### 1. 修复字符删除安全性

```rust
/// 安全地删除指定位置的字符
fn remove_char_at(&mut self, pos: usize) -> bool {
    let chars: Vec<char> = self.value.chars().collect();
    if pos < chars.len() {
        let mut new_chars = Vec::new();
        new_chars.extend_from_slice(&chars[..pos]);
        new_chars.extend_from_slice(&chars[pos + 1..]);
        self.value = new_chars.into_iter().collect();
        true
    } else {
        false
    }
}

/// 确保光标位置在有效范围内
fn validate_cursor_position(&mut self) {
    let char_count = self.value.chars().count();
    if self.cursor_pos > char_count {
        self.cursor_pos = char_count;
    }
}

fn handle_backspace(&mut self, cx: &mut Context<Self>) {
    // ... IME 和选择处理 ...
    
    // 确保光标位置有效
    self.validate_cursor_position();
    
    if self.cursor_pos == 0 {
        return;
    }

    // 安全地删除字符
    if self.remove_char_at(self.cursor_pos - 1) {
        self.cursor_pos -= 1;
    }
    
    // ... 事件发送 ...
}
```

### 2. 正确注册 EntityInputHandler

```rust
// 在 Render 实现中添加
let content = div()
    .id("text-area")
    .track_focus(&self.focus_handle)
    .child(TextAreaElement {
        entity: cx.entity().clone(),
    })
    // ... 其他内容
```

```rust
// TextAreaElement 的 paint 方法
impl Element for TextAreaElement {
    fn paint(&mut self, bounds: Bounds<Pixels>, window: &mut Window, cx: &mut App) {
        // 注册输入处理器
        let focus_handle = self.entity.read(cx).focus_handle.clone();
        window.handle_input(
            &focus_handle,
            ElementInputHandler::new(bounds, self.entity.clone()),
            cx,
        );
    }
}
```

### 3. 修复索引转换逻辑

```rust
/// 字符范围 -> UTF-16 范围
fn char_range_to_utf16(&self, char_range: &std::ops::Range<usize>) -> std::ops::Range<usize> {
    let mut utf16_start = 0;
    let mut utf16_end = 0;
    let mut char_count = 0;
    
    for ch in self.value.chars() {
        if char_count == char_range.start {
            utf16_start = utf16_end;
        }
        utf16_end += ch.len_utf16();
        if char_count == char_range.end {
            break;
        }
        char_count += 1;
    }
    
    utf16_start..utf16_end
}

/// 字节范围 -> UTF-16 范围  
fn byte_range_to_utf16(&self, range: &std::ops::Range<usize>) -> std::ops::Range<usize> {
    let start_idx = range.start.min(self.value.len());
    let end_idx = range.end.min(self.value.len());
    
    let start = self.value[..start_idx].encode_utf16().count();
    let end = self.value[..end_idx].encode_utf16().count();
    start..end
}
```

### 4. 修复字符范围到字节范围转换（关键修复）

```rust
/// 字符范围 -> 字节范围
fn char_range_to_byte_range(&self, char_range: &std::ops::Range<usize>) -> std::ops::Range<usize> {
    // 处理空字符串
    if self.value.is_empty() {
        return 0..0;
    }
    
    // 处理光标位置（空范围）- 关键修复！
    if char_range.start == char_range.end {
        // 找到字符索引对应的字节位置
        let mut char_count = 0;
        for (byte_idx, _) in self.value.char_indices() {
            if char_count == char_range.start {
                return byte_idx..byte_idx;  // 返回相同的字节位置
            }
            char_count += 1;
        }
        // 光标在末尾
        return self.value.len()..self.value.len();
    }

    // 处理非空范围
    let mut byte_start = 0;
    let mut byte_end = self.value.len();
    let mut char_count = 0;
    let mut found_start = false;

    for (byte_idx, _) in self.value.char_indices() {
        if char_count == char_range.start && !found_start {
            byte_start = byte_idx;
            found_start = true;
        }
        if char_count == char_range.end {
            byte_end = byte_idx;
            break;
        }
        char_count += 1;
    }

    // 边界情况处理
    if char_range.start == 0 {
        byte_start = 0;
    }
    if char_range.end >= self.value.chars().count() {
        byte_end = self.value.len();
    }

    byte_start..byte_end
}
```

### 5. 修复 TextRun 长度计算

```rust
// 选择处理中正确的字符到字节转换
let line_chars: Vec<char> = line_str.chars().collect();

// Text before selection
if sel_start_in_line > 0 {
    let before_text: String = line_chars[..sel_start_in_line].iter().collect();
    runs.push(TextRun {
        len: before_text.len(), // 字节长度，不是字符长度
        font: font_clone.clone(),
        color: rgb(0x333333).into(),
        // ...
    });
}
```

### 6. 完整的 EntityInputHandler 实现

```rust
impl EntityInputHandler for TextArea {
    fn replace_text_in_range(
        &mut self,
        range_utf16: Option<std::ops::Range<usize>>,
        new_text: &str,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if self.disabled {
            return;
        }
        
        // 正确处理不同类型的范围
        let mut range = range_utf16
            .as_ref()
            .map(|range_utf16| self.range_from_utf16(range_utf16))
            .or(self.marked_range.clone())
            .unwrap_or_else(|| {
                // 字符选择 -> 字节范围
                let char_range = self.selected_range();
                self.char_range_to_byte_range(&char_range)
            });
        
        // 确保范围在字符边界上
        range.start = range.start.min(self.value.len());
        range.end = range.end.min(self.value.len());
        
        while range.start > 0 && !self.value.is_char_boundary(range.start) {
            range.start -= 1;
        }
        while range.end > 0 && range.end < self.value.len() && !self.value.is_char_boundary(range.end) {
            range.end += 1;
        }
        
        // 替换文本
        let mut new_value = String::new();
        new_value.push_str(&self.value[..range.start]);
        new_value.push_str(new_text);
        new_value.push_str(&self.value[range.end..]);
        
        // 字节位置 -> 字符位置（用于光标）
        let char_pos_before = self.value[..range.start].chars().count();
        
        // 应用更改
        self.value = new_value;
        self.cursor_pos = char_pos_before + new_text.chars().count();
        self.selection_start = None;
        self.selection_end = None;
        self.marked_range = None;
        
        // 发送事件
        cx.emit(TextAreaEvent::Change(self.value.clone()));
        cx.notify();
    }
    
    // ... 其他 EntityInputHandler 方法
}
```

## 调试过程

### 调试技巧

1. **添加详细的调试输出**：
```rust
println!("replace_text_in_range: range_utf16={:?}, new_text='{}', cursor_pos={}, value='{}'", 
         range_utf16, new_text, self.cursor_pos, self.value);
```

2. **跟踪范围转换**：
```rust
println!("Using selected range (chars): {:?} -> byte range: {:?}", char_range, byte_range);
```

3. **对比工作的组件**：研究 TextInput 组件的实现差异

### 关键发现

通过调试输出发现的问题模式：
```
// 问题模式：每次输入都替换之前的内容
replace_text_in_range: range_utf16=None, new_text='s', cursor_pos=1, value='a'
Using selected range (chars): 1..1 -> byte range: 0..1  // 错误！应该是 1..1

// 修复后的正确模式：
replace_text_in_range: range_utf16=None, new_text='s', cursor_pos=1, value='a'
Using selected range (chars): 1..1 -> byte range: 1..1  // 正确！
```

## 测试验证

### 测试用例

创建了专门的测试示例 `text_area_chinese_demo.rs`：

```rust
// 测试场景
1. 输入英文字符：asdfasdf
2. 输入中文字符：你好世界
3. 混合输入：Hello你好World
4. 使用退格键删除
5. 使用Delete键删除
6. 文本选择和删除
```

### 验证结果

- ✅ **正常输入**：可以连续输入英文和中文字符
- ✅ **退格安全**：退格键不会导致崩溃
- ✅ **删除安全**：Delete键正常工作
- ✅ **IME支持**：输入法编辑器功能正常
- ✅ **混合文本**：中英文混合输入完全支持
- ✅ **选择操作**：文本选择和删除正常
- ✅ **光标定位**：光标位置计算准确

## 经验教训

### 1. 多字节字符处理的复杂性

在处理 Unicode 文本时，必须区分：
- **字符数量**：用户看到的字符个数
- **字节长度**：内存中的实际字节数
- **UTF-16 长度**：与系统 IME 通信使用

### 2. EntityInputHandler 的重要性

对于需要 IME 支持的文本组件，正确注册 `EntityInputHandler` 是必须的：
- 必须在 Element 的 `paint` 方法中注册
- 需要提供正确的边界信息
- 必须实现所有必要的方法

### 3. 索引转换的精确性

字符索引和字节索引之间的转换必须精确：
- 空范围（光标位置）必须正确处理
- 边界情况需要特殊处理
- 转换逻辑必须与文本操作逻辑一致

### 4. 系统性调试的价值

通过添加详细的调试输出，能够：
- 快速定位问题根源
- 理解复杂的数据流
- 验证修复的正确性

## 相关文件

- `src/components/form/text_area.rs` - 主要修复文件
- `examples/text_area_chinese_demo.rs` - 测试示例
- `src/components/form/text_input.rs` - 参考实现

## 版本信息

- **修复版本**：v0.1.23
- **修复日期**：2025-11-03
- **影响范围**：TextArea 组件的所有文本输入功能
