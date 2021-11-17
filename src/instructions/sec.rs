use crate::cpu::{Instruction, CPU, StatusFlag};

// http://www.obelisk.me.uk/6502/reference.html#SEC
pub(super) struct SEC {}

impl Instruction for SEC {
    fn execute(&self, cpu: &mut CPU) -> u8 {
        cpu.set_flag(StatusFlag::Carry, true);

        2
    }
}

#[cfg(test)]
mod test {
    use crate::cpu::{CPU, Instruction};
    use super::SEC;

    #[test]
    fn confirm_carry_set() {
        // Given
        let mut cpu = CPU::new();
        cpu.processor_status = 0x00;

        // When
        SEC{}.execute(&mut cpu);

        // Then
        assert_eq!(0x01, cpu.processor_status);
    }

    #[test]
    fn existing_carry_set() {
        // Given
        let mut cpu = CPU::new();
        cpu.processor_status = 0x01;

        // When
        SEC{}.execute(&mut cpu);

        // Then
        assert_eq!(0x01, cpu.processor_status);  // still there
    }
}