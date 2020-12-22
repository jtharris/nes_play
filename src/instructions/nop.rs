use crate::cpu::{Instruction, CPU};

// http://www.obelisk.me.uk/6502/reference.html#NOP
pub struct NOP {}

impl Instruction for NOP {
    fn execute(&self, _cpu: &mut CPU) {
        // no-op!
    }
}