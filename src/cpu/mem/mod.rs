const MEM_MAX: usize = 1 << 16;

pub fn new_mem() -> [u16; MEM_MAX] {
    return [0; MEM_MAX];
}

pub fn mem_read(pc: u16) -> u16 {
    pc
}





