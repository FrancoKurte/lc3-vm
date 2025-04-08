pub mod cflags;

// Register set
// Registers from RR0 to RR7 are general purpose with a size of 3b each.
// RR8/9 should not be used to perform operations but rather be used for flow control.
pub enum Register {
    // RR0 - RR7 -> General Purpose registers
    // RR8 -> Program Counter register
    // RR9 -> Flag register
    RR0 = 0b000,
    RR1 = 0b001,
    RR2 = 0b010,
    RR3 = 0b011,
    RR4 = 0b100,
    RR5 = 0b101,
    RR6 = 0b110,
    RR7 = 0b111,
    RR8 = 0b1000,
    RR9 = 0b1001,
}

impl Register {
    pub const fn num_regs() -> usize {
        return Register::RR9.into() + (1 as usize);
    }
}

impl From<u16> for Register {
    fn from(n: u16) -> u16 {
        match n {

        }
    }
}

impl Into<usize> for Register {
    fn into(self) -> usize {
        self as usize
    }
}

pub const fn new_reg_set() -> [u16; Register::num_regs()] {
    return [0u16; Register::num_regs()];
}
