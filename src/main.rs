pub mod cli;
pub mod cpu;
pub mod img;
use crate::cpu::mem;
use crate::cpu::regs;
use crate::cpu::regs::{Register, cflags::Flags};

fn main() {
    /*
     * CLI should be the entry point, it should accept commands or paths to image files
     * CLI could also be used as a image management and verifier tool
     */

    /*
     * CPU should be initialized if an image is received or an new one is started
     */

    /*
     * IMG should be only used in the following cases:
     * Creating, reading, writing, closing or verifying an image.
     * IMG can be called standalone in main for image management or by CLI but not by the CPU.
     */


    let mut regs_set = regs::new_reg_set();
    regs_set[Register::RR9.as_usize()] = Flags::ZRO.as_u16();

    const PC_INIT: u16 = 0x3000;
    regs_set[Register::RR8.as_usize()] = PC_INIT;
    let running: bool = true;
    while running {
        let instr: u16 = mem::mem_read(regs_set[regs::Register::RR8.as_usize()] + 1);
        cpu::exec_ins(instr);
    }
}
