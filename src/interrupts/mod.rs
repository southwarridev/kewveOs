pub mod pic;

use x86_64::structures::idt::{InterruptDescriptorTable, InterruptStackFrame, PageFaultErrorCode};
use crate::{println, serial_println};

/// Interrupt handling errors
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InterruptError {
    /// Invalid interrupt number
    InvalidInterrupt(u8),
    /// Handler registration failed
    HandlerRegistrationFailed,
    /// Interrupt controller not initialized
    ControllerNotInitialized,
}

impl core::fmt::Display for InterruptError {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self {
            InterruptError::InvalidInterrupt(irq) => write!(f, "Invalid interrupt number: {}", irq),
            InterruptError::HandlerRegistrationFailed => write!(f, "Failed to register interrupt handler"),
            InterruptError::ControllerNotInitialized => write!(f, "Interrupt controller not initialized"),
        }
    }
}

lazy_static::lazy_static! {
    static ref IDT: InterruptDescriptorTable = {
        let mut idt = InterruptDescriptorTable::new();
        idt.breakpoint.set_handler_fn(breakpoint_handler);
        idt.page_fault.set_handler_fn(page_fault_handler);
        idt.general_protection_fault.set_handler_fn(general_protection_fault_handler);
        idt.divide_error.set_handler_fn(divide_error_handler);
        idt[32].set_handler_fn(timer_interrupt_handler);
        idt[33].set_handler_fn(keyboard_interrupt_handler);
        idt
    };
}

pub fn init_idt() {
    IDT.load();
}

// Exception handlers
extern "x86-interrupt" fn breakpoint_handler(stack_frame: InterruptStackFrame) {
    println!("EXCEPTION: BREAKPOINT\n{:#?}", stack_frame);
    serial_println!("EXCEPTION: BREAKPOINT\n{:#?}", stack_frame);
}

extern "x86-interrupt" fn page_fault_handler(
    stack_frame: InterruptStackFrame,
    error_code: PageFaultErrorCode,
) {
    use x86_64::registers::control::Cr2;

    println!("EXCEPTION: PAGE FAULT");
    println!("Accessed Address: {:?}", Cr2::read());
    println!("Error Code: {:?}", error_code);
    println!("{:#?}", stack_frame);
    serial_println!("EXCEPTION: PAGE FAULT");
    loop {
        x86_64::instructions::hlt();
    }
}

extern "x86-interrupt" fn general_protection_fault_handler(
    stack_frame: InterruptStackFrame,
    _error_code: u64,
) {
    println!("EXCEPTION: GENERAL PROTECTION FAULT\n{:#?}", stack_frame);
    serial_println!("EXCEPTION: GENERAL PROTECTION FAULT\n{:#?}", stack_frame);
    loop {
        x86_64::instructions::hlt();
    }
}

extern "x86-interrupt" fn divide_error_handler(
    stack_frame: InterruptStackFrame,
) {
    println!("EXCEPTION: DIVIDE ERROR\n{:#?}", stack_frame);
    serial_println!("EXCEPTION: DIVIDE ERROR\n{:#?}", stack_frame);
    loop {
        x86_64::instructions::hlt();
    }
}

// Hardware interrupt handlers
extern "x86-interrupt" fn timer_interrupt_handler(_stack_frame: InterruptStackFrame) {
    // Handle timer interrupt using our driver
    crate::drivers::timer::handle_timer_interrupt();
}

extern "x86-interrupt" fn keyboard_interrupt_handler(_stack_frame: InterruptStackFrame) {
    // Handle keyboard interrupt using our driver
    crate::drivers::keyboard::handle_keyboard_interrupt();
}