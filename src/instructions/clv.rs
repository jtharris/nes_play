use std::fmt::{Display, Formatter};
use crate::cpu::{CPU, Instruction, StatusFlag};

// http://www.obelisk.me.uk/6502/reference.html#CLV
pub(super) struct CLV {}

impl Display for CLV {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "CLV")
    }
}

impl Instruction for CLV {
    fn execute(&self, cpu: &mut CPU) -> u8 {
        cpu.set_flag(StatusFlag::Overflow, false);
        2
    }
}


#[cfg(test)]
mod test {
    use crate::cpu::{CPU, Instruction};
    use self::super::CLV;

    #[test]
    fn clears_set_id() {
        // Given
        let mut cpu = CPU::empty();
        cpu.processor_status = 0b0100_0000;

        // When
        CLV{}.execute(&mut cpu);

        // Then
        assert_eq!(0b0000_0000, cpu.processor_status);
    }

    #[test]
    fn zeroed_id_is_unchanged() {
        // Given
        let mut cpu = CPU::empty();
        cpu.processor_status = 0b1000_1001;

        // When
        CLV{}.execute(&mut cpu);

        // Then
        assert_eq!(0b1000_1001, cpu.processor_status);
    }
}
