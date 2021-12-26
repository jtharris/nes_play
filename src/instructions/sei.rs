use std::fmt::{Display, Formatter};
use crate::cpu::{Instruction, CPU, StatusFlag};

// http://www.obelisk.me.uk/6502/reference.html#SEI
pub(super) struct SEI {}

impl Display for SEI {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "SEI")
    }
}

impl Instruction for SEI {
    fn execute(&self, cpu: &mut CPU) -> u8 {
        cpu.set_flag(StatusFlag::InterruptDisable, true);

        2
    }

    fn bytes(&self) -> Vec<u8> {
        todo!()
    }
}

#[cfg(test)]
mod test {
    use crate::cpu::{CPU, Instruction};
    use super::SEI;

    #[test]
    fn confirm_interrupt_set() {
        // Given
        let mut cpu = CPU::empty();
        cpu.processor_status = 0x00;

        // When
        SEI{}.execute(&mut cpu);

        // Then
        assert_eq!(0x04, cpu.processor_status);
    }

    #[test]
    fn existing_interrupt_set() {
        // Given
        let mut cpu = CPU::empty();
        cpu.processor_status = 0x0C;   // decimal and interrupt

        // When
        SEI{}.execute(&mut cpu);

        // Then
        assert_eq!(0x0C, cpu.processor_status);  // still there
    }
}