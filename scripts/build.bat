@echo off
REM KewveOS Build Script for Windows

set TARGET=%1
set MODE=%2

if "%TARGET%"=="" set TARGET=x86_64-kewve_os.json
if "%MODE%"=="" set MODE=debug

echo 🔨 Building KewveOS for target: %TARGET%

if "%MODE%"=="release" (
    echo 🚀 Building in release mode...
    cargo build --target %TARGET% --release
    if "%TARGET%"=="x86_64-kewve_os.json" (
        cargo bootimage --target %TARGET% --release
    )
) else (
    echo 🛠️  Building in debug mode...
    cargo build --target %TARGET%
    if "%TARGET%"=="x86_64-kewve_os.json" (
        cargo bootimage --target %TARGET%
    )
)

echo ✅ Build complete!

if "%TARGET%"=="x86_64-kewve_os.json" (
    echo 💿 Bootable image created at: target\%TARGET%\%MODE%\bootimage-kewve-os.bin
    echo 🖥️  Run with: cargo run --target %TARGET%
)