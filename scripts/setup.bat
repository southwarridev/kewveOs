@echo off
REM KewveOS Development Setup Script for Windows

echo 🚀 Setting up KewveOS development environment...

REM Check if Rust is installed
rustc --version >nul 2>&1
if %errorlevel% neq 0 (
    echo ❌ Rust is not installed. Please install Rust first:
    echo    https://rustup.rs/
    exit /b 1
)

echo ✅ Rust found
rustc --version

REM Install nightly toolchain
echo 📦 Installing Rust nightly toolchain...
rustup toolchain install nightly
rustup default nightly

REM Add required components
echo 🔧 Adding required components...
rustup component add rust-src
rustup component add llvm-tools-preview
rustup component add clippy
rustup component add rustfmt

REM Install bootimage
echo 💿 Installing bootimage...
cargo install bootimage

echo ✅ Development environment setup complete!
echo.
echo 🎯 Next steps:
echo    1. Run 'cargo build' to build the kernel
echo    2. Run 'cargo bootimage' to create a bootable image
echo    3. Run 'cargo run' to test in QEMU
echo.
echo 📚 For more information, see the README.md file
pause