pub mod br;
pub mod add;

#[repr(u16)]
// Opcodes for LC-3
// The extra opcode INVALID has no operation asociated
pub enum Opcode {
    BR = 0x0,
    ADD = 0x1,
    LD = 0x2,
    ST = 0x3,
    JSR = 0x4,
    AND = 0x5,
    LDR = 0x6,
    STR = 0x7,
    RTI = 0x8,
    NOT = 0x9,
    LDI = 0xA,
    STI = 0xB,
    JMP = 0xC,
    RES = 0xD,
    LEA = 0xE,
    TRAP = 0xF,
    INVALID = u16::MAX,
}

impl Opcode {
    // Convert enum to a nibble
    pub const fn as_nibble(self) -> u16 {
        self as u16
    }

    // Convert from a nibble to the corresponding Opcode
    pub fn from_nibble(op: u16) -> Opcode {
        match op {
            0x0 => Opcode::BR,
            0x1 => Opcode::ADD,
            0x2 => Opcode::LD,
            0x3 => Opcode::ST,
            0x4 => Opcode::JSR,
            0x5 => Opcode::AND,
            0x6 => Opcode::LDR,
            0x7 => Opcode::STR,
            0x8 => Opcode::RTI,
            0x9 => Opcode::NOT,
            0xA => Opcode::LDI,
            0xB => Opcode::STI,
            0xC => Opcode::JMP,
            0xD => Opcode::RES,
            0xE => Opcode::LEA,
            0xF => Opcode::TRAP,
            _ => Opcode::INVALID, 
        }
    }
}

// Implement From<u16> for converting a u16 to an Opcode
impl From<u16> for Opcode {
    fn from(value: u16) -> Self {
        Opcode::from_nibble(value)
    }
}

// Implement Into<u16> for converting an Opcode to a u16
impl Into<u16> for Opcode {
    fn into(self) -> u16 {
        self.as_nibble()
    }
}

