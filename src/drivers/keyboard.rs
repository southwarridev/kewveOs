//! Keyboard driver for Kewve OS

use super::{Driver, DriverError, KeyEvent};
use x86_64::instructions::port::Port;
use spin::Mutex;
use lazy_static::lazy_static;

/// PS/2 Keyboard driver
pub struct Ps2Keyboard {
    initialized: bool,
    data_port: Port<u8>,
    command_port: Port<u8>,
}

impl Ps2Keyboard {
    /// Create a new PS/2 keyboard driver
    pub const fn new() -> Self {
        Self {
            initialized: false,
            data_port: Port::new(0x60),
            command_port: Port::new(0x64),
        }
    }
    
    /// Read a scancode from the keyboard
    pub fn read_scancode(&mut self) -> u8 {
        unsafe { self.data_port.read() }
    }
    
    /// Send a command to the keyboard
    pub fn send_command(&mut self, command: u8) {
        unsafe {
            self.command_port.write(command);
        }
    }
    
    /// Translate scancode to ASCII (simplified)
    pub fn scancode_to_ascii(&self, scancode: u8) -> Option<char> {
        // Simplified scancode translation for US keyboard layout
        let ascii_map = [
            None,           // 0x00
            Some(' '),      // 0x01 - Escape
            Some('1'),      // 0x02
            Some('2'),      // 0x03
            Some('3'),      // 0x04
            Some('4'),      // 0x05
            Some('5'),      // 0x06
            Some('6'),      // 0x07
            Some('7'),      // 0x08
            Some('8'),      // 0x09
            Some('9'),      // 0x0A
            Some('0'),      // 0x0B
            Some('-'),      // 0x0C
            Some('='),      // 0x0D
            None,           // 0x0E - Backspace
            None,           // 0x0F - Tab
            Some('q'),      // 0x10
            Some('w'),      // 0x11
            Some('e'),      // 0x12
            Some('r'),      // 0x13
            Some('t'),      // 0x14
            Some('y'),      // 0x15
            Some('u'),      // 0x16
            Some('i'),      // 0x17
            Some('o'),      // 0x18
            Some('p'),      // 0x19
            Some('['),      // 0x1A
            Some(']'),      // 0x1B
            None,           // 0x1C - Enter
            None,           // 0x1D - Left Control
            Some('a'),      // 0x1E
            Some('s'),      // 0x1F
            Some('d'),      // 0x20
            Some('f'),      // 0x21
            Some('g'),      // 0x22
            Some('h'),      // 0x23
            Some('j'),      // 0x24
            Some('k'),      // 0x25
            Some('l'),      // 0x26
            Some(';'),      // 0x27
            Some('\''),     // 0x28
            Some('`'),      // 0x29
            None,           // 0x2A - Left Shift
            Some('\\'),     // 0x2B
            Some('z'),      // 0x2C
            Some('x'),      // 0x2D
            Some('c'),      // 0x2E
            Some('v'),      // 0x2F
            Some('b'),      // 0x30
            Some('n'),      // 0x31
            Some('m'),      // 0x32
            Some(','),      // 0x33
            Some('.'),      // 0x34
            Some('/'),      // 0x35
            None,           // 0x36 - Right Shift
            None,           // 0x37 - Keypad *
            None,           // 0x38 - Left Alt
            Some(' '),      // 0x39 - Space
        ];
        
        if scancode as usize >= ascii_map.len() {
            None
        } else {
            ascii_map[scancode as usize]
        }
    }
}

impl Driver for Ps2Keyboard {
    fn name(&self) -> &str {
        "PS/2 Keyboard"
    }
    
    fn init(&mut self) -> Result<(), DriverError> {
        // Reset and enable the keyboard
        self.send_command(0xF4); // Enable scanning command
        
        self.initialized = true;
        Ok(())
    }
    
    fn deinit(&mut self) -> Result<(), DriverError> {
        // Disable the keyboard
        self.send_command(0xF5); // Disable scanning command
        
        self.initialized = false;
        Ok(())
    }
    
    fn is_initialized(&self) -> bool {
        self.initialized
    }
}

lazy_static! {
    /// Global keyboard instance
    pub static ref KEYBOARD: Mutex<Ps2Keyboard> = Mutex::new(Ps2Keyboard::new());
}

/// Process a keyboard interrupt
pub fn handle_keyboard_interrupt() {
    let scancode = KEYBOARD.lock().read_scancode();
    
    // Determine if key was pressed or released
    // For simplicity, we'll assume it's pressed if the scancode < 0x80
    let pressed = scancode < 0x80;
    let actual_scancode = if pressed { scancode } else { scancode - 0x80 };
    
    let _event = KeyEvent {
        scancode: actual_scancode,
        pressed,
    };
    
    // Translate scancode to ASCII if possible
    if let Some(ascii_char) = KEYBOARD.lock().scancode_to_ascii(actual_scancode) {
        if pressed {
            crate::println!("Key pressed: '{}'", ascii_char);
        }
    }
    
    // Send EOI to PIC
    unsafe {
        crate::interrupts::pic::PICS.lock().notify_end_of_interrupt(33);
    }
}