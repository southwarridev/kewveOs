//! Enterprise-grade device driver framework for Kewve OS
//! 
//! This module provides a comprehensive driver framework with:
//! - Cross-platform driver abstraction
//! - Hot-pluggable device support
//! - Resource management and cleanup
//! - Event-driven architecture
//! - Performance monitoring

pub mod keyboard;
pub mod timer;
pub mod storage;
pub mod input;

use alloc::string::String;
use alloc::vec::Vec;
use alloc::collections::BTreeMap;
use spin::Mutex;
use lazy_static::lazy_static;

/// Device identifier type
pub type DeviceId = u64;

/// Generic driver trait for all device drivers
pub trait Driver {
    type Config;
    type Error;
    
    /// Get the driver name
    fn name(&self) -> &'static str;
    
    /// Get the driver version
    fn version(&self) -> &'static str;
    
    /// Initialize the driver with configuration
    fn init(&mut self, config: Self::Config) -> Result<(), Self::Error>;
    
    /// Deinitialize the driver and cleanup resources
    fn deinit(&mut self) -> Result<(), Self::Error>;
    
    /// Check if the driver is initialized
    fn is_initialized(&self) -> bool;
    
    /// Handle device interrupt
    fn handle_interrupt(&mut self, irq: u32) -> Result<(), Self::Error>;
    
    /// Get driver statistics
    fn get_stats(&self) -> DriverStats;
    
    /// Reset the device
    fn reset(&mut self) -> Result<(), Self::Error>;
}

/// Driver error types with comprehensive error handling
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DriverError {
    /// Driver initialization failed
    InitializationFailed(String),
    /// Device not present or not responding
    DeviceNotPresent,
    /// I/O operation failed
    IoError(String),
    /// Unsupported operation for this driver
    UnsupportedOperation,
    /// Invalid configuration provided
    InvalidConfiguration,
    /// Resource allocation failed
    ResourceAllocationFailed,
    /// Interrupt handling failed
    InterruptHandlingFailed,
    /// Device timeout
    Timeout,
    /// Hardware error detected
    HardwareError(String),
}

impl core::fmt::Display for DriverError {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self {
            DriverError::InitializationFailed(msg) => write!(f, "Driver initialization failed: {}", msg),
            DriverError::DeviceNotPresent => write!(f, "Device not present or not responding"),
            DriverError::IoError(msg) => write!(f, "I/O operation failed: {}", msg),
            DriverError::UnsupportedOperation => write!(f, "Unsupported operation for this driver"),
            DriverError::InvalidConfiguration => write!(f, "Invalid configuration provided"),
            DriverError::ResourceAllocationFailed => write!(f, "Resource allocation failed"),
            DriverError::InterruptHandlingFailed => write!(f, "Interrupt handling failed"),
            DriverError::Timeout => write!(f, "Device operation timed out"),
            DriverError::HardwareError(msg) => write!(f, "Hardware error: {}", msg),
        }
    }
}

/// Driver performance and diagnostic statistics
#[derive(Debug, Clone, Copy)]
pub struct DriverStats {
    pub interrupts_handled: u64,
    pub errors_encountered: u64,
    pub bytes_transferred: u64,
    pub operations_completed: u64,
    pub last_error: Option<u64>, // Error code timestamp
}

impl Default for DriverStats {
    fn default() -> Self {
        Self {
            interrupts_handled: 0,
            errors_encountered: 0,
            bytes_transferred: 0,
            operations_completed: 0,
            last_error: None,
        }
    }
}

/// Device types supported by the system
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DeviceType {
    Keyboard,
    Mouse,
    TouchScreen,
    Timer,
    Storage,
    Network,
    Audio,
    Graphics,
    Serial,
    Unknown,
}

/// Device descriptor containing device information
#[derive(Debug, Clone)]
pub struct DeviceDescriptor {
    pub id: DeviceId,
    pub device_type: DeviceType,
    pub name: String,
    pub vendor: String,
    pub version: String,
    pub capabilities: Vec<String>,
    pub initialized: bool,
}

/// Device manager for handling all system devices
pub struct DeviceManager {
    devices: BTreeMap<DeviceId, DeviceDescriptor>,
    next_device_id: DeviceId,
}

impl DeviceManager {
    /// Create a new device manager
    pub fn new() -> Self {
        Self {
            devices: BTreeMap::new(),
            next_device_id: 1,
        }
    }
    
    /// Register a new device
    pub fn register_device(&mut self, device_type: DeviceType, name: String, vendor: String) -> DeviceId {
        let device_id = self.next_device_id;
        self.next_device_id += 1;
        
        let descriptor = DeviceDescriptor {
            id: device_id,
            device_type,
            name,
            vendor,
            version: String::from("1.0.0"),
            capabilities: Vec::new(),
            initialized: false,
        };
        
        self.devices.insert(device_id, descriptor);
        device_id
    }
    
    /// Unregister a device
    pub fn unregister_device(&mut self, device_id: DeviceId) -> Option<DeviceDescriptor> {
        self.devices.remove(&device_id)
    }
    
    /// Get device descriptor
    pub fn get_device(&self, device_id: DeviceId) -> Option<&DeviceDescriptor> {
        self.devices.get(&device_id)
    }
    
    /// List all devices of a specific type
    pub fn list_devices_by_type(&self, device_type: DeviceType) -> Vec<&DeviceDescriptor> {
        self.devices.values()
            .filter(|device| device.device_type == device_type)
            .collect()
    }
    
    /// Mark device as initialized
    pub fn mark_initialized(&mut self, device_id: DeviceId) {
        if let Some(device) = self.devices.get_mut(&device_id) {
            device.initialized = true;
        }
    }
}

lazy_static! {
    /// Global device manager instance
    pub static ref DEVICE_MANAGER: Mutex<DeviceManager> = Mutex::new(DeviceManager::new());
}

/// Input event types for unified input handling
#[derive(Debug, Clone, Copy)]
pub enum InputEvent {
    /// Keyboard key press/release
    KeyEvent {
        scancode: u8,
        pressed: bool,
        modifiers: KeyModifiers,
    },
    /// Mouse movement
    MouseMove {
        delta_x: i32,
        delta_y: i32,
    },
    /// Mouse button press/release
    MouseButton {
        button: MouseButton,
        pressed: bool,
    },
    /// Touch screen event
    TouchEvent {
        id: u32,
        x: f32,
        y: f32,
        pressure: f32,
        event_type: TouchEventType,
    },
}

/// Key modifier flags
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct KeyModifiers {
    pub shift: bool,
    pub ctrl: bool,
    pub alt: bool,
    pub meta: bool,
}

impl Default for KeyModifiers {
    fn default() -> Self {
        Self {
            shift: false,
            ctrl: false,
            alt: false,
            meta: false,
        }
    }
}

/// Mouse button types
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MouseButton {
    Left,
    Right,
    Middle,
    Other(u8),
}

/// Touch event types
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TouchEventType {
    Down,
    Move,
    Up,
    Cancel,
}

/// Input event handler trait
pub trait InputEventHandler {
    /// Handle an input event
    fn handle_input_event(&mut self, event: InputEvent) -> Result<(), DriverError>;
}

/// Initialize the device driver framework
pub fn init_driver_framework() -> Result<(), DriverError> {
    crate::println!("Initializing device driver framework...");
    
    // Register built-in device types
    let mut device_manager = DEVICE_MANAGER.lock();
    
    // Register timer device
    device_manager.register_device(
        DeviceType::Timer,
        String::from("System Timer"),
        String::from("KewveOS"),
    );
    
    // Register keyboard device
    device_manager.register_device(
        DeviceType::Keyboard,
        String::from("PS/2 Keyboard"),
        String::from("Generic"),
    );
    
    crate::println!("Device driver framework initialized successfully");
    Ok(())
}