pub mod cflags;

pub enum Register {
    // RR0 - RR7 -> General Purpose registers
    // RR8 -> Program Counter register
    // RR9 -> Flag register
    RR0,
    RR1,
    RR2,
    RR3,
    RR4,
    RR5,
    RR6,
    RR7,
    RR8,
    RR9,
}

impl Register {
    pub const fn as_usize(&self) -> usize {
        match self {
            Register::RR0 => 0,
            Register::RR1 => 1,
            Register::RR2 => 2,
            Register::RR3 => 3,
            Register::RR4 => 4,
            Register::RR5 => 5,
            Register::RR6 => 6,
            Register::RR7 => 7,
            Register::RR8 => 8,
            Register::RR9 => 9,
        }
    }
    pub const fn num_regs() -> usize {
        return (Register::RR9.as_usize()) + 1;
    }

}

pub fn new_reg_set() -> [u16; Register::num_regs()] {
    return [0u16; Register::num_regs()];
}
