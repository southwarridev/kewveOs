//! x86_64 platform implementation

use super::{Platform, PlatformInfo, PlatformError};

/// x86_64 platform implementation
pub struct X86_64Platform {
    info: PlatformInfo,
}

impl X86_64Platform {
    /// Create a new x86_64 platform instance
    pub const fn new() -> Self {
        Self {
            info: PlatformInfo {
                architecture: "x86_64",
                vendor: "Generic x86_64",
                features: &[
                    "long_mode",
                    "sse",
                    "apic",
                ],
            },
        }
    }
}

impl Platform for X86_64Platform {
    fn name(&self) -> &'static str {
        "x86_64"
    }
    
    fn init(&mut self) -> Result<(), PlatformError> {
        // Initialize x86_64 specific features
        // This would include things like:
        // - Setting up CPU features
        // - Initializing APIC
        // - Setting up memory management
        
        // For now, we'll just return Ok
        Ok(())
    }
    
    fn halt(&self) -> ! {
        loop {
            x86_64::instructions::hlt();
        }
    }
    
    fn info(&self) -> PlatformInfo {
        self.info.clone()
    }
}

/// Get the CPU vendor string
pub fn get_cpu_vendor() -> [u8; 12] {
    let mut vendor = [0; 12];

    // Skip the CPUID implementation for now to avoid register conflicts
    // In a real implementation, you would use the x86_64::instructions::cpuid module

    vendor
}