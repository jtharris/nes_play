use std::fmt::{Display, Formatter};
use crate::cpu::{Instruction, CPU, StatusFlag};

// http://www.obelisk.me.uk/6502/reference.html#PLA
pub(super) struct PLA {}

impl Display for PLA {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "PLA")
    }
}

impl Instruction for PLA {
    fn execute(&self, cpu: &mut CPU) -> u8 {
        cpu.accumulator = cpu.pop_stack();

        cpu.set_flag(StatusFlag::Zero, cpu.accumulator == 0);
        cpu.set_flag(StatusFlag::Negative, cpu.accumulator > 0x7F);

        4
    }
}


#[cfg(test)]
mod test {
    use crate::cpu::{CPU, Instruction};
    use crate::instructions::pla::PLA;

    #[test]
    fn acc_is_pulled() {
        // Given
        let mut cpu = CPU::empty();
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
        let mut cpu = CPU::empty();
        cpu.accumulator = 0xFA;
        cpu.push_stack(0x00);

        // When
        PLA{}.execute(&mut cpu);

        // Then
        assert_eq!(0x00, cpu.accumulator);
        assert_eq!(0x02, cpu.processor_status);  // Zero flag
    }
}