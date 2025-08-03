#!/bin/bash
# KewveOS Development Setup Script

set -e

echo "🚀 Setting up KewveOS development environment..."

# Check if Rust is installed
if ! command -v rustc &> /dev/null; then
    echo "❌ Rust is not installed. Please install Rust first:"
    echo "   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh"
    exit 1
fi

echo "✅ Rust found: $(rustc --version)"

# Install nightly toolchain
echo "📦 Installing Rust nightly toolchain..."
rustup toolchain install nightly
rustup default nightly

# Add required components
echo "🔧 Adding required components..."
rustup component add rust-src
rustup component add llvm-tools-preview
rustup component add clippy
rustup component add rustfmt

# Install bootimage
echo "💿 Installing bootimage..."
cargo install bootimage

# Install QEMU (if on Linux)
if [[ "$OSTYPE" == "linux-gnu"* ]]; then
    echo "🖥️  Installing QEMU..."
    if command -v apt-get &> /dev/null; then
        sudo apt-get update
        sudo apt-get install -y qemu-system-x86 qemu-system-arm
    elif command -v dnf &> /dev/null; then
        sudo dnf install -y qemu-system-x86 qemu-system-aarch64
    elif command -v pacman &> /dev/null; then
        sudo pacman -S qemu-arch-extra
    else
        echo "⚠️  Please install QEMU manually for your distribution"
    fi
fi

echo "✅ Development environment setup complete!"
echo ""
echo "🎯 Next steps:"
echo "   1. Run 'cargo build' to build the kernel"
echo "   2. Run 'cargo bootimage' to create a bootable image"
echo "   3. Run 'cargo run' to test in QEMU"
echo ""
echo "📚 For more information, see the README.md file"