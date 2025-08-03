#!/bin/bash
# KewveOS Build Script

set -e

TARGET=${1:-x86_64-kewve_os.json}
MODE=${2:-debug}

echo "🔨 Building KewveOS for target: $TARGET"

if [ "$MODE" = "release" ]; then
    echo "🚀 Building in release mode..."
    cargo build --target $TARGET --release
    if [ "$TARGET" = "x86_64-kewve_os.json" ]; then
        cargo bootimage --target $TARGET --release
    fi
else
    echo "🛠️  Building in debug mode..."
    cargo build --target $TARGET
    if [ "$TARGET" = "x86_64-kewve_os.json" ]; then
        cargo bootimage --target $TARGET
    fi
fi

echo "✅ Build complete!"

if [ "$TARGET" = "x86_64-kewve_os.json" ]; then
    echo "💿 Bootable image created at: target/$TARGET/$MODE/bootimage-kewve-os.bin"
    echo "🖥️  Run with: cargo run --target $TARGET"
fi