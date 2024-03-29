use std::fmt::{Display, Formatter};
use crate::cpu::{Instruction, CPU, StatusFlag};

// http://www.obelisk.me.uk/6502/reference.html#TXA
pub(super) struct TXA {}

impl Display for TXA {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "TXA")
    }
}

impl Instruction for TXA {
    fn execute(&self, cpu: &mut CPU) -> u8 {
        cpu.accumulator = cpu.index_register_x;

        cpu.set_flag(StatusFlag::Zero, cpu.accumulator == 0);
        cpu.set_flag(StatusFlag::Negative, cpu.accumulator > 0x7F);

        2
    }

    fn bytes(&self) -> Vec<u8> {
        vec![0x8A]
    }
}

#[cfg(test)]
mod test {
    use crate::cpu::{CPU, Instruction};
    use super::TXA;

    #[test]
    fn copies_to_acc_no_flags() {
        // Given
        let mut cpu = CPU::empty();
        cpu.index_register_x = 0x7E;

        // When
        TXA{}.execute(&mut cpu);

        // Then
        assert_eq!(0x7E, cpu.accumulator);
        assert_eq!(0x00, cpu.processor_status);
    }

    #[test]
    fn copies_to_acc_zero_flag() {
        // Given
        let mut cpu = CPU::empty();
        cpu.index_register_x = 0x00;
        cpu.accumulator = 0xAF;

        // When
        TXA{}.execute(&mut cpu);

        // Then
        assert_eq!(0x00, cpu.accumulator);
        assert_eq!(0x02, cpu.processor_status);  // zero flag
    }

    #[test]
    fn copies_to_acc_negative_flag() {
        // Given
        let mut cpu = CPU::empty();
        cpu.index_register_x = 0xFC;
        cpu.accumulator = 0xAF;

        // When
        TXA{}.execute(&mut cpu);

        // Then
        assert_eq!(0xFC, cpu.accumulator);
        assert_eq!(0x80, cpu.processor_status);  // negative flag
    }
}
