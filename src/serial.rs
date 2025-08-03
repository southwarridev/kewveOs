use spin::Mutex;
use lazy_static::lazy_static;

#[cfg(target_arch = "x86_64")]
use uart_16550::SerialPort;

#[cfg(target_arch = "x86_64")]
lazy_static! {
    /// A global serial port instance protected by a spinlock.
    pub static ref SERIAL1: Mutex<SerialPort> = {
        let mut serial_port = unsafe { SerialPort::new(0x3F8) };
        serial_port.init();
        Mutex::new(serial_port)
    };
}

#[doc(hidden)]
pub fn _print(args: ::core::fmt::Arguments) {
    use core::fmt::Write;

    #[cfg(target_arch = "x86_64")]
    {
        SERIAL1.lock().write_fmt(args).expect("Printing to serial failed");
    }
    
    #[cfg(not(target_arch = "x86_64"))]
    {
        // For non-x86_64 platforms, we'll skip serial output for now
        // TODO: Implement platform-specific serial output
        let _ = args; // Suppress unused variable warning
    }
}

/// Prints to the host through the serial interface.
#[macro_export]
macro_rules! serial_print {
    ($($arg:tt)*) => {
        $crate::serial::_print(format_args!($($arg)*));
    };
}

/// Prints to the host through the serial interface, appending a newline.
#[macro_export]
macro_rules! serial_println {
    () => ($crate::serial_print!("\n"));
    ($fmt:expr) => ($crate::serial_print!(concat!($fmt, "\n")));
    ($fmt:expr, $($arg:tt)*) => ($crate::serial_print!(
        concat!($fmt, "\n"), $($arg)*));
}