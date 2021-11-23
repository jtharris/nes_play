use std::fmt::{Display, Formatter};
use crate::cpu::{Instruction, CPU, StatusFlag};

// http://www.obelisk.me.uk/6502/reference.html#TAY
pub(super) struct TAY {}

impl Display for TAY {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl Instruction for TAY {
    fn execute(&self, cpu: &mut CPU) -> u8 {
        cpu.index_register_y = cpu.accumulator;

        cpu.set_flag(StatusFlag::Zero, cpu.index_register_y == 0);
        cpu.set_flag(StatusFlag::Negative, cpu.index_register_y > 0x7F);

        2
    }
}

#[cfg(test)]
mod test {
    use crate::cpu::{CPU, Instruction};
    use super::TAY;

    #[test]
    fn copies_to_y_no_flags() {
        // Given
        let mut cpu = CPU::empty();
        cpu.accumulator = 0x7E;

        // When
        TAY{}.execute(&mut cpu);

        // Then
        assert_eq!(0x7E, cpu.index_register_y);
        assert_eq!(0x00, cpu.processor_status);
    }

    #[test]
    fn copies_to_y_zero_flag() {
        // Given
        let mut cpu = CPU::empty();
        cpu.accumulator = 0x00;
        cpu.index_register_y = 0xAF;

        // When
        TAY{}.execute(&mut cpu);

        // Then
        assert_eq!(0x00, cpu.index_register_y);
        assert_eq!(0x02, cpu.processor_status);  // zero flag
    }

    #[test]
    fn copies_to_y_negative_flag() {
        // Given
        let mut cpu = CPU::empty();
        cpu.accumulator = 0xFC;
        cpu.index_register_y = 0xAF;

        // When
        TAY{}.execute(&mut cpu);

        // Then
        assert_eq!(0xFC, cpu.index_register_y);
        assert_eq!(0x80, cpu.processor_status);  // negative flag
    }
}
