use std::fmt::{Display, Formatter};
use crate::cpu::{CPU, Instruction, StatusFlag};

// http://www.obelisk.me.uk/6502/reference.html#CLC
pub struct CLC {}

impl Display for CLC {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "CLC")
    }
}

impl Instruction for CLC {
    fn execute(&self, cpu: &mut CPU) -> u8 {
        cpu.set_flag(StatusFlag::Carry, false);
        2
    }
}


#[cfg(test)]
mod test {
    use crate::cpu::{CPU, Instruction};
    use self::super::CLC;

    #[test]
    fn clears_set_c() {
        // Given
        let mut cpu = CPU::empty();
        cpu.processor_status = 0b0000_0001;

        // When
        CLC{}.execute(&mut cpu);

        // Then
        assert_eq!(0b0000_0000, cpu.processor_status);
    }

    #[test]
    fn zeroed_c_is_unchanged() {
        // Given
        let mut cpu = CPU::empty();
        cpu.processor_status = 0b0100_1100;

        // When
        CLC{}.execute(&mut cpu);

        // Then
        assert_eq!(0b0100_1100, cpu.processor_status);
    }
}
