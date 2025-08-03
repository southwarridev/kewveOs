#![no_std]
#![no_main]
#![feature(abi_x86_interrupt)]

extern crate alloc;

mod vga_buffer;
mod serial;
mod memory;
mod interrupts;
mod platform;
mod drivers;
mod process;

use uart_16550::SerialPort;
use alloc::boxed::Box;
use crate::drivers::Driver;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    // Initialize VGA buffer
    println!("Kewve OS is booting...");
    
    // Initialize memory management
    // TODO: Get memory map from bootloader and initialize proper memory management
    // For now, use simplified heap initialization
    memory::init_heap()
        .expect("Heap initialization failed");
    println!("Heap initialized successfully");
    
    // Initialize platform
    let platform_name = platform::detect_platform().unwrap_or("unknown");
    println!("Detected platform: {}", platform_name);
    
    // Initialize interrupts
    interrupts::init_idt();
    println!("IDT initialized successfully");
    
    // Initialize PIC
    unsafe {
        interrupts::pic::PICS.lock().initialize();
    }
    println!("PIC initialized successfully");
    
    // Initialize drivers
    drivers::timer::SYSTEM_TIMER.lock().init()
        .expect("Timer initialization failed");
    println!("Timer initialized successfully");
    
    drivers::keyboard::KEYBOARD.lock().init()
        .expect("Keyboard initialization failed");
    println!("Keyboard initialized successfully");
    
    // Initialize process management
    process::init();
    
    // Enable interrupts
    x86_64::instructions::interrupts::enable();
    println!("Interrupts enabled");
    
    // Initialize serial port
    let mut serial_port = unsafe { SerialPort::new(0x3F8) };
    serial_port.init();
    serial_println!("Kewve OS serial output initialized");
    
    // Test heap allocation
    let x = Box::new(42);
    println!("Boxed value: {}", x);
    
    // Test process creation
    let pid1 = process::create_process(alloc::string::String::from("test_process_1"));
    let pid2 = process::create_process(alloc::string::String::from("test_process_2"));
    println!("Created processes with PIDs: {}, {}", pid1, pid2);
    
    #[cfg(test)]
    test_main();
    
    println!("Kewve OS initialization complete!");
    
    // Simple test of process switching
    for _ in 0..5 {
        process::switch_to_next_process();
        
        // Small delay to see the switching
        let start = drivers::timer::SYSTEM_TIMER.lock().ticks();
        while drivers::timer::SYSTEM_TIMER.lock().ticks() < start + 100 {
            x86_64::instructions::hlt();
        }
    }
    
    loop {
        // For now, just halt the CPU
        kewve_os::hlt_loop();
    }
}