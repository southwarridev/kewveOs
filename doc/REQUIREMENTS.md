
---

## ✅ `REQUIREMENTS.md`

```markdown
# Kewve OS — Requirements Specification

This document outlines the functional and non-functional requirements for Kewve OS.

---

## 🎯 Vision

To create a **secure, fast, and beautiful OS** where **web and cross-platform apps run natively**, powered by **Rust** and **GPU-first rendering**.

---

## 📋 Functional Requirements

| ID | Requirement | Priority |
|----|-----------|----------|
| FR-1 | Boot on x86_64 and aarch64 | High |
| FR-2 | Display text and graphics via UEFI/GOP | High |
| FR-3 | Run WebAssembly (.wasm) modules securely | High |
| FR-4 | Execute JavaScript (PWA/web apps) | Medium |
| FR-5 | Support Flutter apps via Skia embedding | Medium |
| FR-6 | App sandboxing with capability-based permissions | High |
| FR-7 | IPC between apps and system services | High |
| FR-8 | Windowed UI shell with compositor | Medium |
| FR-9 | Network stack (TCP/IP, HTTPS) | Medium |
| FR-10 | Package manager for `.kapp` bundles | Low |
| FR-11 | Touch and keyboard input support | Medium |

---

## 🔐 Non-Functional Requirements

| ID | Requirement | Priority |
|----|-----------|----------|
| NFR-1 | Memory safety (no UB, no C) | Critical |
| NFR-2 | Boot time < 3 seconds (on Pi 5) | Medium |
| NFR-3 | GPU-accelerated UI (60 FPS) | High |
| NFR-4 | Attack surface < 10K LOC in kernel | High |
| NFR-5 | No root/admin privileges | High |
| NFR-6 | Support QEMU, Raspberry Pi 4/5, Intel NUC | Medium |
| NFR-7 | Max kernel size: 1MB | Medium |
| NFR-8 | Energy efficient (for handhelds) | Medium |
| NFR-9 | Modular, extensible architecture | High |
| NFR-10 | Developer-friendly SDK & tooling | High |

---

## 🧩 Constraints

- No use of C/C++ (except in GPU drivers via FFI)
- Must compile with `#![forbid(unsafe_code)]` where possible
- Target embedded systems (2GB RAM minimum)
- Apps cannot access hardware directly

---

## 🔄 Future Requirements

- Voice input
- AI assistant integration
- Multi-user support
- Offline-first PWAs