use crate::cpu::{AddressingMode, Instruction, CPU, StatusFlag};

// http://www.obelisk.me.uk/6502/reference.html#ADC
struct ADC {
    mode: AddressingMode
}

impl ADC {
    pub fn new(mode: AddressingMode) -> Self {
        ADC{ mode }
    }
}

impl Instruction for ADC {
    fn execute(&self, cpu: &mut CPU) {
        let memory_value= cpu.read(&self.mode);
        let existing_carry = cpu.processor_status & 0x01;
        let sum = memory_value as u16 + existing_carry as u16 + cpu.accumulator as u16;

        // http://nesdev.com/6502_cpu.txt
        // http://www.6502.org/tutorials/vflag.html
        let carry = sum > 0xFF;
        let overflow = sum >= 0x007F && sum <= 0x017F;

        cpu.accumulator = sum as u8;
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
    use crate::instructions::adc::ADC;

    #[test]
    fn basic_addition() {
        // Given
        let mut cpu = CPU::new();
        let mode = ZeroPage(0xA1);

        cpu.accumulator = 0x0A;
        cpu.write(&mode, 0x10);

        // When
        let adc = ADC::new(mode);
        adc.execute(&mut cpu);

        // Then
        assert_eq!(0x1A, cpu.accumulator);
        assert_eq!(0b00000000, cpu.processor_status);
    }

    #[test]
    fn addition_with_carry_and_overflow() {
        // Given
        let mut cpu = CPU::new();
        let mode = ZeroPage(0xA1);

        cpu.accumulator = 0xF3;         // -115, 243
        cpu.write(&mode, 0x11);   //  17,  17

        // When
        let adc = ADC::new(mode);
        adc.execute(&mut cpu);

        // Then
        // -115 + 17 = -98 so V is set, 243 + 17 = 260 so C is set
        assert_eq!(0x04, cpu.accumulator);
        assert_eq!(0b01000001, cpu.processor_status);
    }

    #[test]
    fn addition_using_carry() {
        // Given
        let mut cpu = CPU::new();
        let mode = ZeroPage(0xA1);

        cpu.accumulator = 0x03;
        cpu.set_flag(StatusFlag::Carry, true);
        cpu.write(&mode, 0x11);

        // When
        let adc = ADC::new(mode);
        adc.execute(&mut cpu);

        // Then
        assert_eq!(0x15, cpu.accumulator);
        assert_eq!(0b00000000, cpu.processor_status);
    }

    #[test]
    fn addition_with_overflow_and_negative() {
        // Given
        let mut cpu = CPU::new();

        cpu.accumulator = 0xFF;
        let mode = Immediate(0x89);

        // When
        let adc = ADC::new(mode);
        adc.execute(&mut cpu);

        // Then
        assert_eq!(0x88, cpu.accumulator);
        assert_eq!(0b10000001, cpu.processor_status);
    }

    #[test]
    fn addition_with_overflow_zero_and_carry() {
        // Given
        let mut cpu = CPU::new();

        cpu.accumulator = 0xFF;
        let mode = Immediate(0x01);

        // When
        let adc = ADC::new(mode);
        adc.execute(&mut cpu);

        // Then
        assert_eq!(0x00, cpu.accumulator);
        assert_eq!(0b01000011, cpu.processor_status);
    }

}