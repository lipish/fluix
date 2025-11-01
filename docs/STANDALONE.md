# Making Fluix a Standalone Crate

This guide explains how to extract Fluix from the z-agent project as a standalone crate.

## 📋 Preparation

### 1. Create New Git Repository

```bash
# Create new repository on GitHub
# Name: fluix
# Description: A comprehensive UI component library for GPUI 0.2

# Or initialize locally
cd /path/to/new/location
git init fluix
cd fluix
```

### 2. Copy Fluix Code

```bash
# Copy fluix directory from z-agent
cp -r /Users/xinference/github/z-agent/crates/fluix/* .

# Or use git subtree to separate history
cd /Users/xinference/github/z-agent
git subtree split --prefix=crates/fluix -b fluix-standalone
cd /path/to/new/fluix
git pull /Users/xinference/github/z-agent fluix-standalone
```

### 3. Clean Up Standalone Project

```bash
# Make sure all necessary files exist
ls -la

# Should include:
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

## 🔧 Configuration Adjustments

### Update Cargo.toml

Make sure `Cargo.toml` contains correct information:

```toml
[package]
name = "fluix"
version = "0.1.0"
edition = "2024"
authors = ["Your Name <your.email@example.com>"]
description = "A comprehensive UI component library for GPUI 0.2"
license = "MIT"
repository = "https://github.com/yourusername/fluix"
homepage = "https://github.com/yourusername/fluix"
keywords = ["ui", "gui", "gpui", "components", "widgets"]
categories = ["gui", "graphics"]
readme = "README.md"

[dependencies]
gpui = "0.2"
anyhow = "1.0"

[dev-dependencies]
env_logger = "0.11"
```

### Update README.md

Update repository links and badges:

```markdown
[![Crates.io](https://img.shields.io/crates/v/fluix.svg)](https://crates.io/crates/fluix)
[![Documentation](https://docs.rs/fluix/badge.svg)](https://docs.rs/fluix)
[![License](https://img.shields.io/badge/license-MIT-green.svg)](LICENSE)
```

## 📦 Publishing to crates.io

### 1. Register crates.io Account

```bash
# Visit https://crates.io/
# Login with GitHub account

# Get API token
# Visit https://crates.io/settings/tokens
```

### 2. Configure Cargo

```bash
cargo login <your-api-token>
```

### 3. Check Publishing Readiness

```bash
# Make sure project builds
cargo build --release

# Run all examples
cargo run --example text_input_demo
cargo run --example button_demo

# Check package contents
cargo package --list

# Dry run publish
cargo publish --dry-run
```

### 4. Publish

```bash
# Publish to crates.io
cargo publish
```

## 🔗 Using in Other Projects

### Method 1: From crates.io (After Publishing)

```toml
[dependencies]
fluix = "0.1"
gpui = "0.2"
```

### Method 2: From Git Repository

```toml
[dependencies]
fluix = { git = "https://github.com/yourusername/fluix" }
gpui = "0.2"
```

### Method 3: Local Path (During Development)

```toml
[dependencies]
fluix = { path = "../fluix" }
gpui = "0.2"
```

## 📝 Using Standalone Fluix in z-agent

Update z-agent's `Cargo.toml`:

```toml
[dependencies]
# Option 1: Use local path (development)
fluix = { path = "../../fluix" }

# Option 2: Use Git (after publishing)
# fluix = { git = "https://github.com/yourusername/fluix" }

# Option 3: Use crates.io (after publishing)
# fluix = "0.1"
```

## 🚀 Continuous Development

### Version Updates

1. Update `CHANGELOG.md`
2. Update version number in `Cargo.toml`
3. Commit changes
4. Create Git tag

```bash
git add .
git commit -m "Release v0.2.0"
git tag -a v0.2.0 -m "Version 0.2.0"
git push origin main --tags
cargo publish
```

### Semantic Versioning

- **0.1.x**: Patch - Bug fixes
- **0.x.0**: Minor - New features
- **x.0.0**: Major - Breaking changes

## 📚 Documentation

### Generate Documentation

```bash
# Generate and view documentation
cargo doc --open --no-deps

# Test for docs.rs
cargo doc --all-features --no-deps
```

### Add Documentation Examples

Add examples in code:

```rust
/// A button component
/// 
/// # Example
/// 
/// ```
/// use fluix::prelude::*;
/// 
/// let button = Button::new("Click me")
///     .variant(ButtonVariant::Primary)
///     .size(ComponentSize::Medium);
/// ```
pub struct Button { }
```

## ✅ Checklist

Before standalone release, ensure:

- [ ] All code compiles without warnings
- [ ] All examples run
- [ ] README.md is complete and accurate
- [ ] LICENSE file exists
- [ ] Cargo.toml metadata is complete
- [ ] .gitignore is configured correctly
- [ ] CHANGELOG.md is updated
- [ ] Documentation comments are complete
- [ ] Code follows Rust conventions
- [ ] Version number is correct

## 🤝 Contribution Guidelines

Create `CONTRIBUTING.md` explaining:
- How to report issues
- How to submit PRs
- Code standards
- Testing requirements

## 📄 License

Make sure LICENSE file exists and is correct. Fluix uses MIT License.

---

**Good luck!** 🎉

For questions, refer to:
- [Cargo Book](https://doc.rust-lang.org/cargo/)
- [crates.io Documentation](https://doc.rust-lang.org/cargo/reference/publishing.html)
- [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/)
