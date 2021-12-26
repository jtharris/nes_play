use std::fmt::{Display, Formatter};
use crate::cpu::{CPU, Instruction, StatusFlag};

// http://www.obelisk.me.uk/6502/reference.html#CLI
pub struct CLI {}

impl Display for CLI {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "CLI")
    }
}

impl Instruction for CLI {
    fn execute(&self, cpu: &mut CPU) -> u8 {
        cpu.set_flag(StatusFlag::InterruptDisable, false);
        2
    }

    fn bytes(&self) -> Vec<u8> {
        todo!()
    }
}


#[cfg(test)]
mod test {
    use crate::cpu::{CPU, Instruction};
    use self::super::CLI;

    #[test]
    fn clears_set_id() {
        // Given
        let mut cpu = CPU::empty();
        cpu.processor_status = 0b0000_0100;

        // When
        CLI{}.execute(&mut cpu);

        // Then
        assert_eq!(0b0000_0000, cpu.processor_status);
    }

    #[test]
    fn zeroed_id_is_unchanged() {
        // Given
        let mut cpu = CPU::empty();
        cpu.processor_status = 0b0100_1001;

        // When
        CLI{}.execute(&mut cpu);

        // Then
        assert_eq!(0b0100_1001, cpu.processor_status);
    }
}
