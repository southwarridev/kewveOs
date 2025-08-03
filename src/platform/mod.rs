/// Platform abstraction layer for Kewve OS
/// 
/// This module provides traits and implementations for platform-specific functionality
/// to enable cross-platform compatibility.

pub mod x86_64;

use core::fmt;

/// Trait defining platform-specific functionality
pub trait Platform {
    /// Get the platform name
    fn name(&self) -> &'static str;
    
    /// Initialize the platform
    fn init(&mut self) -> Result<(), PlatformError>;
    
    /// Halt the CPU
    fn halt(&self) -> !;
    
    /// Get platform-specific information
    fn info(&self) -> PlatformInfo;
}

/// Platform-specific information
#[derive(Debug, Clone)]
pub struct PlatformInfo {
    pub architecture: &'static str,
    pub vendor: &'static str,
    pub features: &'static [&'static str],
}

/// Error type for platform operations
#[derive(Debug, Clone)]
pub enum PlatformError {
    InitializationFailed,
    UnsupportedFeature,
    HardwareError,
}

impl fmt::Display for PlatformError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            PlatformError::InitializationFailed => write!(f, "Platform initialization failed"),
            PlatformError::UnsupportedFeature => write!(f, "Unsupported platform feature"),
            PlatformError::HardwareError => write!(f, "Platform hardware error"),
        }
    }
}

/// Detect the current platform
pub fn detect_platform() -> Option<&'static str> {
    // For now, we only support x86_64
    // In the future, this could detect other platforms
    Some("x86_64")
}