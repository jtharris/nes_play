use std::fmt::{Display, Formatter};
use crate::cpu::{Instruction, CPU};

// http://www.obelisk.me.uk/6502/reference.html#NOP
pub struct NOP {}

impl Display for NOP {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "NOP")
    }
}

impl Instruction for NOP {
    fn execute(&self, _cpu: &mut CPU) -> u8 { 2 }

    fn bytes(&self) -> Vec<u8> {
        todo!()
    }
}