#![no_std]
#![feature(custom_test_frameworks)]
#![feature(alloc_error_handler)]
#![feature(abi_x86_interrupt)]

extern crate alloc;

pub mod serial;
pub mod vga_buffer;
pub mod memory;
pub mod interrupts;
pub mod platform;
pub mod drivers;
pub mod process;

use core::panic::PanicInfo;
use linked_list_allocator::LockedHeap;

/// The global allocator for the kernel heap.
#[global_allocator]
static ALLOCATOR: LockedHeap = LockedHeap::empty();

/// A function to halt the CPU indefinitely
pub fn hlt_loop() -> ! {
    loop {
        x86_64::instructions::hlt();
    }
}

#[alloc_error_handler]
fn alloc_error_handler(layout: alloc::alloc::Layout) -> ! {
    panic!("Allocation error: {:?}", layout)
}

/// This function is called on panic.
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("KERNEL PANIC: {}", info);
    serial_println!("KERNEL PANIC: {}", info);
    
    hlt_loop();
}