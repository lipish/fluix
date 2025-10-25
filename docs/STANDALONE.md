# 将 RUI 独立为单独的 Crate

本指南说明如何将 RUI 从 z-agent 项目中独立出来作为单独的 crate。

## 📋 准备工作

### 1. 创建新的 Git 仓库

```bash
# 在 GitHub 上创建新仓库
# 名称: rui
# 描述: A comprehensive UI component library for GPUI 0.2

# 或者在本地初始化
cd /path/to/new/location
git init rui
cd rui
```

### 2. 复制 RUI 代码

```bash
# 从 z-agent 复制 rui 目录
cp -r /Users/xinference/github/z-agent/crates/rui/* .

# 或者使用 git subtree 分离历史记录
cd /Users/xinference/github/z-agent
git subtree split --prefix=crates/rui -b rui-standalone
cd /path/to/new/rui
git pull /Users/xinference/github/z-agent rui-standalone
```

### 3. 清理独立项目

```bash
# 确保所有必要文件都存在
ls -la

# 应该包含:
# - Cargo.toml
# - LICENSE
# - README.md
# - ROADMAP.md
# - CONTRIBUTING.md
# - CHANGELOG.md
# - .gitignore
# - src/
# - examples/
```

## 🔧 配置调整

### 更新 Cargo.toml

确保 `Cargo.toml` 包含正确的信息：

```toml
[package]
name = "rui"
version = "0.1.0"
edition = "2024"
authors = ["Your Name <your.email@example.com>"]
description = "A comprehensive UI component library for GPUI 0.2"
license = "MIT"
repository = "https://github.com/yourusername/rui"
homepage = "https://github.com/yourusername/rui"
keywords = ["ui", "gui", "gpui", "components", "widgets"]
categories = ["gui", "graphics"]
readme = "README.md"

[dependencies]
gpui = "0.2"
anyhow = "1.0"

[dev-dependencies]
env_logger = "0.11"
```

### 更新 README.md

更新仓库链接和徽章：

```markdown
[![Crates.io](https://img.shields.io/crates/v/rui.svg)](https://crates.io/crates/rui)
[![Documentation](https://docs.rs/rui/badge.svg)](https://docs.rs/rui)
[![License](https://img.shields.io/badge/license-MIT-green.svg)](LICENSE)
```

## 📦 发布到 crates.io

### 1. 注册 crates.io 账号

```bash
# 访问 https://crates.io/
# 使用 GitHub 账号登录

# 获取 API token
# 访问 https://crates.io/settings/tokens
```

### 2. 配置 cargo

```bash
cargo login <your-api-token>
```

### 3. 检查发布准备

```bash
# 确保项目可以构建
cargo build --release

# 运行所有示例
cargo run --example text_input_demo
cargo run --example button_demo

# 检查包内容
cargo package --list

# 干运行发布
cargo publish --dry-run
```

### 4. 发布

```bash
# 发布到 crates.io
cargo publish
```

## 🔗 在其他项目中使用

### 方法 1: 从 crates.io (发布后)

```toml
[dependencies]
rui = "0.1"
gpui = "0.2"
```

### 方法 2: 从 Git 仓库

```toml
[dependencies]
rui = { git = "https://github.com/yourusername/rui" }
gpui = "0.2"
```

### 方法 3: 本地路径 (开发期间)

```toml
[dependencies]
rui = { path = "../rui" }
gpui = "0.2"
```

## 📝 在 z-agent 中使用独立的 RUI

更新 z-agent 的 `Cargo.toml`:

```toml
[dependencies]
# 选项 1: 使用本地路径 (开发)
rui = { path = "../../rui" }

# 选项 2: 使用 Git (发布后)
# rui = { git = "https://github.com/yourusername/rui" }

# 选项 3: 使用 crates.io (发布后)
# rui = "0.1"
```

## 🚀 持续开发

### 更新版本

1. 更新 `CHANGELOG.md`
2. 更新 `Cargo.toml` 中的版本号
3. 提交更改
4. 创建 Git tag

```bash
git add .
git commit -m "Release v0.2.0"
git tag -a v0.2.0 -m "Version 0.2.0"
git push origin main --tags
cargo publish
```

### 语义化版本

- **0.1.x**: Patch - Bug 修复
- **0.x.0**: Minor - 新功能
- **x.0.0**: Major - 破坏性变更

## 📚 文档

### 生成文档

```bash
# 生成并查看文档
cargo doc --open --no-deps

# 为 docs.rs 测试
cargo doc --all-features --no-deps
```

### 添加文档示例

在代码中添加示例：

```rust
/// A button component
/// 
/// # Example
/// 
/// ```
/// use rui::prelude::*;
/// 
/// let button = Button::new("Click me")
///     .variant(ButtonVariant::Primary)
///     .size(ComponentSize::Medium);
/// ```
pub struct Button { }
```

## ✅ 检查清单

独立发布前确保：

- [ ] 所有代码编译通过无警告
- [ ] 所有示例可以运行
- [ ] README.md 完整准确
- [ ] LICENSE 文件存在
- [ ] Cargo.toml 元数据完整
- [ ] .gitignore 配置正确
- [ ] CHANGELOG.md 更新
- [ ] 文档注释完整
- [ ] 代码符合 Rust 规范
- [ ] 版本号正确

## 🤝 贡献指南

创建 `CONTRIBUTING.md` 说明：
- 如何报告问题
- 如何提交 PR
- 代码规范
- 测试要求

## 📄 许可证

确保 LICENSE 文件存在并正确。RUI 使用 MIT 许可证。

---

**祝你成功！** 🎉

如有问题，请查阅：
- [Cargo Book](https://doc.rust-lang.org/cargo/)
- [crates.io 文档](https://doc.rust-lang.org/cargo/reference/publishing.html)
- [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/)
