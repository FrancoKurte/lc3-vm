pub mod ins;
pub mod mem;
pub mod regs;

use crate::cpu::ins::Opcode;

fn unpack_ins(ins: u16) -> (Opcode, u16) {
    let op = Opcode::from_nibble(ins >> 12);
    return (op, ins & 0xFFF);
}

pub fn exec_ins(ins: u16) { 
     let (_op, _arg): (Opcode, u16) = unpack_ins(ins);
}

