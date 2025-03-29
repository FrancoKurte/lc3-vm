pub enum Flags {
    POS = 1 << 0,
    ZRO = 1 << 1,
    NEG = 1 << 2
}

impl Flags {
    pub fn as_u16(&self) -> u16 {
        match self {
            Flags::POS => Flags::POS as u16,
            Flags::ZRO => Flags::ZRO as u16,
            Flags::NEG => Flags::NEG as u16,
        }
    }
}
