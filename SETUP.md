# KewveOS Development Setup Guide

This guide will help you set up your development environment for KewveOS.

## Prerequisites

### 1. Install Rust

#### Windows
1. Download and run the installer from [rustup.rs](https://rustup.rs/)
2. Follow the installation prompts
3. Restart your terminal/command prompt

#### Linux/macOS
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source ~/.cargo/env
```

### 2. Configure Rust for OS Development

```bash
# Install nightly toolchain (required for OS development)
rustup toolchain install nightly
rustup default nightly

# Add required components
rustup component add rust-src
rustup component add llvm-tools-preview
rustup component add clippy
rustup component add rustfmt

# Install bootimage tool
cargo install bootimage
```

### 3. Install QEMU (Optional but Recommended)

#### Windows
1. Download QEMU from [qemu.org](https://www.qemu.org/download/#windows)
2. Install and add to PATH
3. Or use Chocolatey: `choco install qemu`

#### Linux
```bash
# Ubuntu/Debian
sudo apt-get install qemu-system-x86 qemu-system-arm

# Fedora
sudo dnf install qemu-system-x86 qemu-system-aarch64

# Arch Linux
sudo pacman -S qemu-arch-extra
```

#### macOS
```bash
# Using Homebrew
brew install qemu
```

## Verification

After installation, verify your setup:

```bash
# Check Rust version
rustc --version
# Should show: rustc 1.x.x-nightly

# Check cargo version
cargo --version

# Check required components
rustup component list --installed
# Should include: rust-src, llvm-tools-preview

# Check bootimage installation
cargo bootimage --version
```

## First Build

Once everything is installed:

```bash
# Clone the repository (if not already done)
git clone <repository-url>
cd kewve-os

# Build the kernel
cargo build

# Create bootable image (x86_64 only)
cargo bootimage

# Run in QEMU
cargo run
```

## Troubleshooting

### Common Issues

1. **"cargo: command not found"**
   - Restart your terminal after installing Rust
   - Make sure `~/.cargo/bin` is in your PATH

2. **"rust-src component not found"**
   - Make sure you're using nightly: `rustup default nightly`
   - Install the component: `rustup component add rust-src`

3. **"bootimage not found"**
   - Install bootimage: `cargo install bootimage`
   - Make sure `~/.cargo/bin` is in your PATH

4. **Build errors about "no_std"**
   - This is normal for OS development
   - Make sure you're using the nightly toolchain

### Getting Help

- Check the [README.md](README.md) for basic usage
- Review the [requirements](.kiro/specs/kewve-os-mvp/requirements.md) and [design](.kiro/specs/kewve-os-mvp/design.md) documents
- Open an issue on GitHub if you encounter problems

## Next Steps

After setup is complete:

1. Read through the project documentation
2. Review the implementation plan in [tasks.md](.kiro/specs/kewve-os-mvp/tasks.md)
3. Start with Task 2: "Implement basic kernel entry point and initialization"
4. Join our community discussions (links in README.md)

Happy coding! 🚀