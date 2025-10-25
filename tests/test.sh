#!/bin/bash

# RUI 独立测试脚本

set -e

echo "🔍 检查 RUI crate..."
echo ""

cd "$(dirname "$0")"

echo "✅ 1. 代码检查..."
cargo check --all-targets

echo ""
echo "✅ 2. 构建 release..."
cargo build --release

echo ""
echo "✅ 3. 运行测试..."
# cargo test (暂无测试)

echo ""
echo "✅ 4. 检查示例..."
cargo build --examples

echo ""
echo "✅ 5. 生成文档..."
cargo doc --no-deps --document-private-items

echo ""
echo "✅ 6. 检查发布准备..."
cargo package --list

echo ""
echo "🎉 所有检查通过！"
echo ""
echo "📦 可以发布的文件:"
cargo package --list | head -20

echo ""
echo "💡 提示:"
echo "  - 运行示例: cargo run --example button_demo"
echo "  - 查看文档: cargo doc --open"
echo "  - 发布测试: cargo publish --dry-run"
echo "  - 正式发布: cargo publish"
