pub mod cpu;
use crate::cpu::mem;
use crate::cpu::regs;
use crate::cpu::regs::cflags;

fn main() {
    let mut regs_set = regs::new_reg_set();
    regs_set[regs::Register::RR9.as_usize()] = cflags::Flags::ZRO.as_u16();

    const PC_INIT: u16 = 0x3000;
    regs_set[regs::Register::RR8.as_usize()] = PC_INIT;
    let running: bool = true;
    while running {
        let instr = mem::mem_read(regs_set[regs::Register::RR8.as_usize()] + 1);
        cpu::exec_instr(instr);
    }
}
