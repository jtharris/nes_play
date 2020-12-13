use crate::cpu::{Instruction, CPU, StatusFlag, AddressingMode};

// http://www.obelisk.me.uk/6502/reference.html#BIT
struct BIT {
    mode: AddressingMode
}

impl BIT {
    pub fn new(mode: AddressingMode) -> Self {
        BIT{ mode }
    }
}

impl Instruction for BIT {
    fn execute(&self, cpu: &mut CPU) {
        let val = cpu.accumulator & cpu.read(&self.mode);

        cpu.set_flag(StatusFlag::Zero, val == 0);
        cpu.set_flag(StatusFlag::Overflow, val & 0x40 == 0x40);  // if 6th bit is set
        cpu.set_flag(StatusFlag::Negative, val > 0x7F);          // if 7th bit is set
    }
}

#[cfg(test)]
mod test {
    use crate::cpu::{CPU, Instruction};
    use super::BIT;
    use crate::cpu::AddressingMode::ZeroPage;

    #[test]
    fn clear_top_bits() {
        // Given
        let mut cpu = CPU::new();
        let mode = ZeroPage(0xA8);
        cpu.write(&mode, 0xF0);
        cpu.accumulator = 0x0F;
        cpu.processor_status = 0b11000000;       // overflow and negative set

        // When
        BIT::new(mode).execute(&mut cpu);

        // Then
        assert_eq!(0x02, cpu.processor_status);  // Only zero flag should be set
    }

    #[test]
    fn set_top_bits() {
        // Given
        let mut cpu = CPU::new();
        let mode = ZeroPage(0xA8);
        cpu.write(&mode, 0xF0);
        cpu.accumulator = 0xEF;
        cpu.processor_status = 0b00000010;       // only zero flag set

        // When
        BIT::new(mode).execute(&mut cpu);

        // Then
        assert_eq!(0xC0, cpu.processor_status);  // overflow and negative set and zero cleared
    }
}
