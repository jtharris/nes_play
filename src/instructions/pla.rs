use crate::cpu::{Instruction, CPU, StatusFlag};

// http://www.obelisk.me.uk/6502/reference.html#PLA
pub(super) struct PLA {}

impl Instruction for PLA {
    fn execute(&self, cpu: &mut CPU) {
        cpu.accumulator = cpu.pop_stack();

        cpu.set_flag(StatusFlag::Zero, cpu.accumulator == 0);
        cpu.set_flag(StatusFlag::Negative, cpu.accumulator > 0x7F);
    }
}


#[cfg(test)]
mod test {
    use crate::cpu::{CPU, Instruction};
    use crate::instructions::pla::PLA;

    #[test]
    fn acc_is_pulled() {
        // Given
        let mut cpu = CPU::new();
        cpu.push_stack(0x8C);

        // When
        PLA{}.execute(&mut cpu);

        // Then
        assert_eq!(0x8C, cpu.accumulator);
        assert_eq!(0x80, cpu.processor_status);  // Negative flag
    }

    #[test]
    fn acc_is_pulled_zero_flag() {
        // Given
        let mut cpu = CPU::new();
        cpu.accumulator = 0xFA;
        cpu.push_stack(0x00);

        // When
        PLA{}.execute(&mut cpu);

        // Then
        assert_eq!(0x00, cpu.accumulator);
        assert_eq!(0x02, cpu.processor_status);  // Zero flag
    }
}