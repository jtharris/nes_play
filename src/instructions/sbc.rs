use crate::cpu::{AddressingMode, Instruction, CPU, StatusFlag};

// http://www.obelisk.me.uk/6502/reference.html#SBC
struct SBC {
    mode: AddressingMode
}

impl SBC {
    pub fn new(mode: AddressingMode) -> Self {
        SBC{ mode }
    }
}

impl Instruction for SBC {
    fn execute(&self, cpu: &mut CPU) {
        let memory_value= cpu.read(&self.mode) as u16;
        let existing_carry = (cpu.processor_status & 0x01) as u16;

        let diff = (cpu.accumulator as u16)
            .wrapping_sub(memory_value)
            .wrapping_sub(1 - existing_carry);

        // http://nesdev.com/6502_cpu.txt
        // http://www.6502.org/tutorials/vflag.html
        let carry = diff <= 0xFF;
        let overflow = diff > 0xFF7F || diff < 0xFE80;

        cpu.accumulator = diff as u8;
        cpu.set_flag(StatusFlag::Carry, carry);
        cpu.set_flag(StatusFlag::Zero, cpu.accumulator == 0);
        cpu.set_flag(StatusFlag::Overflow, overflow);
        cpu.set_flag(StatusFlag::Negative, cpu.accumulator > 0x7F);
    }
}

#[cfg(test)]
mod test {
    use crate::cpu::{CPU, Instruction, StatusFlag};
    use crate::cpu::AddressingMode::{ZeroPage, Immediate};
    use super::SBC;

    #[test]
    fn basic_sub_with_carry_set() {
        // Given
        let mut cpu = CPU::new();
        cpu.set_flag(StatusFlag::Carry, true);
        let mode = ZeroPage(0xA1);

        cpu.accumulator = 0x0A;
        cpu.write(&mode, 0x03);

        // When
        SBC::new(mode).execute(&mut cpu);

        // Then
        assert_eq!(0x07, cpu.accumulator);
        assert_eq!(0b01000001, cpu.processor_status);   // carry and overflow
    }

    #[test]
    fn basic_sub_without_existing_carry() {
        // Given
        let mut cpu = CPU::new();
        cpu.set_flag(StatusFlag::Carry, false);
        let mode = ZeroPage(0xA1);

        cpu.accumulator = 0x0A;
        cpu.write(&mode, 0x03);

        // When
        SBC::new(mode).execute(&mut cpu);

        // Then
        assert_eq!(0x06, cpu.accumulator);  // Subtract one more because carry was 0
        assert_eq!(0b01000001, cpu.processor_status);  // carry and overflow
    }

    #[test]
    fn sub_setting_overflow_and_negative() {
        // Given
        let mut cpu = CPU::new();
        let mode = ZeroPage(0xA1);

        cpu.accumulator = 0x03;
        cpu.write(&mode, 0x07);

        // When
        SBC::new(mode).execute(&mut cpu);

        // Then
        assert_eq!(0xFB, cpu.accumulator);
        assert_eq!(0b11000000, cpu.processor_status);  // negative and overflow
    }

    #[test]
    fn sub_with_overflow_zero() {
        // Given
        let mut cpu = CPU::new();
        cpu.set_flag(StatusFlag::Carry, true);

        cpu.accumulator = 0xFF;
        let mode = Immediate(0xFF);

        // When
        SBC::new(mode).execute(&mut cpu);

        // Then
        assert_eq!(0x00, cpu.accumulator);
        assert_eq!(0b01000011, cpu.processor_status);
    }

    #[test]
    fn sub_with_overflow_cleared() {
        // Given
        let mut cpu = CPU::new();
        cpu.set_flag(StatusFlag::Carry, true);

        cpu.accumulator = 0x00;
        let mode = Immediate(0xA0);

        // When
        SBC::new(mode).execute(&mut cpu);

        // Then
        assert_eq!(0x60, cpu.accumulator);
        assert_eq!(0b00000000, cpu.processor_status);  // no overflow, carry, or negative
    }

}