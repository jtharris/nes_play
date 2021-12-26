use std::fmt::{Display, Formatter};
use crate::cpu::{Instruction, CPU};

// http://www.obelisk.me.uk/6502/reference.html#RTI
pub(super) struct RTI {}

impl Display for RTI {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "RTI")
    }
}

impl Instruction for RTI {
    fn execute(&self, cpu: &mut CPU) -> u8 {
        let existing_bits = cpu.processor_status & 0b0011_0000;
        let stack_value = cpu.pop_stack() & 0b1100_1111;
        cpu.processor_status = stack_value | existing_bits;

        let low_pc = cpu.pop_stack();
        let high_pc = cpu.pop_stack();
        cpu.program_counter = u16::from_be_bytes([high_pc, low_pc]);

        6
    }

    fn bytes(&self) -> Vec<u8> {
        todo!()
    }
}

#[cfg(test)]
mod test {
    use crate::cpu::{CPU, Instruction};
    use super::RTI;

    #[test]
    fn values_all_pulled() {
        // Given
        let mut cpu = CPU::empty();
        cpu.push_stack(0xA1);
        cpu.push_stack(0xC3);
        cpu.push_stack(0x03);       // S - carry and zero flags set

        // When
        RTI{}.execute(&mut cpu);

        // Then
        assert_eq!(0x03, cpu.processor_status);
        assert_eq!(0xA1C3, cpu.program_counter);
    }

    #[test]
    fn nestest_scenario1() {
        // Given
        let mut cpu = CPU::empty();
        cpu.accumulator = 0x55;
        cpu.index_register_x = 0x99;
        cpu.index_register_y = 0x88;
        cpu.processor_status = 0xA5;

        cpu.push_stack(0xCE);
        cpu.push_stack(0xAE);
        cpu.push_stack(0x65);

        // When
        RTI{}.execute(&mut cpu);

        // Then
        assert_eq!(0x55, cpu.accumulator);
        assert_eq!(0x99, cpu.index_register_x);
        assert_eq!(0x88, cpu.index_register_y);
        assert_eq!(0x65, cpu.processor_status);
        assert_eq!(0xCEAE, cpu.program_counter);
    }

    #[test]
    fn nestest_scenario2() {
        // Given
        let mut cpu = CPU::empty();
        cpu.accumulator = 0x55;
        cpu.index_register_x = 0x99;
        cpu.index_register_y = 0x88;
        cpu.processor_status = 0x65;

        cpu.push_stack(0xCE);
        cpu.push_stack(0xCE);
        cpu.push_stack(0x87);

        // When
        RTI{}.execute(&mut cpu);

        // Then
        assert_eq!(0x55, cpu.accumulator);
        assert_eq!(0x99, cpu.index_register_x);
        assert_eq!(0x88, cpu.index_register_y);
        assert_eq!(0xA7, cpu.processor_status);
        assert_eq!(0xCECE, cpu.program_counter);
    }
}