use spin::Mutex;
use x86_64::instructions::port::Port;
use core::iter::Iterator;

/// The offset for the first PIC (master)
const PIC_1_OFFSET: u8 = 32;
/// The offset for the second PIC (slave)
const PIC_2_OFFSET: u8 = PIC_1_OFFSET + 8;

/// Command sent to begin PIC initialization.
const CMD_INIT: u8 = 0x11;
/// Command sent to acknowledge an interrupt.
const CMD_END_OF_INTERRUPT: u8 = 0x20;
/// Command sent to read the interrupt request register
const CMD_READ_IRR: u8 = 0x0A;
/// Command sent to read the interrupt service register
const CMD_READ_ISR: u8 = 0x0B;

/// The mode in which we want to run our PICs.
const MODE_8086: u8 = 0x01;

/// A single PIC chip.
#[derive(Debug)]
struct Pic {
    /// The base offset for interrupts on this PIC.
    offset: u8,
    /// The command port for this PIC.
    command: Port<u8>,
    /// The data port for this PIC.
    data: Port<u8>,
}

impl Pic {
    /// Are we in change of handling the specified interrupt?
    fn handles_interrupt(&self, interupt_id: u8) -> bool {
        self.offset <= interupt_id && interupt_id < self.offset + 8
    }

    /// Notify us that an interrupt has been handled and that we're ready for more.
    unsafe fn end_of_interrupt(&mut self) {
        self.command.write(CMD_END_OF_INTERRUPT);
    }
}

/// A pair of PICs (primary and secondary).
#[derive(Debug)]
pub struct ChainedPics {
    pics: [Pic; 2],
}

impl ChainedPics {
    pub const unsafe fn new(offset1: u8, offset2: u8) -> ChainedPics {
        ChainedPics {
            pics: [
                Pic {
                    offset: offset1,
                    command: Port::new(0x20),
                    data: Port::new(0x21),
                },
                Pic {
                    offset: offset2,
                    command: Port::new(0xA0),
                    data: Port::new(0xA1),
                },
            ]
        }
    }

    /// Initialize both our PICs.
    pub unsafe fn initialize(&mut self) {
        // Save current masks
        let saved_mask1 = self.pics[0].data.read();
        let saved_mask2 = self.pics[1].data.read();

        // Tell each PIC that we're going to send it a three-byte initialization sequence on its data port.
        self.pics[0].command.write(CMD_INIT);
        self.pics[1].command.write(CMD_INIT);

        // Byte 1: Set up our base offsets
        self.pics[0].data.write(self.pics[0].offset);
        self.pics[1].data.write(self.pics[1].offset);

        // Byte 2: Configure chaining between PIC1 and PIC2
        self.pics[0].data.write(4); // PIC1 is master, PIC2 is slave
        self.pics[1].data.write(2); // PIC2 is connected to IRQ2 of PIC1

        // Byte 3: Set mode
        self.pics[0].data.write(MODE_8086);
        self.pics[1].data.write(MODE_8086);

        // Restore saved masks
        self.pics[0].data.write(saved_mask1);
        self.pics[1].data.write(saved_mask2);
    }

    /// Returns the interrupt mask for both PICs
    pub unsafe fn read_masks(&mut self) -> (u8, u8) {
        (self.pics[0].data.read(), self.pics[1].data.read())
    }

    /// Writes the interrupt masks for both PICs
    pub unsafe fn write_masks(&mut self, mask1: u8, mask2: u8) {
        self.pics[0].data.write(mask1);
        self.pics[1].data.write(mask2);
    }

    /// Disable both PICs by masking all interrupts
    pub unsafe fn disable(&mut self) {
        self.pics[0].data.write(0xFF);
        self.pics[1].data.write(0xFF);
    }

    /// Do we handle this interrupt?
    pub fn handles_interrupt(&self, interrupt_id: u8) -> bool {
        self.pics.iter().any(|p| p.handles_interrupt(interrupt_id))
    }

    /// Figure out which (if any) PICs in our chain need to know about this interrupt.
    ///
    /// This is tricky, because all interrupts from `pics[1]` are also sent to `pics[0]`.
    pub unsafe fn notify_end_of_interrupt(&mut self, interrupt_id: u8) {
        if self.handles_interrupt(interrupt_id) {
            if self.pics[1].handles_interrupt(interrupt_id) {
                self.pics[1].end_of_interrupt();
            }
            self.pics[0].end_of_interrupt();
        }
    }
}

pub static PICS: Mutex<ChainedPics> = 
    Mutex::new(unsafe { ChainedPics::new(PIC_1_OFFSET, PIC_2_OFFSET) });