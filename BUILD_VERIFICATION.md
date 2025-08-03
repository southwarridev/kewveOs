# KewveOS Build Verification Guide

This guide helps you verify that your KewveOS build environment is working correctly.

## Prerequisites Check

Before building, ensure you have:

1. **Rust Nightly Toolchain**
   ```bash
   rustc --version
   # Should show: rustc 1.x.x-nightly
   ```

2. **Required Components**
   ```bash
   rustup component list --installed
   # Should include: rust-src, llvm-tools-preview
   ```

3. **Bootimage Tool**
   ```bash
   bootimage --version
   # Should show version information
   ```

## Automated Validation

Run the validation script for your platform:

### Windows
```cmd
scripts\validate.bat
```

### Linux/macOS
```bash
chmod +x scripts/validate.sh
./scripts/validate.sh
```

## Manual Build Steps

If you prefer to build manually:

### 1. Check Code Syntax
```bash
cargo check --target x86_64-kewve_os.json
```
This should complete without errors.

### 2. Build the Kernel
```bash
cargo build --target x86_64-kewve_os.json
```
This creates the kernel binary.

### 3. Create Bootable Image
```bash
cargo bootimage --target x86_64-kewve_os.json
```
This creates a bootable disk image.

### 4. Run in QEMU
```bash
cargo run --target x86_64-kewve_os.json
```
This should boot KewveOS in QEMU and display:
```
🌐 KewveOS v0.1.0
🚀 Universal Operating System - Built in Rust
📱 Supporting PC (x86_64) and Mobile (ARM64) platforms

🔧 Initializing KewveOS kernel...
🖥️  Detected x86_64 platform
✅ Basic kernel initialization complete
✅ Kernel initialization complete
🔄 Entering main kernel loop...
```

## Expected Build Artifacts

After a successful build, you should have:

```
target/
└── x86_64-kewve_os/
    └── debug/
        ├── kewve-os              # Kernel binary
        ├── bootimage-kewve-os.bin # Bootable disk image
        └── deps/                 # Dependencies
```

## Cross-Platform Building

### ARM64 Build (for mobile platforms)
```bash
cargo build --target aarch64-kewve_os.json
```

Note: ARM64 builds don't create bootimages yet, as that requires additional setup.

## Troubleshooting Common Issues

### 1. "cargo: command not found"
- **Solution**: Install Rust from [rustup.rs](https://rustup.rs/)
- **Windows**: Restart your terminal after installation

### 2. "rust-src component not found"
- **Solution**: 
  ```bash
  rustup default nightly
  rustup component add rust-src
  ```

### 3. "bootimage not found"
- **Solution**: 
  ```bash
  cargo install bootimage
  ```

### 4. "linker errors" or "undefined symbols"
- **Solution**: Make sure you're using the correct target:
  ```bash
  cargo build --target x86_64-kewve_os.json
  ```

### 5. "feature not found" errors
- **Solution**: Ensure you're using nightly Rust:
  ```bash
  rustup default nightly
  ```

### 6. QEMU not starting
- **Solution**: Install QEMU for your platform:
  - **Windows**: Download from [qemu.org](https://www.qemu.org/download/#windows)
  - **Linux**: `sudo apt install qemu-system-x86` (Ubuntu/Debian)
  - **macOS**: `brew install qemu`

## Testing the Build

### Run Unit Tests
```bash
cargo test --target x86_64-kewve_os.json
```

### Run Integration Tests
```bash
cargo test --test integration --target x86_64-kewve_os.json
```

## Performance Verification

### Build Time
A clean build should complete in under 2 minutes on modern hardware.

### Boot Time
KewveOS should boot in QEMU within 3-5 seconds.

### Memory Usage
The kernel binary should be under 1MB in debug mode.

## Success Indicators

✅ **Build Success**: No compilation errors or warnings  
✅ **Bootimage Creation**: Bootable image created successfully  
✅ **QEMU Boot**: OS boots and displays welcome message  
✅ **Tests Pass**: All unit tests execute successfully  
✅ **Clean Output**: No panic messages or crashes  

## Next Steps

Once build verification is complete:

1. **Explore the Code**: Review `src/main.rs`, `src/lib.rs`, and modules
2. **Run Tests**: Execute the test suite to ensure everything works
3. **Next Task**: Proceed to Task 3 in the implementation plan
4. **Documentation**: Read the design and requirements documents

## Getting Help

If you encounter issues:

1. Check this troubleshooting guide
2. Review the [SETUP.md](SETUP.md) file
3. Consult the [README.md](README.md) for general information
4. Open an issue on GitHub with your error details

Remember: KewveOS is an operating system kernel, so it requires specific tooling and setup. The validation scripts help ensure everything is configured correctly before you start development.

Happy coding! 🚀