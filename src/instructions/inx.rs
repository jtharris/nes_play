use std::fmt::{Display, Formatter};
use crate::cpu::{CPU, Instruction, StatusFlag};

// http://www.obelisk.me.uk/6502/reference.html#INX
pub(super) struct INX {}

impl Display for INX {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "INX")
    }
}

impl Instruction for INX {
    fn execute(&self, cpu: &mut CPU) -> u8 {
        let (val, _) = cpu.index_register_x.overflowing_add(1);
        cpu.index_register_x = val;

        cpu.set_flag(StatusFlag::Zero,  val == 0);
        cpu.set_flag(StatusFlag::Negative, val > 0x7F);

        2
    }

    fn bytes(&self) -> Vec<u8> {
        todo!()
    }
}

#[cfg(test)]
mod test {
    use crate::cpu::{CPU, Instruction};
    use crate::instructions::inx::INX;

    #[test]
    fn basic_increment() {
        // Given
        let mut cpu = CPU::empty();
        cpu.index_register_x = 0x0C;

        // When
        INX{}.execute(&mut cpu);

        // Then
        assert_eq!(0x0D, cpu.index_register_x);
        assert_eq!(0, cpu.processor_status);    // Make sure no bits were set
    }

    #[test]
    fn negative_result() {
        // Given
        let mut cpu = CPU::empty();
        cpu.index_register_x = 0xFD;

        // When
        INX{}.execute(&mut cpu);

        // Then
        assert_eq!(0xFE, cpu.index_register_x);
        assert_eq!(0x80, cpu.processor_status);    // Negative flag is set
    }

    #[test]
    fn zero_wrap() {
        // Given
        let mut cpu = CPU::empty();
        cpu.index_register_x = 0xFF;

        // When
        INX{}.execute(&mut cpu);

        // Then
        assert_eq!(0x00, cpu.index_register_x);
        assert_eq!(0x02, cpu.processor_status);    // Zero flag is set
    }
}
