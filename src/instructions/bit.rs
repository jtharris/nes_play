use std::fmt::{Display, Formatter};
use crate::cpu::{Instruction, CPU, StatusFlag, AddressingMode};

// http://www.obelisk.me.uk/6502/reference.html#BIT
pub(super) struct BIT {
    mode: AddressingMode
}

impl BIT {
    pub fn new(mode: AddressingMode) -> Self {
        BIT{ mode }
    }
}

impl Display for BIT {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "BIT {}", self.mode)
    }
}

impl Instruction for BIT {
    fn execute(&self, cpu: &mut CPU) -> u8 {
        let operand = cpu.read(&self.mode);

        cpu.set_flag(StatusFlag::Zero, cpu.accumulator & operand == 0);
        cpu.set_flag(StatusFlag::Overflow, operand & 0x40 == 0x40);  // if 6th bit is set
        cpu.set_flag(StatusFlag::Negative, operand > 0x7F);          // if 7th bit is set

        cpu.default_cycles(&self.mode)
    }
}

#[cfg(test)]
mod test {
    use crate::cpu::{CPU, Instruction, AddressingMode::ZeroPage};
    use super::BIT;

    #[test]
    fn clear_top_bits() {
        // Given
        let mut cpu = CPU::empty();
        let mode = ZeroPage(0xA8);
        cpu.write(&mode, 0x0F);
        cpu.accumulator = 0xF0;
        cpu.processor_status = 0b11000000;       // overflow and negative set

        // When
        BIT::new(mode).execute(&mut cpu);

        // Then
        assert_eq!(0x02, cpu.processor_status);  // Only zero flag should be set
    }

    #[test]
    fn set_top_bits() {
        // Given
        let mut cpu = CPU::empty();
        let mode = ZeroPage(0xA8);
        cpu.write(&mode, 0xF0);
        cpu.accumulator = 0xEF;
        cpu.processor_status = 0b00000010;       // only zero flag set

        // When
        BIT::new(mode).execute(&mut cpu);

        // Then
        assert_eq!(0xC0, cpu.processor_status);  // overflow and negative set and zero cleared
    }

    #[test]
    fn string_representation() {
        // Given
        let bit = BIT::new(ZeroPage(0x0B));

        // Then
        assert_eq!("BIT $0B", bit.to_string())
    }
}
