use std::fmt::{Display, Formatter};
use crate::cpu::{Instruction, CPU, StatusFlag};

// http://www.obelisk.me.uk/6502/reference.html#TSX
pub(super) struct TSX {}

impl Display for TSX {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "TSX")
    }
}

impl Instruction for TSX {
    fn execute(&self, cpu: &mut CPU) -> u8 {
        cpu.index_register_x = cpu.stack_pointer;

        cpu.set_flag(StatusFlag::Zero, cpu.index_register_x == 0);
        cpu.set_flag(StatusFlag::Negative, cpu.index_register_x > 0x7F);

        2
    }
}

#[cfg(test)]
mod test {
    use crate::cpu::{CPU, Instruction};
    use super::TSX;

    #[test]
    fn copies_to_x_no_flags() {
        // Given
        let mut cpu = CPU::empty();
        cpu.stack_pointer = 0x7E;

        // When
        TSX{}.execute(&mut cpu);

        // Then
        assert_eq!(0x7E, cpu.index_register_x);
        assert_eq!(0x00, cpu.processor_status);
    }

    #[test]
    fn copies_to_x_zero_flag() {
        // Given
        let mut cpu = CPU::empty();
        cpu.stack_pointer = 0x00;
        cpu.index_register_x = 0xAF;

        // When
        TSX{}.execute(&mut cpu);

        // Then
        assert_eq!(0x00, cpu.index_register_x);
        assert_eq!(0x02, cpu.processor_status);  // zero flag
    }

    #[test]
    fn copies_to_x_negative_flag() {
        // Given
        let mut cpu = CPU::empty();
        cpu.stack_pointer = 0xFC;
        cpu.index_register_x = 0xAF;

        // When
        TSX{}.execute(&mut cpu);

        // Then
        assert_eq!(0xFC, cpu.index_register_x);
        assert_eq!(0x80, cpu.processor_status);  // negative flag
    }
}
