use std::fmt::{Display, Formatter};
use crate::cpu::{AddressingMode, Instruction, CPU, StatusFlag};

// http://www.obelisk.me.uk/6502/reference.html#ADC
pub(super) struct ADC {
    mode: AddressingMode
}

impl ADC {
    pub fn new(mode: AddressingMode) -> Self {
        ADC{ mode }
    }
}

impl Display for ADC {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "ADC {}", self.mode)
    }
}

impl Instruction for ADC {
    fn execute(&self, cpu: &mut CPU) -> u8 {
        let memory_value = cpu.read(&self.mode);
        let existing_carry = cpu.processor_status & 0x01;
        let sum = memory_value as u16 + existing_carry as u16 + cpu.accumulator as u16;

        let carry = sum > 0xFF;
        // Looking at logic implemented here:
        // https://github.com/bfirsh/jsnes/blob/master/src/cpu.js
        let overflow = (cpu.accumulator ^ memory_value) & 0x80 == 0 &&
                             (cpu.accumulator ^ sum as u8) & 0x80 != 0;

        cpu.accumulator = sum as u8;
        cpu.set_flag(StatusFlag::Carry, carry);
        cpu.set_flag(StatusFlag::Zero, cpu.accumulator == 0);
        cpu.set_flag(StatusFlag::Overflow, overflow);
        cpu.set_flag(StatusFlag::Negative, cpu.accumulator > 0x7F);

        cpu.default_cycles(&self.mode)
    }
}

#[cfg(test)]
mod test {
    use crate::cpu::{CPU, Instruction, StatusFlag};
    use crate::cpu::AddressingMode::{ZeroPage, Immediate, AbsoluteX, Absolute};
    use crate::instructions::adc::ADC;

    #[test]
    fn basic_addition() {
        // Given
        let mut cpu = CPU::empty();

        cpu.accumulator = 0x01;

        // When
        let adc = ADC::new(Immediate(0x01));
        adc.execute(&mut cpu);

        // Then
        assert_eq!(0x02, cpu.accumulator);
        assert_eq!(0b0000_0000, cpu.processor_status);
    }

    #[test]
    fn addition_with_carry() {
        // Given
        let mut cpu = CPU::empty();
        let mode = ZeroPage(0xA1);

        cpu.accumulator = 0xF3;         // -115, 243
        cpu.write(&mode, 0x11);   //  17,  17

        // When
        let adc = ADC::new(mode);
        adc.execute(&mut cpu);

        // Then
        // -115 + 17 = -98 so V is not set, 243 + 17 = 260 so C is set
        assert_eq!(0x04, cpu.accumulator);
        assert_eq!(0b0000_0001, cpu.processor_status);
    }

    #[test]
    fn addition_using_carry() {
        // Given
        let mut cpu = CPU::empty();
        let mode = ZeroPage(0xA1);

        cpu.accumulator = 0x03;
        cpu.set_flag(StatusFlag::Carry, true);
        cpu.write(&mode, 0x11);

        // When
        let adc = ADC::new(mode);
        adc.execute(&mut cpu);

        // Then
        assert_eq!(0x15, cpu.accumulator);
        assert_eq!(0b0000_0000, cpu.processor_status);
    }

    #[test]
    fn addition_with_negative() {
        // Given
        let mut cpu = CPU::empty();
        cpu.accumulator = 0xFF;

        // When
        let adc = ADC::new(Immediate(0x89));
        adc.execute(&mut cpu);

        // Then
        assert_eq!(0x88, cpu.accumulator);
        assert_eq!(0b10000001, cpu.processor_status);
    }

    #[test]
    fn addition_with_zero_and_carry() {
        // Given
        let mut cpu = CPU::empty();

        cpu.accumulator = 0xFF;
        let mode = Immediate(0x01);

        // When
        let adc = ADC::new(mode);
        adc.execute(&mut cpu);

        // Then
        assert_eq!(0x00, cpu.accumulator);
        assert_eq!(0b0000_0011, cpu.processor_status);
    }

    #[test]
    fn nestest_scenario1() {
        // Given
        let mut cpu = CPU::empty();
        cpu.accumulator = 0x7F;
        cpu.processor_status = 0x64;

        // When
        ADC::new(Immediate(0x80)).execute(&mut cpu);

        println!("Desired:  {:b}", 0xA4);
        println!("Actual:   {:b}", cpu.processor_status);

        // Then
        assert_eq!(0xFF, cpu.accumulator);
        assert_eq!(0xA4, cpu.processor_status);
    }

    #[test]
    fn nestest_scenario2() {
        // Given
        let mut cpu = CPU::empty();
        cpu.accumulator = 0x00;
        cpu.processor_status = 0x6E;

        // When
        ADC::new(Immediate(0x69)).execute(&mut cpu);

        // Then
        assert_eq!(0x69, cpu.accumulator);
        assert_eq!(0x2C, cpu.processor_status);
    }

    #[test]
    fn string_representation() {
        // Given
        let adc = ADC::new(AbsoluteX(0x003A));

        // Then
        assert_eq!("ADC $003A,X", adc.to_string());
    }

}