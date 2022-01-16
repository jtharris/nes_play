use std::fmt::{Display, Formatter};
use crate::cpu::{Instruction, CPU};

// http://www.obelisk.me.uk/6502/reference.html#RTS
pub(super) struct RTS {}

impl Display for RTS {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "RTS")
    }
}

impl Instruction for RTS {
    fn execute(&self, cpu: &mut CPU) -> u8 {
        let low_pc = cpu.pop_stack();
        let high_pc = cpu.pop_stack();
        cpu.program_counter = u16::from_be_bytes([high_pc, low_pc]) + 1;

        6
    }

    fn bytes(&self) -> Vec<u8> {
        vec![0x60]
    }
}

#[cfg(test)]
mod test {
    use crate::cpu::{CPU, Instruction};
    use super::RTS;

    #[test]
    fn values_all_pulled() {
        // Given
        let mut cpu = CPU::empty();
        cpu.push_stack(0xA1);
        cpu.push_stack(0xC3);

        // When
        RTS{}.execute(&mut cpu);

        // Then
        assert_eq!(0xA1C4, cpu.program_counter);
    }
}