# GPUI Asset Loading Guide

## Overview

This document explains how to correctly load SVG icons and other assets in GPUI applications.

## Core Concepts

### AssetSource

GPUI uses the `AssetSource` trait to load assets. `svg().path()` does not directly read the file system, but instead calls `AssetSource::load()` to get the asset content.

```rust
pub trait AssetSource {
    fn load(&self, path: &str) -> Result<Option<Cow<'static, [u8]>>>;
    fn list(&self, path: &str) -> Result<Vec<SharedString>>;
}
```

### Workflow

```
svg().path("icons/arrow.svg")
    ↓
cx.asset_source().load("icons/arrow.svg")
    ↓
Returns SVG byte data
    ↓
usvg/resvg renders to pixels
    ↓
Displayed on screen
```

## Two Implementation Approaches

### Approach A: Embedded Assets (Recommended, used by Zed)

#### Advantages
- ✅ Resources bundled into binary, no additional files needed
- ✅ Simple deployment, single executable file
- ✅ Fast loading speed
- ✅ Cross-platform consistent

#### Implementation Steps

**1. Add Dependencies**

```toml
[dependencies]
rust-embed = "8"
```

**2. Create Directory Structure**

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

**3. Implement AssetSource**

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

**4. Register AssetSource**

```rust
// src/main.rs or examples/xxx.rs
use gpui::*;

fn main() {
    let app = Application::new()
        .with_assets(your_crate::Assets);  // Key!
    
    app.run(|cx| {
        cx.open_window(window_options, |window, cx| {
            cx.new(|cx| YourView::new(window, cx))
        }).unwrap();
    });
}
```

**5. Use Assets**

```rust
// Load SVG icon
svg().path("icons/arrow-down.svg")

// Load image
img("images/logo.png")
```

### Approach B: Filesystem Loading (For Development)

#### Advantages
- ✅ Update assets without recompiling
- ✅ Good for development and debugging

#### Disadvantages
- ❌ Need to distribute asset files
- ❌ Complex path management

#### Implementation Steps

**1. Implement Filesystem AssetSource**

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

**2. Register**

```rust
fn main() {
    let app = Application::new()
        .with_assets(your_crate::FsAssets::new());
    
    app.run(|cx| {
        // ...
    });
}
```

## Common Questions

### Q: Why isn't my SVG displaying?

**A:** Check the following:

1. **Is AssetSource registered?**
   ```rust
   // Must call with_assets()
   Application::new().with_assets(Assets)
   ```

2. **Is the path correct?**
   ```rust
   // Path relative to assets/ directory
   svg().path("icons/arrow.svg")  // ✅
   svg().path("assets/icons/arrow.svg")  // ❌
   ```

3. **Does the file exist?**
   ```bash
   ls assets/icons/arrow.svg
   ```

4. **Is it included in the embed rules?**
   ```rust
   #[derive(RustEmbed)]
   #[folder = "assets"]
   #[include = "icons/**/*"]  // Make sure your files are included
   ```

### Q: How to debug asset loading?

**A:** Add debug code:

```rust
// After application startup
if let Some(data) = cx.asset_source().load("icons/arrow.svg")? {
    println!("✅ Icon loaded: {} bytes", data.len());
} else {
    println!("❌ Icon not found");
}
```

### Q: Can I mix embedded and filesystem assets?

**A:** Yes, implement a combined AssetSource:

```rust
pub struct HybridAssets {
    embedded: Assets,
    fs: FsAssets,
}

impl AssetSource for HybridAssets {
    fn load(&self, path: &str) -> Result<Option<Cow<'static, [u8]>>> {
        // Try filesystem first (for development)
        if let Some(data) = self.fs.load(path)? {
            return Ok(Some(data));
        }
        // Fall back to embedded assets (for production)
        self.embedded.load(path)
    }
    
    // ...
}
```

### Q: How to add new icons?

**A:** 

1. Place SVG file in `assets/icons/`
2. Add new item to `IconName` enum
3. Add mapping in `path()` method
4. Recompile (for embedded approach) or use directly (for filesystem approach)

## Performance Considerations

### Embedded Assets
- **Compile Time**: Slower initial compile, fast incremental compile
- **Binary Size**: Increases (each SVG ~1-5KB)
- **Runtime Performance**: Fastest, assets in memory

### Filesystem
- **Compile Time**: Fast
- **Binary Size**: Small
- **Runtime Performance**: Requires I/O, slightly slower

## Best Practices

1. **Use Embedded Assets for Production**
   - Simple deployment
   - Best performance

2. **Optional Filesystem for Development**
   - Fast iteration
   - No recompilation needed

3. **Use Conditional Compilation**
   ```rust
   #[cfg(debug_assertions)]
   let assets = FsAssets::new();
   
   #[cfg(not(debug_assertions))]
   let assets = Assets;
   
   Application::new().with_assets(assets)
   ```

4. **Organize Asset Directories**
   ```
   assets/
   ├── icons/      # SVG icons
   ├── images/     # PNG/JPG images
   ├── fonts/      # Font files
   └── data/       # Other data
   ```

## References

- [rust-embed Documentation](https://docs.rs/rust-embed/)
- [GPUI Examples](https://github.com/zed-industries/zed/tree/main/crates/gpui/examples)
- [Zed Asset Implementation](https://github.com/zed-industries/zed/blob/main/crates/zed/src/zed.rs)
