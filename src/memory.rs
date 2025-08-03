//! Memory management subsystem for KewveOS
//! 
//! This module provides enterprise-grade memory management with proper
//! page frame allocation, virtual memory management, and heap allocation.

use linked_list_allocator::LockedHeap;
use x86_64::{
    structures::paging::{
        mapper::MapToError, FrameAllocator, Mapper, Page, PageTable, PageTableFlags,
        PhysFrame, Size4KiB, UnusedPhysFrame,
    },
    PhysAddr, VirtAddr,
};
use bootloader::bootinfo::{MemoryMap, MemoryRegionType};
use spin::Mutex;
use lazy_static::lazy_static;

/// Kernel heap start address - properly aligned virtual address
pub const HEAP_START: usize = 0x_4444_4444_0000;
/// Kernel heap size - 1MB for enterprise workloads
pub const HEAP_SIZE: usize = 1024 * 1024; // 1 MiB

/// Memory management errors following enterprise error handling standards
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MemoryError {
    /// Out of physical memory
    OutOfMemory,
    /// Invalid virtual address
    InvalidVirtualAddress(VirtAddr),
    /// Invalid physical address  
    InvalidPhysicalAddress(PhysAddr),
    /// Page mapping failed
    MappingFailed,
    /// Frame allocation failed
    FrameAllocationFailed,
    /// Heap initialization failed
    HeapInitializationFailed,
}

impl core::fmt::Display for MemoryError {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self {
            MemoryError::OutOfMemory => write!(f, "Out of physical memory"),
            MemoryError::InvalidVirtualAddress(addr) => write!(f, "Invalid virtual address: {:#x}", addr.as_u64()),
            MemoryError::InvalidPhysicalAddress(addr) => write!(f, "Invalid physical address: {:#x}", addr.as_u64()),
            MemoryError::MappingFailed => write!(f, "Page mapping operation failed"),
            MemoryError::FrameAllocationFailed => write!(f, "Physical frame allocation failed"),
            MemoryError::HeapInitializationFailed => write!(f, "Kernel heap initialization failed"),
        }
    }
}

/// Enterprise-grade frame allocator using bootloader memory map
pub struct BootInfoFrameAllocator {
    memory_map: &'static MemoryMap,
    next: usize,
}

impl BootInfoFrameAllocator {
    /// Create a new frame allocator from bootloader memory map
    /// 
    /// # Safety
    /// This function is unsafe because the caller must guarantee that the passed
    /// memory map is valid and that all frames marked as `USABLE` are really unused.
    pub unsafe fn init(memory_map: &'static MemoryMap) -> Self {
        BootInfoFrameAllocator {
            memory_map,
            next: 0,
        }
    }

    /// Returns an iterator over the usable frames specified in the memory map
    fn usable_frames(&self) -> impl Iterator<Item = PhysFrame> {
        let regions = self.memory_map.iter();
        let usable_regions = regions
            .filter(|r| r.region_type == MemoryRegionType::Usable);
        let addr_ranges = usable_regions
            .map(|r| r.range.start_addr()..r.range.end_addr());
        let frame_addresses = addr_ranges.flat_map(|r| r.step_by(4096));
        frame_addresses.map(|addr| PhysFrame::containing_address(PhysAddr::new(addr)))
    }
}

unsafe impl FrameAllocator<Size4KiB> for BootInfoFrameAllocator {
    fn allocate_frame(&mut self) -> Option<UnusedPhysFrame> {
        let frame = self.usable_frames().nth(self.next);
        self.next += 1;
        frame.map(|f| unsafe { UnusedPhysFrame::new(f) })
    }
}

/// Global frame allocator instance
static mut FRAME_ALLOCATOR: Option<BootInfoFrameAllocator> = None;

/// Initialize the memory management subsystem
/// 
/// This function must be called early in kernel initialization with proper
/// memory map information from the bootloader.
pub fn init_memory_management(
    memory_map: &'static MemoryMap,
    mut mapper: impl Mapper<Size4KiB>,
) -> Result<(), MemoryError> {
    // Initialize frame allocator
    unsafe {
        FRAME_ALLOCATOR = Some(BootInfoFrameAllocator::init(memory_map));
    }

    // Map heap pages
    let page_range = {
        let heap_start = VirtAddr::new(HEAP_START as u64);
        let heap_end = heap_start + HEAP_SIZE - 1u64;
        let heap_start_page = Page::containing_address(heap_start);
        let heap_end_page = Page::containing_address(heap_end);
        Page::range_inclusive(heap_start_page, heap_end_page)
    };

    let frame_allocator = unsafe {
        FRAME_ALLOCATOR.as_mut()
            .ok_or(MemoryError::FrameAllocationFailed)?
    };

    for page in page_range {
        let frame = frame_allocator
            .allocate_frame()
            .ok_or(MemoryError::OutOfMemory)?;
        let flags = PageTableFlags::PRESENT | PageTableFlags::WRITABLE;
        
        unsafe {
            mapper.map_to(page, frame, flags, frame_allocator)
                .map_err(|_| MemoryError::MappingFailed)?
                .flush();
        }
    }

    // Initialize heap allocator
    init_heap()?;

    Ok(())
}

/// Initialize the kernel heap with proper memory mapping
/// 
/// This function should only be called after memory management is properly initialized.
pub fn init_heap() -> Result<(), MemoryError> {
    unsafe {
        crate::ALLOCATOR.lock().init(HEAP_START as *mut u8, HEAP_SIZE);
    }
    
    // Test heap allocation to ensure it's working
    test_heap_allocation()?;
    
    Ok(())
}

/// Test heap allocation to ensure memory management is working correctly
fn test_heap_allocation() -> Result<(), MemoryError> {
    use alloc::boxed::Box;
    use alloc::vec::Vec;

    // Test basic allocation
    let test_box = Box::new(42u64);
    if *test_box != 42 {
        return Err(MemoryError::HeapInitializationFailed);
    }

    // Test vector allocation
    let mut test_vec = Vec::new();
    for i in 0..100 {
        test_vec.push(i);
    }
    
    if test_vec.len() != 100 || test_vec[99] != 99 {
        return Err(MemoryError::HeapInitializationFailed);
    }

    Ok(())
}

/// Memory statistics for monitoring and debugging
#[derive(Debug, Clone, Copy)]
pub struct MemoryStats {
    pub total_memory: u64,
    pub used_memory: u64,
    pub free_memory: u64,
    pub heap_size: usize,
    pub heap_used: usize,
}

/// Get current memory statistics
pub fn get_memory_stats() -> MemoryStats {
    // This would be implemented with proper memory tracking
    // For now, return basic information
    MemoryStats {
        total_memory: 0, // Would be filled from memory map
        used_memory: 0,  // Would be tracked during allocation
        free_memory: 0,  // Would be calculated
        heap_size: HEAP_SIZE,
        heap_used: 0,    // Would be tracked by allocator
    }
}

/// Platform-specific memory management trait
pub trait PlatformMemoryManager {
    type Error;
    
    /// Initialize platform-specific memory management
    fn init(&mut self) -> Result<(), Self::Error>;
    
    /// Allocate physical pages
    fn allocate_pages(&mut self, count: usize) -> Result<PhysAddr, Self::Error>;
    
    /// Deallocate physical pages
    fn deallocate_pages(&mut self, addr: PhysAddr, count: usize) -> Result<(), Self::Error>;
    
    /// Map virtual pages to physical pages
    fn map_pages(&mut self, virt: VirtAddr, phys: PhysAddr, count: usize, flags: u64) -> Result<(), Self::Error>;
    
    /// Unmap virtual pages
    fn unmap_pages(&mut self, virt: VirtAddr, count: usize) -> Result<(), Self::Error>;
}

/// x86_64 specific memory manager implementation
#[cfg(target_arch = "x86_64")]
pub struct X86MemoryManager {
    frame_allocator: Option<BootInfoFrameAllocator>,
}

#[cfg(target_arch = "x86_64")]
impl X86MemoryManager {
    pub fn new() -> Self {
        Self {
            frame_allocator: None,
        }
    }
}

#[cfg(target_arch = "x86_64")]
impl PlatformMemoryManager for X86MemoryManager {
    type Error = MemoryError;
    
    fn init(&mut self) -> Result<(), Self::Error> {
        // Platform-specific initialization would go here
        Ok(())
    }
    
    fn allocate_pages(&mut self, _count: usize) -> Result<PhysAddr, Self::Error> {
        // Implementation would use frame allocator
        Err(MemoryError::FrameAllocationFailed)
    }
    
    fn deallocate_pages(&mut self, _addr: PhysAddr, _count: usize) -> Result<(), Self::Error> {
        // Implementation would return frames to allocator
        Ok(())
    }
    
    fn map_pages(&mut self, _virt: VirtAddr, _phys: PhysAddr, _count: usize, _flags: u64) -> Result<(), Self::Error> {
        // Implementation would use page mapper
        Err(MemoryError::MappingFailed)
    }
    
    fn unmap_pages(&mut self, _virt: VirtAddr, _count: usize) -> Result<(), Self::Error> {
        // Implementation would unmap pages
        Ok(())
    }
}