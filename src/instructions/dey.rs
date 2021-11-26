use std::fmt::{Display, Formatter};
use crate::cpu::{CPU, Instruction, StatusFlag};

// http://www.obelisk.me.uk/6502/reference.html#DEY
pub(super) struct DEY {}

impl Display for DEY {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "DEY")
    }
}

impl Instruction for DEY {
    fn execute(&self, cpu: &mut CPU) -> u8 {
        let (val, _) = cpu.index_register_y.overflowing_sub(1);
        cpu.index_register_y = val;

        cpu.set_flag(StatusFlag::Zero,  val == 0);
        cpu.set_flag(StatusFlag::Negative, val > 0x7F);

        2
    }
}

#[cfg(test)]
mod test {
    use crate::cpu::{CPU, Instruction};
    use crate::instructions::dey::DEY;

    #[test]
    fn basic_decrement() {
        // Given
        let mut cpu = CPU::empty();
        cpu.index_register_y = 0x0C;

        // When
        DEY{}.execute(&mut cpu);

        // Then
        assert_eq!(0x0B, cpu.index_register_y);
        assert_eq!(0, cpu.processor_status);    // Make sure no bits were set
    }

    #[test]
    fn zero_result() {
        // Given
        let mut cpu = CPU::empty();
        cpu.index_register_y = 0x01;

        // When
        DEY{}.execute(&mut cpu);

        // Then
        assert_eq!(0, cpu.index_register_y);
        assert_eq!(0x02, cpu.processor_status);    // Zero flag is set
    }

    #[test]
    fn negative_result() {
        // Given
        let mut cpu = CPU::empty();
        cpu.index_register_y = 0xFD;

        // When
        DEY{}.execute(&mut cpu);

        // Then
        assert_eq!(0xFC, cpu.index_register_y);
        assert_eq!(0x80, cpu.processor_status);    // Negative flag is set
    }

    #[test]
    fn negative_wrap() {
        // Given
        let mut cpu = CPU::empty();
        cpu.index_register_y = 0x00;

        // When
        DEY{}.execute(&mut cpu);

        // Then
        assert_eq!(0xFF, cpu.index_register_y);
        assert_eq!(0x80, cpu.processor_status);    // Negative flag is set
    }
}
