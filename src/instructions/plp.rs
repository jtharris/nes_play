use std::fmt::{Display, Formatter};
use crate::cpu::{Instruction, CPU};

// http://www.obelisk.me.uk/6502/reference.html#PLP
pub(super) struct PLP {}

impl Display for PLP {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "PLP")
    }
}

impl Instruction for PLP {
    fn execute(&self, cpu: &mut CPU) -> u8 {
        let existing_bits = cpu.processor_status & 0b0011_0000;
        let stack_value = cpu.pop_stack() & 0b1100_1111;

        cpu.processor_status = stack_value | existing_bits;

        4
    }

    fn bytes(&self) -> Vec<u8> {
        vec![0x28]
    }
}


#[cfg(test)]
mod test {
    use crate::cpu::{CPU, Instruction};
    use crate::instructions::plp::PLP;

    #[test]
    fn s_is_pulled() {
        // Given
        let mut cpu = CPU::empty();
        cpu.push_stack(0xAF);

        // When
        PLP{}.execute(&mut cpu);

        // Then
        assert_eq!(0x8F, cpu.processor_status);
    }

    #[test]
    fn s_is_pulled_ignoring_bits() {
        // Given
        let mut cpu = CPU::empty();
        cpu.processor_status = 0b1111_1111;
        cpu.push_stack(0x00);

        // When
        PLP{}.execute(&mut cpu);

        // Then
        assert_eq!(0b0011_0000, cpu.processor_status);
    }
}