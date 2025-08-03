# 🌐 KewveOS

> **A Universal, Decentralized Operating System Built in Rust**

[![CI](https://github.com/kewve/kewve-os/workflows/KewveOS%20CI/badge.svg)](https://github.com/kewve/kewve-os/actions)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

KewveOS is a revolutionary operating system that breaks down the walls between platforms. Built entirely in **Rust** for memory safety and performance, it enables users to run **Android, iOS, Windows, Linux, macOS, and Web applications** on a single, unified platform.

## ✨ Features

- 🛡️ **Memory-Safe Kernel** - Built in Rust with zero C/C++ in core
- 🌍 **Universal App Compatibility** - Run apps from all major platforms
- 📱 **Cross-Platform** - Supports both PC (x86_64) and Mobile (ARM64)
- 🔒 **Container Security** - Sandboxed app execution with capability-based permissions
- 🎨 **GPU-Accelerated UI** - 60+ FPS graphics using wgpu/WebGPU
- 🌐 **Decentralized Services** - P2P app distribution and identity
- 📖 **Open Source** - MIT License, community-driven development

## 🚀 Quick Start

### Prerequisites

- **Rust Nightly** toolchain
- **QEMU** for testing (optional but recommended)
- **Git** for version control

### Setup Development Environment

#### Windows
```cmd
# Run the setup script
scripts\setup.bat

# Or manually:
rustup toolchain install nightly
rustup default nightly
rustup component add rust-src llvm-tools-preview
cargo install bootimage
```

#### Linux/macOS
```bash
# Run the setup script
chmod +x scripts/setup.sh
./scripts/setup.sh

# Or manually:
rustup toolchain install nightly
rustup default nightly
rustup component add rust-src llvm-tools-preview
cargo install bootimage
```

### Build and Run

```bash
# Build the kernel
cargo build

# Create bootable image (x86_64 only)
cargo bootimage

# Run in QEMU
cargo run

# Build for specific target
cargo build --target x86_64-kewve_os.json
cargo build --target aarch64-kewve_os.json
```

### Using Build Scripts

#### Windows
```cmd
# Build debug version
scripts\build.bat

# Build release version
scripts\build.bat x86_64-kewve_os.json release

# Build for ARM64
scripts\build.bat aarch64-kewve_os.json
```

#### Linux/macOS
```bash
# Build debug version
./scripts/build.sh

# Build release version
./scripts/build.sh x86_64-kewve_os.json release

# Build for ARM64
./scripts/build.sh aarch64-kewve_os.json
```

## 🏗️ Architecture

KewveOS follows a layered architecture designed for security, performance, and maintainability:

```
┌─────────────────────────────────────────────────────────────┐
│                    USER APPLICATIONS                        │
│  WASM Apps │ JS Apps │ Win32 Apps │ Linux Apps │ macOS Apps │
├─────────────────────────────────────────────────────────────┤
│                  APPLICATION RUNTIME LAYER                  │
│  WASM Runtime │ JS Engine │ Win32 Emu │ ELF Loader │ Mach-O │
├─────────────────────────────────────────────────────────────┤
│                   GRAPHICS & UI SYSTEM                      │
│  Window Manager │ Compositor │ Input System │ Shell/Launcher │
├─────────────────────────────────────────────────────────────┤
│                    SYSTEM SERVICES                          │
│  File System │ Network Stack │ Audio │ Power Management     │
├─────────────────────────────────────────────────────────────┤
│                      KERNEL LAYER                           │
│  Process Mgr │ Memory Mgr │ Scheduler │ IPC │ System Calls  │
├─────────────────────────────────────────────────────────────┤
│                HARDWARE ABSTRACTION LAYER                   │
│  Platform HAL │ Device Drivers │ Interrupt Handlers         │
├─────────────────────────────────────────────────────────────┤
│                        HARDWARE                             │
│        PC (x86_64)        │        Phone (ARM64)            │
└─────────────────────────────────────────────────────────────┘
```

## 📁 Project Structure

```
kewve-os/
├── .cargo/                  # Cargo configuration
├── .github/                 # CI/CD workflows
├── .kiro/                   # Kiro IDE specifications
│   └── specs/
│       └── kewve-os-mvp/
├── scripts/                 # Build and setup scripts
├── src/                     # Source code
│   ├── main.rs             # Kernel entry point
│   └── lib.rs              # Library root
├── target/                  # Build artifacts
├── Cargo.toml              # Project configuration
├── x86_64-kewve_os.json    # x86_64 target specification
├── aarch64-kewve_os.json   # ARM64 target specification
└── README.md               # This file
```

## 🔄 Development Phases

### Phase 1: Core OS Foundation ✅ (In Progress)
- [x] Project infrastructure and build system
- [ ] Kernel initialization and memory management
- [ ] Interrupt handling and device drivers
- [ ] File system and process management

### Phase 2: Graphics and UI (Planned)
- [ ] GPU drivers and wgpu integration
- [ ] Window management system
- [ ] Input handling and shell environment

### Phase 3: Application Runtime (Planned)
- [ ] WASM and JavaScript runtime
- [ ] Platform compatibility layers
- [ ] Security and sandboxing

### Phase 4: Integration and Polish (Planned)
- [ ] Testing framework and debugging tools
- [ ] Performance optimization
- [ ] Documentation and release

## 🧪 Testing

```bash
# Run unit tests
cargo test

# Run integration tests
cargo test --test integration

# Run tests for specific target
cargo test --target x86_64-kewve_os.json

# Run with coverage
cargo tarpaulin --out Html
```

## 🤝 Contributing

We welcome contributions! Please see our [Contributing Guidelines](CONTRIBUTING.md) for details.

### Development Workflow

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Make your changes
4. Add tests for your changes
5. Ensure all tests pass (`cargo test`)
6. Commit your changes (`git commit -m 'Add amazing feature'`)
7. Push to the branch (`git push origin feature/amazing-feature`)
8. Open a Pull Request

### Code Standards

- Follow Rust idioms and best practices
- Write comprehensive tests for new features
- Document public APIs with rustdoc
- Minimize unsafe code and document when necessary
- Use `cargo fmt` and `cargo clippy` before committing

## 📚 Documentation

- [Requirements Specification](.kiro/specs/kewve-os-mvp/requirements.md)
- [Design Document](.kiro/specs/kewve-os-mvp/design.md)
- [Implementation Plan](.kiro/specs/kewve-os-mvp/tasks.md)
- [API Documentation](https://docs.rs/kewve-os) (Coming Soon)

## 🛠️ Debugging

### Serial Output
The kernel outputs debug information via serial port. In QEMU, this appears in the terminal.

### GDB Debugging
```bash
# Start QEMU with GDB server
qemu-system-x86_64 -s -S -drive format=raw,file=target/x86_64-kewve_os/debug/bootimage-kewve-os.bin

# In another terminal, connect GDB
gdb target/x86_64-kewve_os/debug/kewve-os
(gdb) target remote :1234
(gdb) continue
```

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 🌟 Vision

KewveOS aims to create a **secure, universal, and user-owned operating system** that:
- Eliminates platform lock-in
- Provides memory-safe computing
- Enables decentralized app distribution
- Empowers users with digital freedom

> **"One OS. All Apps. Zero Walls."**

## 🔗 Links

- **Website**: [https://kewve.org](https://kewve.org) (Coming Soon)
- **Documentation**: [https://docs.kewve.org](https://docs.kewve.org) (Coming Soon)
- **Community**: [Discord](https://discord.gg/kewve) (Coming Soon)
- **Issues**: [GitHub Issues](https://github.com/kewve/kewve-os/issues)

---

**KewveOS - The Universal, Decentralized Operating System.**  
*Built in Rust. Owned by the People. Open for All.*