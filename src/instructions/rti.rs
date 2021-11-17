use crate::cpu::{Instruction, CPU};

// http://www.obelisk.me.uk/6502/reference.html#RTI
pub(super) struct RTI {}

impl Instruction for RTI {
    fn execute(&self, cpu: &mut CPU) -> u8 {
        cpu.processor_status = cpu.pop_stack();

        let low_pc = cpu.pop_stack();
        let high_pc = cpu.pop_stack();
        cpu.program_counter = u16::from_be_bytes([high_pc, low_pc]);

        6
    }
}

#[cfg(test)]
mod test {
    use crate::cpu::{CPU, Instruction};
    use super::RTI;

    #[test]
    fn values_all_pulled() {
        // Given
        let mut cpu = CPU::new();
        cpu.push_stack(0xA1);
        cpu.push_stack(0xC3);
        cpu.push_stack(0x03);       // S - carry and zero flags set

        // When
        RTI{}.execute(&mut cpu);

        // Then
        assert_eq!(0x03, cpu.processor_status);
        assert_eq!(0xA1C3, cpu.program_counter);
    }
}