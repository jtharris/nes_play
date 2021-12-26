use std::fmt::{Display, Formatter};
use crate::cpu::{Instruction, CPU};

// http://www.obelisk.me.uk/6502/reference.html#TXS
pub(super) struct TXS {}

impl Display for TXS {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "TXS")
    }
}

impl Instruction for TXS {
    fn execute(&self, cpu: &mut CPU) -> u8 {
        cpu.stack_pointer = cpu.index_register_x;

        2
    }

    fn bytes(&self) -> Vec<u8> {
        todo!()
    }
}

#[cfg(test)]
mod test {
    use crate::cpu::{CPU, Instruction};
    use super::TXS;

    #[test]
    fn copies_to_stack() {
        // Given
        let mut cpu = CPU::empty();
        cpu.index_register_x = 0x7E;

        // When
        TXS{}.execute(&mut cpu);

        // Then
        assert_eq!(0x7E, cpu.stack_pointer);
        assert_eq!(0x00, cpu.processor_status);
    }
}
