# Fluix Component Library Roadmap

Fluix 是一个基于 GPUI 0.2 的 Rust UI 组件库，目标是提供完整的、易用的组件集合。

## 项目目标

- 提供完整的组件库
- 保持简洁、高性能的实现
- 支持主题定制和样式扩展
- 提供详细的示例和文档

## 组件分类与实现状态

### ✅ 已实现组件 (2)

#### Form Components
- [x] **TextInput** - 单行文本输入框
  - 支持 placeholder、password 模式、验证、最大长度
  - 文件: `src/text_input.rs`
  
- [x] **TextArea** - 多行文本编辑器
  - 支持自动扩展高度、Shift+Enter 换行、Enter 提交
  - 光标位置跟踪和正确显示
  - 文件: `src/text_area.rs`

---

### 📋 待实现组件 (44)

#### 基础组件 (Basic Components) - 19个

- [ ] **Accordion** - 可折叠内容面板
- [ ] **Alert** - 不同变体的警告消息
- [ ] **Avatar** - 用户头像（带后备文字）
- [ ] **Badge** - 计数徽章和指示器
- [ ] **Button** - 多种变体的交互按钮
  - Primary, Secondary, Outline, Text
  - 支持图标、加载状态、禁用状态
- [ ] **Checkbox** - 复选框控制
- [ ] **Icon** - 图标显示组件
  - 集成 Lucide 图标
- [ ] **Image** - 图片显示（带后备）
- [ ] **Indicator** - 加载和状态指示器
- [ ] **Kbd** - 键盘快捷键显示
- [ ] **Label** - 表单元素文本标签
- [ ] **Progress** - 进度条
- [ ] **Radio** - 单选框（多选一）
- [ ] **Skeleton** - 加载占位符
- [ ] **Slider** - 范围选择滑块
- [ ] **Switch** - 开关切换控制
- [ ] **Tag** - 标签和分类
- [ ] **Toggle** - 切换按钮状态
- [ ] **Tooltip** - 悬停提示

#### 表单组件 (Form Components) - 6个

- [ ] **ColorPicker** - 颜色选择界面
- [ ] **DatePicker** - 日期选择（带日历）
- [ ] **Dropdown** - 下拉选择
- [ ] **Form** - 表单容器和布局
- [ ] **NumberInput** - 数字输入（带增减）
- [ ] **OtpInput** - 一次性密码输入
- [ ] **Editor** - 代码编辑器

#### 布局组件 (Layout Components) - 9个

- [ ] **DescriptionList** - 键值对显示
- [ ] **Drawer** - 从边缘滑入的面板
- [ ] **GroupBox** - 带边框的分组内容
- [ ] **Modal** - 对话框和模态窗口
- [ ] **Notification** - Toast 通知
- [ ] **Popover** - 浮动内容显示
- [ ] **Resizable** - 可调整大小的面板
- [ ] **Scrollable** - 可滚动容器
- [ ] **Sidebar** - 导航侧边栏

#### 高级组件 (Advanced Components) - 10个

- [ ] **Calendar** - 日历显示和导航
- [ ] **Chart** - 数据可视化图表
  - Line, Bar, Area, Pie
- [ ] **List** - 列表显示
- [ ] **PopupMenu** - 菜单和上下文菜单
- [ ] **Table** - 高性能数据表格
- [ ] **Tabs** - 选项卡界面
- [ ] **Tree** - 分层树形数据显示
- [ ] **VirtualList** - 虚拟化列表（大数据集）
- [ ] **WebView** - 嵌入式 Web 浏览器

---

## 实施计划

### Phase 1: 核心基础组件 (优先级: 高)

**目标**: 提供最常用的基础 UI 元素

1. **Button** - 各种按钮样式
2. **Icon** - 图标支持
3. **Label** - 文本标签
4. **Checkbox** - 复选框
5. **Radio** - 单选框
6. **Switch** - 开关
7. **Badge** - 徽章
8. **Tag** - 标签

**预计时间**: 2-3 周

### Phase 2: 表单组件 (优先级: 高)

**目标**: 完善表单输入能力

1. **Dropdown** - 下拉选择
2. **Form** - 表单容器
3. **NumberInput** - 数字输入
4. **ColorPicker** - 颜色选择器
5. **DatePicker** - 日期选择器

**预计时间**: 2-3 周

### Phase 3: 反馈组件 (优先级: 中)

**目标**: 用户反馈和交互提示

1. **Alert** - 警告消息
2. **Tooltip** - 工具提示
3. **Modal** - 模态对话框
4. **Notification** - 通知
5. **Progress** - 进度条
6. **Indicator** - 加载指示器
7. **Skeleton** - 骨架屏

**预计时间**: 2 周

### Phase 4: 布局组件 (优先级: 中)

**目标**: 页面布局和容器

1. **Drawer** - 抽屉
2. **Sidebar** - 侧边栏
3. **Tabs** - 选项卡
4. **Accordion** - 手风琴
5. **GroupBox** - 分组框
6. **Resizable** - 可调整大小
7. **Scrollable** - 滚动容器

**预计时间**: 2-3 周

### Phase 5: 数据展示组件 (优先级: 中低)

**目标**: 复杂数据展示

1. **Table** - 数据表格
2. **List** - 列表
3. **VirtualList** - 虚拟列表
4. **Tree** - 树形组件
5. **Calendar** - 日历
6. **DescriptionList** - 描述列表

**预计时间**: 3-4 周

### Phase 6: 高级组件 (优先级: 低)

**目标**: 高级功能组件

1. **Chart** - 图表组件
2. **PopupMenu** - 弹出菜单
3. **Popover** - 气泡卡片
4. **WebView** - Web 视图
5. **Editor** - 代码编辑器
6. **OtpInput** - OTP 输入

**预计时间**: 4-5 周

---

## 目录结构规划

```
crates/fluix/
├── src/
│   ├── lib.rs                    # 库入口
│   ├── theme.rs                  # 主题系统
│   ├── components/
│   │   ├── mod.rs                # 组件模块
│   │   ├── basic/                # 基础组件
│   │   │   ├── mod.rs
│   │   │   ├── button.rs
│   │   │   ├── icon.rs
│   │   │   ├── badge.rs
│   │   │   ├── checkbox.rs
│   │   │   ├── radio.rs
│   │   │   ├── switch.rs
│   │   │   ├── tag.rs
│   │   │   ├── label.rs
│   │   │   ├── avatar.rs
│   │   │   ├── kbd.rs
│   │   │   ├── progress.rs
│   │   │   ├── slider.rs
│   │   │   ├── skeleton.rs
│   │   │   ├── tooltip.rs
│   │   │   ├── toggle.rs
│   │   │   ├── image.rs
│   │   │   ├── indicator.rs
│   │   │   ├── alert.rs
│   │   │   └── accordion.rs
│   │   ├── form/                 # 表单组件
│   │   │   ├── mod.rs
│   │   │   ├── text_input.rs     # ✅ 已实现
│   │   │   ├── text_area.rs      # ✅ 已实现
│   │   │   ├── dropdown.rs
│   │   │   ├── form.rs
│   │   │   ├── number_input.rs
│   │   │   ├── color_picker.rs
│   │   │   ├── date_picker.rs
│   │   │   ├── otp_input.rs
│   │   │   └── editor.rs
│   │   ├── layout/               # 布局组件
│   │   │   ├── mod.rs
│   │   │   ├── drawer.rs
│   │   │   ├── modal.rs
│   │   │   ├── sidebar.rs
│   │   │   ├── tabs.rs
│   │   │   ├── group_box.rs
│   │   │   ├── resizable.rs
│   │   │   ├── scrollable.rs
│   │   │   ├── popover.rs
│   │   │   ├── notification.rs
│   │   │   └── description_list.rs
│   │   └── advanced/             # 高级组件
│   │       ├── mod.rs
│   │       ├── table.rs
│   │       ├── list.rs
│   │       ├── virtual_list.rs
│   │       ├── tree.rs
│   │       ├── calendar.rs
│   │       ├── chart.rs
│   │       ├── popup_menu.rs
│   │       └── webview.rs
│   └── utils/                    # 工具函数
│       ├── mod.rs
│       ├── colors.rs             # 颜色工具
│       └── icons.rs              # 图标工具
├── examples/
│   ├── text_input_demo.rs        # ✅ 已有
│   ├── button_demo.rs
│   ├── form_demo.rs
│   ├── layout_demo.rs
│   └── showcase.rs               # 所有组件展示
├── Cargo.toml
├── README.md
└── ROADMAP.md                    # 本文件

```

---

## 设计原则

1. **一致性**: 所有组件使用统一的 API 设计模式
2. **可扩展**: 支持自定义样式和主题
3. **高性能**: 充分利用 GPUI 的 GPU 加速能力
4. **类型安全**: 充分利用 Rust 的类型系统
5. **文档完善**: 每个组件都有详细的示例和文档

## API 设计规范

### 组件创建模式
```rust
// Builder 模式
let button = Button::new("Click me")
    .variant(ButtonVariant::Primary)
    .size(Size::Medium)
    .disabled(false)
    .on_click(|cx| { /* handler */ });
```

### 事件处理模式
```rust
// 使用 EventEmitter trait
pub enum ButtonEvent {
    Click,
    DoubleClick,
}

impl EventEmitter<ButtonEvent> for Button {}
```

### 样式定制模式
```rust
// 使用 theme 系统
button.theme(|theme| {
    theme.primary_color(rgb(0x696FC7))
         .border_radius(px(6.))
})
```

---

## 下一步行动

### 立即开始 (本周)

1. **重构现有组件**
   - 将 `text_input.rs` 移到 `src/components/form/`
   - 将 `text_area.rs` 移到 `src/components/form/`
   - 创建新的目录结构

2. **创建主题系统**
   - 定义统一的颜色方案
   - 创建尺寸规范 (Size enum)
   - 创建间距规范

3. **实现 Button 组件**
   - 作为第一个新组件
   - 建立标准的组件实现模式
   - 创建完整的示例

### 短期目标 (1-2 周)

- 完成 Phase 1 的前 3 个组件 (Button, Icon, Label)
- 创建统一的 showcase 示例
- 编写组件开发指南文档

### 中期目标 (1-2 月)

- 完成 Phase 1 和 Phase 2 的所有组件
- 提供完整的表单解决方案
- 发布 v0.1.0 版本

### 长期目标 (3-6 月)

- 完成所有 46 个组件
- 达到生产环境可用状态
- 发布 v1.0.0 版本

---

## 贡献指南

欢迎贡献！在实现新组件时，请遵循：

1. 保持 API 设计简洁一致
2. 保持代码风格一致
3. 编写详细的示例
4. 添加单元测试（如果适用）
5. 更新此 ROADMAP 文档

---

**最后更新**: 2025-10-25
**当前版本**: v0.1.0-dev
**已实现组件**: 2/46 (4.3%)
