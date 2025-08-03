# Kewve OS — Dependencies

All dependencies are **Rust crates** (no C/C++), prioritizing safety, performance, and portability.

---

## 🔧 Core Dependencies

| Crate | Version | Purpose | Safety |
|------|--------|--------|--------|
| `bootloader` | `0.10` | Generates UEFI bootable image | ✅ Safe |
| `x86_64` | `0.14` | CPU instructions, registers, interrupts | ✅ Safe (minimal unsafe) |
| `volatile` | `0.2` | Safe volatile access to memory | ✅ |
| `spin` | `0.9` | Spinlocks for no-std | ✅ |
| `uart_16550` | `0.2` | Serial output for debugging | ✅ |
| `pic8259` | `0.10` | Legacy interrupt controller | ✅ |

---

## 🖼️ Graphics & UI

| Crate | Version | Purpose |
|------|--------|--------|
| `wgpu` | `0.17` | Cross-platform GPU rendering (WebGPU) |
| `winit` | `0.28` | Windowing and input events |
| `futures` | `0.3` | Async support for GPU tasks |
| `pollster` | `0.3` | Run async on bare metal |

> Eventually: `wgpu-hal` for direct driver access.

---

## ⚙️ Runtime & Apps

| Crate | Version | Purpose |
|------|--------|--------|
| `wasm3` | `0.4` | Fast WASM interpreter |
| `wasmtime` | `14.0` | (Optional) JIT for high-performance WASM |
| `rquickjs` | `0.6` | Embed QuickJS for JS apps |
| `skia-safe` | `0.80` | Skia graphics for Flutter-like rendering |
| `flutter-rs` | (custom) | Experimental Flutter embedding |

---

## 🌐 Networking

| Crate | Version | Purpose |
|------|--------|--------|
| `smoltcp` | `0.8` | Pure-Rust TCP/IP stack |
| `rustls` | `0.21` | TLS 1.3 for HTTPS |
| `http` | `1.0` | HTTP types |

---

## 📦 Tooling

| Tool | Purpose |
|------|--------|
| `bootimage` | Build bootable disk |
| `cargo-xbuild` | Cross-compile for bare metal |
| `qemu-system-x86_64` | Emulation |
| `flamegraph` | Performance profiling |

---

## 🚫 Excluded Dependencies

| Why Rejected | Reason |
|------------|--------|
| `libc` | Not needed; no C ABI |
| `std` | Disabled (`no_std`) |
| `tokio` | Too heavy; custom async runtime planned |
| `openssl` | Avoid C; use `rustls` instead |

---

> All dependencies are audited for `unsafe` code via `cargo-geiger`.