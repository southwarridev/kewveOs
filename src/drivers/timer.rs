//! Timer driver for Kewve OS

use super::{Driver, DriverError};
use x86_64::instructions::port::Port;
use spin::Mutex;
use lazy_static::lazy_static;

/// PIT (Programmable Interval Timer) driver
pub struct PitTimer {
    initialized: bool,
    command_port: Port<u8>,
    data_port: Port<u8>,
    frequency: u32,
}

/// System time tracking
#[derive(Debug, Clone, Copy)]
pub struct SystemTime {
    pub seconds: u64,
    pub milliseconds: u32,
}

impl PitTimer {
    /// Create a new PIT timer
    pub const fn new() -> Self {
        Self {
            initialized: false,
            command_port: Port::new(0x43),
            data_port: Port::new(0x40),
            frequency: 0,
        }
    }
    
    /// Configure the timer with a specific frequency
    pub fn configure(&mut self, frequency: u32) {
        self.frequency = frequency;
        
        // Calculate the divisor
        let divisor = 1193182 / frequency as u64;
        
        unsafe {
            // Send command byte
            self.command_port.write(0x36);
            
            // Send divisor
            self.data_port.write((divisor & 0xFF) as u8);
            self.data_port.write(((divisor >> 8) & 0xFF) as u8);
        }
    }
    
    /// Get the timer frequency
    pub fn frequency(&self) -> u32 {
        self.frequency
    }
}

impl Driver for PitTimer {
    fn name(&self) -> &str {
        "PIT Timer"
    }
    
    fn init(&mut self) -> Result<(), DriverError> {
        // Configure timer with 1000 Hz frequency (1ms intervals)
        self.configure(1000);
        
        self.initialized = true;
        Ok(())
    }
    
    fn deinit(&mut self) -> Result<(), DriverError> {
        // Reset timer
        unsafe {
            self.command_port.write(0x30);
            self.data_port.write(0);
            self.data_port.write(0);
        }
        
        self.initialized = false;
        Ok(())
    }
    
    fn is_initialized(&self) -> bool {
        self.initialized
    }
}

/// Global system timer instance
lazy_static! {
    pub static ref SYSTEM_TIMER: Mutex<SystemTimer> = Mutex::new(SystemTimer::new());
}

/// System timer for tracking time
pub struct SystemTimer {
    ticks: u64,
    timer: PitTimer,
}

impl SystemTimer {
    /// Create a new system timer
    pub const fn new() -> Self {
        Self {
            ticks: 0,
            timer: PitTimer::new(),
        }
    }
    
    /// Initialize the system timer
    pub fn init(&mut self) -> Result<(), DriverError> {
        self.timer.init()?;
        self.ticks = 0;
        Ok(())
    }
    
    /// Handle a timer tick
    pub fn handle_tick(&mut self) {
        self.ticks += 1;
    }
    
    /// Get the number of ticks since boot
    pub fn ticks(&self) -> u64 {
        self.ticks
    }
    
    /// Get the current system time
    pub fn time(&self) -> SystemTime {
        let total_ms = self.ticks;
        SystemTime {
            seconds: total_ms / 1000,
            milliseconds: (total_ms % 1000) as u32,
        }
    }
    
    /// Sleep for a specified number of milliseconds
    pub fn sleep(&mut self, ms: u64) {
        let start = self.ticks;
        let end = start + ms;
        
        while self.ticks < end {
            x86_64::instructions::hlt();
        }
    }
}

/// Handle timer interrupt
pub fn handle_timer_interrupt() {
    // Increment system tick counter
    SYSTEM_TIMER.lock().handle_tick();
    
    // Send EOI to PIC
    unsafe {
        crate::interrupts::pic::PICS.lock().notify_end_of_interrupt(32);
    }
}