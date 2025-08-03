# KewveOS MVP - Design Document

## Overview

KewveOS is designed as a modern, memory-safe operating system built entirely in Rust, targeting both PC (x86_64) and mobile (ARM64) platforms. The design follows a layered architecture with clear separation of concerns, enabling incremental development while maintaining security, performance, and cross-platform compatibility.

The system is built around three core principles:
1. **Memory Safety First**: Leveraging Rust's ownership system to eliminate entire classes of vulnerabilities
2. **Cross-Platform by Design**: Unified codebase with platform-specific optimizations
3. **Modular Architecture**: Clean interfaces enabling independent development and testing of components

## Architecture

### System Architecture Overview

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
└─────────────────────────────────────────────────────────────┤
```

### Cross-Platform Design Strategy

The architecture uses Rust's trait system to abstract platform differences:

```rust
// Platform abstraction trait
pub trait Platform {
    type MemoryManager: MemoryManager;
    type InterruptController: InterruptController;
    type Timer: Timer;
    
    fn init() -> Result<Self, PlatformError>;
    fn memory_manager(&self) -> &Self::MemoryManager;
    fn interrupt_controller(&self) -> &Self::InterruptController;
}

// Platform-specific implementations
#[cfg(target_arch = "x86_64")]
pub struct X86Platform {
    memory_manager: X86MemoryManager,
    interrupt_controller: X86InterruptController,
}

#[cfg(target_arch = "aarch64")]
pub struct ArmPlatform {
    memory_manager: ArmMemoryManager,
    interrupt_controller: ArmInterruptController,
}
```

## Components and Interfaces

### 1. Hardware Abstraction Layer (HAL)

**Purpose**: Provide a unified interface to hardware across different architectures.

**Key Components**:
- **Platform HAL**: Abstract interface for platform-specific operations
- **Device Drivers**: Modular driver system for various hardware components
- **Interrupt Management**: Platform-specific interrupt handling

**Interface Design**:
```rust
pub trait DeviceDriver {
    type Config;
    type Error;
    
    fn init(config: Self::Config) -> Result<Self, Self::Error>;
    fn name(&self) -> &'static str;
    fn handle_interrupt(&mut self, irq: u32) -> Result<(), Self::Error>;
}

pub trait InterruptController {
    fn enable_interrupt(&mut self, irq: u32) -> Result<(), InterruptError>;
    fn disable_interrupt(&mut self, irq: u32) -> Result<(), InterruptError>;
    fn register_handler(&mut self, irq: u32, handler: InterruptHandler);
}
```

### 2. Kernel Layer

**Purpose**: Core OS functionality including process management, memory management, and system calls.

**Key Components**:
- **Memory Manager**: Virtual memory, paging, heap allocation
- **Process Manager**: Task creation, scheduling, lifecycle management
- **System Call Interface**: Secure kernel-userspace communication
- **IPC System**: Inter-process communication mechanisms

**Memory Management Design**:
```rust
pub trait MemoryManager {
    fn allocate_pages(&mut self, count: usize) -> Result<PhysAddr, MemoryError>;
    fn deallocate_pages(&mut self, addr: PhysAddr, count: usize) -> Result<(), MemoryError>;
    fn map_pages(&mut self, virt: VirtAddr, phys: PhysAddr, count: usize, flags: PageFlags) -> Result<(), MemoryError>;
    fn unmap_pages(&mut self, virt: VirtAddr, count: usize) -> Result<(), MemoryError>;
}

pub struct KernelHeap {
    allocator: Mutex<LinkedListAllocator>,
    start: VirtAddr,
    size: usize,
}
```

**Process Management Design**:
```rust
pub struct Process {
    pid: ProcessId,
    state: ProcessState,
    memory_space: AddressSpace,
    file_descriptors: Vec<FileDescriptor>,
    capabilities: CapabilitySet,
}

pub trait Scheduler {
    fn schedule(&mut self) -> Option<ProcessId>;
    fn add_process(&mut self, process: Process);
    fn remove_process(&mut self, pid: ProcessId) -> Option<Process>;
    fn yield_current(&mut self);
}
```

### 3. Graphics and UI System

**Purpose**: Hardware-accelerated graphics rendering and user interface management.

**Key Components**:
- **GPU Drivers**: Platform-specific graphics hardware support
- **wgpu Integration**: Cross-platform graphics API abstraction
- **Window Manager**: Window creation, management, and compositing
- **Input System**: Keyboard, mouse, and touch input handling

**Graphics Architecture**:
```rust
pub trait GraphicsDriver {
    fn initialize(&mut self) -> Result<(), GraphicsError>;
    fn create_surface(&mut self, width: u32, height: u32) -> Result<Surface, GraphicsError>;
    fn present(&mut self, surface: &Surface) -> Result<(), GraphicsError>;
}

pub struct WindowManager {
    windows: HashMap<WindowId, Window>,
    compositor: Compositor,
    input_manager: InputManager,
}

pub struct Compositor {
    device: wgpu::Device,
    queue: wgpu::Queue,
    surface: wgpu::Surface,
    render_pipeline: wgpu::RenderPipeline,
}
```

**Input System Design**:
```rust
pub enum InputEvent {
    KeyPress { key: KeyCode, modifiers: Modifiers },
    KeyRelease { key: KeyCode, modifiers: Modifiers },
    MouseMove { x: i32, y: i32 },
    MouseButton { button: MouseButton, pressed: bool },
    TouchStart { id: u32, x: f32, y: f32 },
    TouchMove { id: u32, x: f32, y: f32 },
    TouchEnd { id: u32 },
}

pub trait InputHandler {
    fn handle_input(&mut self, event: InputEvent) -> Result<(), InputError>;
}
```

### 4. Application Runtime Infrastructure

**Purpose**: Secure execution environment for applications from different platforms.

**Key Components**:
- **WASM Runtime**: WebAssembly execution with wasmtime
- **JavaScript Engine**: JS execution with QuickJS integration
- **Platform Compatibility**: Win32, Linux, macOS API emulation
- **Sandboxing**: Container-based application isolation

**Runtime Architecture**:
```rust
pub trait ApplicationRuntime {
    type Application;
    type Error;
    
    fn load_application(&mut self, binary: &[u8]) -> Result<Self::Application, Self::Error>;
    fn execute(&mut self, app: &mut Self::Application) -> Result<(), Self::Error>;
    fn terminate(&mut self, app: &Self::Application) -> Result<(), Self::Error>;
}

pub struct WasmRuntime {
    engine: wasmtime::Engine,
    store: wasmtime::Store<WasmContext>,
    linker: wasmtime::Linker<WasmContext>,
}

pub struct Win32Emulator {
    api_handlers: HashMap<String, Box<dyn Win32ApiHandler>>,
    process_context: Win32ProcessContext,
}
```

### 5. System Services

**Purpose**: High-level OS services like filesystem, networking, and audio.

**Key Components**:
- **Virtual File System**: Unified file access across different filesystems
- **Network Stack**: TCP/IP implementation with security features
- **Audio System**: Cross-platform audio input/output
- **Power Management**: Battery and thermal management for mobile platforms

**File System Design**:
```rust
pub trait FileSystem {
    fn open(&mut self, path: &str, flags: OpenFlags) -> Result<FileHandle, FsError>;
    fn read(&mut self, handle: FileHandle, buffer: &mut [u8]) -> Result<usize, FsError>;
    fn write(&mut self, handle: FileHandle, data: &[u8]) -> Result<usize, FsError>;
    fn close(&mut self, handle: FileHandle) -> Result<(), FsError>;
    fn create_directory(&mut self, path: &str) -> Result<(), FsError>;
}

pub struct VirtualFileSystem {
    mount_points: HashMap<String, Box<dyn FileSystem>>,
    open_files: HashMap<FileHandle, OpenFile>,
}
```

## Data Models

### Core Data Structures

**Process Representation**:
```rust
#[derive(Debug, Clone)]
pub struct ProcessDescriptor {
    pub pid: ProcessId,
    pub parent_pid: Option<ProcessId>,
    pub state: ProcessState,
    pub priority: Priority,
    pub memory_usage: MemoryStats,
    pub cpu_time: Duration,
    pub capabilities: CapabilitySet,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProcessState {
    Ready,
    Running,
    Blocked,
    Terminated,
}
```

**Memory Management Structures**:
```rust
#[derive(Debug, Clone, Copy)]
pub struct PageFrame {
    pub address: PhysAddr,
    pub flags: PageFlags,
    pub ref_count: u32,
}

#[derive(Debug)]
pub struct AddressSpace {
    pub page_table: PageTable,
    pub heap_start: VirtAddr,
    pub heap_end: VirtAddr,
    pub stack_start: VirtAddr,
    pub stack_end: VirtAddr,
}
```

**Graphics Data Models**:
```rust
#[derive(Debug, Clone)]
pub struct Window {
    pub id: WindowId,
    pub title: String,
    pub position: Point2D<i32>,
    pub size: Size2D<u32>,
    pub visible: bool,
    pub surface: Option<wgpu::Surface>,
}

#[derive(Debug, Clone, Copy)]
pub struct Vertex {
    pub position: [f32; 3],
    pub color: [f32; 4],
    pub tex_coords: [f32; 2],
}
```

## Error Handling

### Comprehensive Error System

The system uses a hierarchical error handling approach with specific error types for each subsystem:

```rust
#[derive(Debug, thiserror::Error)]
pub enum KernelError {
    #[error("Memory management error: {0}")]
    Memory(#[from] MemoryError),
    
    #[error("Process management error: {0}")]
    Process(#[from] ProcessError),
    
    #[error("Graphics error: {0}")]
    Graphics(#[from] GraphicsError),
    
    #[error("Platform-specific error: {0}")]
    Platform(#[from] PlatformError),
    
    #[error("I/O error: {0}")]
    Io(#[from] IoError),
}

#[derive(Debug, thiserror::Error)]
pub enum MemoryError {
    #[error("Out of memory")]
    OutOfMemory,
    
    #[error("Invalid address: {address:#x}")]
    InvalidAddress { address: u64 },
    
    #[error("Page fault at address {address:#x}")]
    PageFault { address: u64 },
    
    #[error("Memory protection violation")]
    ProtectionViolation,
}
```

### Error Recovery Strategies

1. **Graceful Degradation**: When non-critical components fail, the system continues with reduced functionality
2. **Process Isolation**: Application failures are contained and don't affect the kernel or other processes
3. **Resource Cleanup**: Automatic cleanup of resources when processes terminate or fail
4. **Logging and Diagnostics**: Comprehensive error logging for debugging and monitoring

## Testing Strategy

### Multi-Level Testing Approach

**1. Unit Testing**:
```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_memory_allocation() {
        let mut allocator = BuddyAllocator::new();
        let page = allocator.allocate_pages(1).expect("Failed to allocate page");
        assert!(page.is_valid());
        allocator.deallocate_pages(page, 1).expect("Failed to deallocate page");
    }
    
    #[test]
    fn test_process_creation() {
        let mut scheduler = RoundRobinScheduler::new();
        let process = Process::new(ProcessId(1), "test_process");
        scheduler.add_process(process);
        assert_eq!(scheduler.process_count(), 1);
    }
}
```

**2. Integration Testing**:
```rust
#[test]
fn test_graphics_pipeline() {
    let mut graphics = GraphicsSystem::new();
    graphics.initialize().expect("Graphics initialization failed");
    
    let window = graphics.create_window("Test Window", 800, 600)
        .expect("Window creation failed");
    
    graphics.render_frame(&window).expect("Rendering failed");
}
```

**3. Platform-Specific Testing**:
```rust
#[cfg(target_arch = "x86_64")]
#[test]
fn test_x86_interrupt_handling() {
    let mut controller = X86InterruptController::new();
    controller.enable_interrupt(32).expect("Failed to enable interrupt");
    // Test interrupt handling logic
}

#[cfg(target_arch = "aarch64")]
#[test]
fn test_arm_exception_handling() {
    let mut controller = ArmInterruptController::new();
    controller.setup_exception_vectors().expect("Failed to setup exception vectors");
    // Test exception handling logic
}
```

**4. Hardware-in-the-Loop Testing**:
- QEMU-based automated testing for both x86_64 and ARM64
- Real hardware testing on development boards
- Performance benchmarking across different hardware configurations

### Continuous Integration Strategy

```yaml
# CI Pipeline Structure
stages:
  - build_x86_64
  - build_aarch64
  - unit_tests
  - integration_tests
  - qemu_tests
  - performance_tests
  - security_analysis
  - documentation_build
```

This design provides a solid foundation for building KewveOS incrementally while maintaining high code quality, security, and cross-platform compatibility. The modular architecture enables parallel development of different components while ensuring they integrate seamlessly.