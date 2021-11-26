use std::fmt::{Display, Formatter};
use crate::cpu::{CPU, Instruction, StatusFlag};

// http://www.obelisk.me.uk/6502/reference.html#DEX
pub(super) struct DEX {}

impl Display for DEX {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "DEX")
    }
}

impl Instruction for DEX {
    fn execute(&self, cpu: &mut CPU) -> u8 {
        let (val, _) = cpu.index_register_x.overflowing_sub(1);
        cpu.index_register_x = val;

        cpu.set_flag(StatusFlag::Zero,  val == 0);
        cpu.set_flag(StatusFlag::Negative, val > 0x7F);

        2
    }
}

#[cfg(test)]
mod test {
    use crate::cpu::{CPU, Instruction};
    use crate::instructions::dex::DEX;

    #[test]
    fn basic_decrement() {
        // Given
        let mut cpu = CPU::empty();
        cpu.index_register_x = 0x0C;

        // When
        DEX{}.execute(&mut cpu);

        // Then
        assert_eq!(0x0B, cpu.index_register_x);
        assert_eq!(0, cpu.processor_status);    // Make sure no bits were set
    }

    #[test]
    fn zero_result() {
        // Given
        let mut cpu = CPU::empty();
        cpu.index_register_x = 0x01;

        // When
        DEX{}.execute(&mut cpu);

        // Then
        assert_eq!(0, cpu.index_register_x);
        assert_eq!(0x02, cpu.processor_status);    // Zero flag is set
    }

    #[test]
    fn negative_result() {
        // Given
        let mut cpu = CPU::empty();
        cpu.index_register_x = 0xFD;

        // When
        DEX{}.execute(&mut cpu);

        // Then
        assert_eq!(0xFC, cpu.index_register_x);
        assert_eq!(0x80, cpu.processor_status);    // Negative flag is set
    }

    #[test]
    fn negative_wrap() {
        // Given
        let mut cpu = CPU::empty();
        cpu.index_register_x = 0x00;

        // When
        DEX{}.execute(&mut cpu);

        // Then
        assert_eq!(0xFF, cpu.index_register_x);
        assert_eq!(0x80, cpu.processor_status);    // Negative flag is set
    }
}
