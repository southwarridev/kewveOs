#!/bin/bash
# KewveOS Code Validation Script

set -e

echo "🔍 Validating KewveOS code structure..."

# Check if Rust is available
if ! command -v rustc &> /dev/null; then
    echo "❌ Rust not found. Please run setup.sh first."
    echo "   Or install Rust from https://rustup.rs/"
    exit 1
fi

echo "✅ Rust toolchain found: $(rustc --version)"

# Check if we're using nightly
if ! rustc --version | grep -q "nightly"; then
    echo "⚠️  Warning: Not using nightly toolchain"
    echo "   Run: rustup default nightly"
fi

# Check required components
echo "🔧 Checking required components..."
if ! rustup component list --installed | grep -q "rust-src"; then
    echo "❌ rust-src component missing"
    echo "   Run: rustup component add rust-src"
    exit 1
fi

if ! rustup component list --installed | grep -q "llvm-tools-preview"; then
    echo "❌ llvm-tools-preview component missing"
    echo "   Run: rustup component add llvm-tools-preview"
    exit 1
fi

echo "✅ Required components found"

# Check if bootimage is installed
if ! command -v bootimage &> /dev/null; then
    echo "❌ bootimage not found"
    echo "   Run: cargo install bootimage"
    exit 1
fi

echo "✅ bootimage found: $(bootimage --version)"

# Try to build
echo "🔨 Attempting to build..."
if ! cargo check --target x86_64-kewve_os.json; then
    echo "❌ Build check failed"
    exit 1
fi

echo "✅ Build check passed"

# Try to create bootimage
echo "💿 Attempting to create bootimage..."
if ! cargo bootimage --target x86_64-kewve_os.json; then
    echo "❌ Bootimage creation failed"
    exit 1
fi

echo "✅ Bootimage creation successful"

echo "🎉 All validations passed!"
echo "🚀 KewveOS is ready to run!"
echo ""
echo "Next steps:"
echo "  - Run: cargo run --target x86_64-kewve_os.json"
echo "  - Or use QEMU directly with the generated bootimage"