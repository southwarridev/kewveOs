@echo off
REM KewveOS Code Validation Script

echo 🔍 Validating KewveOS code structure...

REM Check if Rust is available
rustc --version >nul 2>&1
if %errorlevel% neq 0 (
    echo ❌ Rust not found. Please run setup.bat first.
    echo    Or install Rust from https://rustup.rs/
    exit /b 1
)

echo ✅ Rust toolchain found

REM Check if we're using nightly
rustc --version | findstr "nightly" >nul
if %errorlevel% neq 0 (
    echo ⚠️  Warning: Not using nightly toolchain
    echo    Run: rustup default nightly
)

REM Check required components
echo 🔧 Checking required components...
rustup component list --installed | findstr "rust-src" >nul
if %errorlevel% neq 0 (
    echo ❌ rust-src component missing
    echo    Run: rustup component add rust-src
    exit /b 1
)

rustup component list --installed | findstr "llvm-tools-preview" >nul
if %errorlevel% neq 0 (
    echo ❌ llvm-tools-preview component missing  
    echo    Run: rustup component add llvm-tools-preview
    exit /b 1
)

echo ✅ Required components found

REM Check if bootimage is installed
bootimage --version >nul 2>&1
if %errorlevel% neq 0 (
    echo ❌ bootimage not found
    echo    Run: cargo install bootimage
    exit /b 1
)

echo ✅ bootimage found

REM Try to build
echo 🔨 Attempting to build...
cargo check --target x86_64-kewve_os.json
if %errorlevel% neq 0 (
    echo ❌ Build check failed
    exit /b 1
)

echo ✅ Build check passed

REM Try to create bootimage
echo 💿 Attempting to create bootimage...
cargo bootimage --target x86_64-kewve_os.json
if %errorlevel% neq 0 (
    echo ❌ Bootimage creation failed
    exit /b 1
)

echo ✅ Bootimage creation successful

echo 🎉 All validations passed!
echo 🚀 KewveOS is ready to run!
echo.
echo Next steps:
echo   - Run: cargo run --target x86_64-kewve_os.json
echo   - Or use QEMU directly with the generated bootimage