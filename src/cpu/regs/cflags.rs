pub enum Flags {
    POS,
    ZRO,
    NEG
}

impl Flags {
    pub const fn as_binary(&self) -> u16 {
        match self {
            Flags::POS => 1 << 0,
            Flags::ZRO => 1 << 1,
            Flags::NEG => 1 << 2,
        }
    }

    pub fn as_u16(&self) -> u16 {
        match self {
            Flags::POS => Flags::POS.as_binary() as u16,
            Flags::ZRO => Flags::ZRO.as_binary() as u16,
            Flags::NEG => Flags::NEG.as_binary() as u16,
        }
    }
}
