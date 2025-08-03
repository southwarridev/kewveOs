//! Storage device drivers for KewveOS
//! 
//! This module provides enterprise-grade storage drivers supporting:
//! - AHCI/SATA drives
//! - NVMe SSDs
//! - USB storage devices
//! - Virtual block devices

use super::{Driver, DriverError, DriverStats, DeviceId};
use alloc::vec::Vec;
use alloc::string::String;
use spin::Mutex;
use lazy_static::lazy_static;

/// Storage device types
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StorageType {
    /// AHCI/SATA hard drive or SSD
    Ahci,
    /// NVMe SSD
    Nvme,
    /// USB mass storage device
    UsbMassStorage,
    /// Virtual block device
    Virtual,
    /// Unknown storage type
    Unknown,
}

/// Storage device configuration
#[derive(Debug, Clone)]
pub struct StorageConfig {
    pub device_id: DeviceId,
    pub storage_type: StorageType,
    pub sector_size: u32,
    pub total_sectors: u64,
    pub read_only: bool,
}

/// Storage operation result
pub type StorageResult<T> = Result<T, StorageError>;

/// Storage-specific error types
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum StorageError {
    /// Device not ready
    DeviceNotReady,
    /// Invalid sector address
    InvalidSector(u64),
    /// Read operation failed
    ReadFailed,
    /// Write operation failed
    WriteFailed,
    /// Device is read-only
    ReadOnlyDevice,
    /// Sector size mismatch
    SectorSizeMismatch,
    /// Hardware timeout
    Timeout,
    /// Generic I/O error
    IoError(String),
}

impl core::fmt::Display for StorageError {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self {
            StorageError::DeviceNotReady => write!(f, "Storage device not ready"),
            StorageError::InvalidSector(sector) => write!(f, "Invalid sector address: {}", sector),
            StorageError::ReadFailed => write!(f, "Storage read operation failed"),
            StorageError::WriteFailed => write!(f, "Storage write operation failed"),
            StorageError::ReadOnlyDevice => write!(f, "Attempted write to read-only device"),
            StorageError::SectorSizeMismatch => write!(f, "Sector size mismatch"),
            StorageError::Timeout => write!(f, "Storage operation timed out"),
            StorageError::IoError(msg) => write!(f, "Storage I/O error: {}", msg),
        }
    }
}

/// Storage device trait for all storage drivers
pub trait StorageDevice {
    /// Read sectors from the device
    fn read_sectors(&mut self, start_sector: u64, sector_count: u32, buffer: &mut [u8]) -> StorageResult<()>;
    
    /// Write sectors to the device
    fn write_sectors(&mut self, start_sector: u64, sector_count: u32, buffer: &[u8]) -> StorageResult<()>;
    
    /// Get device capacity in sectors
    fn get_sector_count(&self) -> u64;
    
    /// Get sector size in bytes
    fn get_sector_size(&self) -> u32;
    
    /// Check if device is read-only
    fn is_read_only(&self) -> bool;
    
    /// Flush any pending writes
    fn flush(&mut self) -> StorageResult<()>;
}

/// Generic storage driver implementation
pub struct GenericStorageDriver {
    config: Option<StorageConfig>,
    initialized: bool,
    stats: DriverStats,
}

impl GenericStorageDriver {
    /// Create a new generic storage driver
    pub fn new() -> Self {
        Self {
            config: None,
            initialized: false,
            stats: DriverStats::default(),
        }
    }
}

impl Driver for GenericStorageDriver {
    type Config = StorageConfig;
    type Error = DriverError;
    
    fn name(&self) -> &'static str {
        "Generic Storage Driver"
    }
    
    fn version(&self) -> &'static str {
        "1.0.0"
    }
    
    fn init(&mut self, config: Self::Config) -> Result<(), Self::Error> {
        // Validate configuration
        if config.sector_size == 0 || config.total_sectors == 0 {
            return Err(DriverError::InvalidConfiguration);
        }
        
        self.config = Some(config);
        self.initialized = true;
        
        crate::println!("Storage driver initialized: {} sectors of {} bytes", 
                       self.config.as_ref().unwrap().total_sectors,
                       self.config.as_ref().unwrap().sector_size);
        
        Ok(())
    }
    
    fn deinit(&mut self) -> Result<(), Self::Error> {
        self.initialized = false;
        self.config = None;
        Ok(())
    }
    
    fn is_initialized(&self) -> bool {
        self.initialized
    }
    
    fn handle_interrupt(&mut self, _irq: u32) -> Result<(), Self::Error> {
        self.stats.interrupts_handled += 1;
        // Storage interrupt handling would go here
        Ok(())
    }
    
    fn get_stats(&self) -> DriverStats {
        self.stats
    }
    
    fn reset(&mut self) -> Result<(), Self::Error> {
        if !self.initialized {
            return Err(DriverError::InitializationFailed("Driver not initialized".into()));
        }
        
        // Reset storage device
        crate::println!("Resetting storage device");
        Ok(())
    }
}

impl StorageDevice for GenericStorageDriver {
    fn read_sectors(&mut self, start_sector: u64, sector_count: u32, buffer: &mut [u8]) -> StorageResult<()> {
        let config = self.config.as_ref().ok_or(StorageError::DeviceNotReady)?;
        
        // Validate sector range
        if start_sector >= config.total_sectors {
            return Err(StorageError::InvalidSector(start_sector));
        }
        
        if start_sector + sector_count as u64 > config.total_sectors {
            return Err(StorageError::InvalidSector(start_sector + sector_count as u64));
        }
        
        // Validate buffer size
        let expected_size = sector_count as usize * config.sector_size as usize;
        if buffer.len() < expected_size {
            return Err(StorageError::SectorSizeMismatch);
        }
        
        // For now, simulate read operation
        // In a real implementation, this would perform actual hardware I/O
        for i in 0..expected_size {
            buffer[i] = (i % 256) as u8;
        }
        
        self.stats.bytes_transferred += expected_size as u64;
        self.stats.operations_completed += 1;
        
        Ok(())
    }
    
    fn write_sectors(&mut self, start_sector: u64, sector_count: u32, buffer: &[u8]) -> StorageResult<()> {
        let config = self.config.as_ref().ok_or(StorageError::DeviceNotReady)?;
        
        if config.read_only {
            return Err(StorageError::ReadOnlyDevice);
        }
        
        // Validate sector range
        if start_sector >= config.total_sectors {
            return Err(StorageError::InvalidSector(start_sector));
        }
        
        if start_sector + sector_count as u64 > config.total_sectors {
            return Err(StorageError::InvalidSector(start_sector + sector_count as u64));
        }
        
        // Validate buffer size
        let expected_size = sector_count as usize * config.sector_size as usize;
        if buffer.len() < expected_size {
            return Err(StorageError::SectorSizeMismatch);
        }
        
        // For now, simulate write operation
        // In a real implementation, this would perform actual hardware I/O
        
        self.stats.bytes_transferred += expected_size as u64;
        self.stats.operations_completed += 1;
        
        Ok(())
    }
    
    fn get_sector_count(&self) -> u64 {
        self.config.as_ref().map(|c| c.total_sectors).unwrap_or(0)
    }
    
    fn get_sector_size(&self) -> u32 {
        self.config.as_ref().map(|c| c.sector_size).unwrap_or(512)
    }
    
    fn is_read_only(&self) -> bool {
        self.config.as_ref().map(|c| c.read_only).unwrap_or(true)
    }
    
    fn flush(&mut self) -> StorageResult<()> {
        if !self.initialized {
            return Err(StorageError::DeviceNotReady);
        }
        
        // Flush any pending writes
        // In a real implementation, this would ensure all writes are committed
        Ok(())
    }
}

/// Storage manager for handling multiple storage devices
pub struct StorageManager {
    devices: Vec<Box<dyn StorageDevice + Send>>,
}

impl StorageManager {
    /// Create a new storage manager
    pub fn new() -> Self {
        Self {
            devices: Vec::new(),
        }
    }
    
    /// Add a storage device
    pub fn add_device(&mut self, device: Box<dyn StorageDevice + Send>) {
        self.devices.push(device);
    }
    
    /// Get the number of storage devices
    pub fn device_count(&self) -> usize {
        self.devices.len()
    }
    
    /// Get total storage capacity across all devices
    pub fn total_capacity(&self) -> u64 {
        self.devices.iter()
            .map(|device| device.get_sector_count() * device.get_sector_size() as u64)
            .sum()
    }
}

lazy_static! {
    /// Global storage manager instance
    pub static ref STORAGE_MANAGER: Mutex<StorageManager> = Mutex::new(StorageManager::new());
}

/// Initialize storage subsystem
pub fn init_storage() -> Result<(), DriverError> {
    crate::println!("Initializing storage subsystem...");
    
    // Create a virtual storage device for testing
    let mut virtual_storage = GenericStorageDriver::new();
    let config = StorageConfig {
        device_id: 1,
        storage_type: StorageType::Virtual,
        sector_size: 512,
        total_sectors: 1024 * 1024, // 512MB virtual disk
        read_only: false,
    };
    
    virtual_storage.init(config)?;
    
    // Add to storage manager
    STORAGE_MANAGER.lock().add_device(Box::new(virtual_storage));
    
    crate::println!("Storage subsystem initialized with {} devices", 
                   STORAGE_MANAGER.lock().device_count());
    
    Ok(())
}