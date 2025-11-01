# GPUI 资源加载指南

## 概述

本文档说明如何在 GPUI 应用中正确加载 SVG 图标和其他资源。

## 核心概念

### AssetSource

GPUI 使用 `AssetSource` trait 来加载资源。`svg().path()` 不会直接读取文件系统，而是调用 `AssetSource::load()` 来获取资源内容。

```rust
pub trait AssetSource {
    fn load(&self, path: &str) -> Result<Option<Cow<'static, [u8]>>>;
    fn list(&self, path: &str) -> Result<Vec<SharedString>>;
}
```

### 工作流程

```
svg().path("icons/arrow.svg")
    ↓
cx.asset_source().load("icons/arrow.svg")
    ↓
返回 SVG 字节数据
    ↓
usvg/resvg 渲染成像素
    ↓
显示在屏幕上
```

## 两种实现方式

### 方案 A：嵌入资源（推荐，Zed 使用的方式）

#### 优点
- ✅ 资源打包到二进制，无需额外文件
- ✅ 部署简单，单个可执行文件
- ✅ 加载速度快
- ✅ 跨平台一致

#### 实现步骤

**1. 添加依赖**

```toml
[dependencies]
rust-embed = "8"
```

**2. 创建目录结构**

```
your-project/
├── assets/
│   ├── icons/
│   │   ├── arrow-down.svg
│   │   ├── check.svg
│   │   └── ...
│   ├── images/
│   └── fonts/
└── src/
    ├── assets.rs
    └── ...
```

**3. 实现 AssetSource**

```rust
// src/assets.rs
use gpui::{AssetSource, Result, SharedString};
use rust_embed::RustEmbed;
use std::borrow::Cow;

#[derive(RustEmbed)]
#[folder = "assets"]
#[include = "icons/**/*"]
#[include = "images/**/*"]
#[include = "fonts/**/*"]
pub struct Assets;

impl AssetSource for Assets {
    fn load(&self, path: &str) -> Result<Option<Cow<'static, [u8]>>> {
        Ok(Self::get(path).map(|f| f.data))
    }

    fn list(&self, path: &str) -> Result<Vec<SharedString>> {
        Ok(Self::iter()
            .filter(|p| p.starts_with(path))
            .map(SharedString::from)
            .collect())
    }
}
```

**4. 注册 AssetSource**

```rust
// src/main.rs 或 examples/xxx.rs
use gpui::*;

fn main() {
    let app = Application::new()
        .with_assets(your_crate::Assets);  // 关键！
    
    app.run(|cx| {
        cx.open_window(window_options, |window, cx| {
            cx.new(|cx| YourView::new(window, cx))
        }).unwrap();
    });
}
```

**5. 使用资源**

```rust
// 加载 SVG 图标
svg().path("icons/arrow-down.svg")

// 加载图片
img("images/logo.png")
```

### 方案 B：文件系统加载（开发时使用）

#### 优点
- ✅ 无需重新编译即可更新资源
- ✅ 适合开发和调试

#### 缺点
- ❌ 需要分发资源文件
- ❌ 路径管理复杂

#### 实现步骤

**1. 实现文件系统 AssetSource**

```rust
// src/assets.rs
use gpui::{AssetSource, Result, SharedString};
use std::{borrow::Cow, fs, path::{Path, PathBuf}};

pub struct FsAssets {
    base: PathBuf,
}

impl FsAssets {
    pub fn new() -> Self {
        Self {
            base: PathBuf::from(env!("CARGO_MANIFEST_DIR")),
        }
    }
}

impl AssetSource for FsAssets {
    fn load(&self, path: &str) -> Result<Option<Cow<'static, [u8]>>> {
        let p = Path::new(path);
        let full = if p.is_absolute() {
            p.to_path_buf()
        } else {
            self.base.join("assets").join(p)
        };
        
        match fs::read(full) {
            Ok(data) => Ok(Some(Cow::Owned(data))),
            Err(_) => Ok(None),
        }
    }

    fn list(&self, path: &str) -> Result<Vec<SharedString>> {
        let dir = self.base.join("assets").join(path);
        let entries = fs::read_dir(dir)?
            .filter_map(|e| e.ok())
            .filter_map(|e| e.file_name().into_string().ok())
            .map(SharedString::from)
            .collect();
        Ok(entries)
    }
}
```

**2. 注册**

```rust
fn main() {
    let app = Application::new()
        .with_assets(your_crate::FsAssets::new());
    
    app.run(|cx| {
        // ...
    });
}
```

## 常见问题

### Q: 为什么我的 SVG 不显示？

**A:** 检查以下几点：

1. **是否注册了 AssetSource？**
   ```rust
   // 必须调用 with_assets()
   Application::new().with_assets(Assets)
   ```

2. **路径是否正确？**
   ```rust
   // 路径相对于 assets/ 目录
   svg().path("icons/arrow.svg")  // ✅
   svg().path("assets/icons/arrow.svg")  // ❌
   ```

3. **文件是否存在？**
   ```bash
   ls assets/icons/arrow.svg
   ```

4. **是否包含在嵌入规则中？**
   ```rust
   #[derive(RustEmbed)]
   #[folder = "assets"]
   #[include = "icons/**/*"]  // 确保包含你的文件
   ```

### Q: 如何调试资源加载？

**A:** 添加调试代码：

```rust
// 在应用启动后
if let Some(data) = cx.asset_source().load("icons/arrow.svg")? {
    println!("✅ Icon loaded: {} bytes", data.len());
} else {
    println!("❌ Icon not found");
}
```

### Q: 可以混合使用嵌入和文件系统吗？

**A:** 可以，实现一个组合的 AssetSource：

```rust
pub struct HybridAssets {
    embedded: Assets,
    fs: FsAssets,
}

impl AssetSource for HybridAssets {
    fn load(&self, path: &str) -> Result<Option<Cow<'static, [u8]>>> {
        // 先尝试文件系统（开发时）
        if let Some(data) = self.fs.load(path)? {
            return Ok(Some(data));
        }
        // 回退到嵌入资源（生产环境）
        self.embedded.load(path)
    }
    
    // ...
}
```

### Q: 如何添加新图标？

**A:** 

1. 将 SVG 文件放到 `assets/icons/`
2. 在 `IconName` 枚举中添加新项
3. 在 `path()` 方法中添加映射
4. 重新编译（嵌入方案）或直接使用（文件系统方案）

## 性能考虑

### 嵌入资源
- **编译时间**：首次编译较慢，增量编译快
- **二进制大小**：增加（每个 SVG 约 1-5KB）
- **运行时性能**：最快，资源在内存中

### 文件系统
- **编译时间**：快
- **二进制大小**：小
- **运行时性能**：需要 I/O，稍慢

## 最佳实践

1. **生产环境使用嵌入资源**
   - 部署简单
   - 性能最佳

2. **开发环境可选文件系统**
   - 快速迭代
   - 无需重新编译

3. **使用条件编译**
   ```rust
   #[cfg(debug_assertions)]
   let assets = FsAssets::new();
   
   #[cfg(not(debug_assertions))]
   let assets = Assets;
   
   Application::new().with_assets(assets)
   ```

4. **组织资源目录**
   ```
   assets/
   ├── icons/      # SVG 图标
   ├── images/     # PNG/JPG 图片
   ├── fonts/      # 字体文件
   └── data/       # 其他数据
   ```

## 参考

- [rust-embed 文档](https://docs.rs/rust-embed/)
- [GPUI 示例](https://github.com/zed-industries/zed/tree/main/crates/gpui/examples)
- [Zed 资源实现](https://github.com/zed-industries/zed/blob/main/crates/zed/src/zed.rs)

