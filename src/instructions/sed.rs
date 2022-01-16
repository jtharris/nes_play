use std::fmt::{Display, Formatter};
use crate::cpu::{Instruction, CPU, StatusFlag};

// http://www.obelisk.me.uk/6502/reference.html#SED
pub(super) struct SED {}

impl Display for SED {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "SED")
    }
}

impl Instruction for SED {
    fn execute(&self, cpu: &mut CPU) -> u8 {
        cpu.set_flag(StatusFlag::Decimal, true);

        2
    }

    fn bytes(&self) -> Vec<u8> {
        vec![0xF8]
    }
}

#[cfg(test)]
mod test {
    use crate::cpu::{CPU, Instruction};
    use super::SED;

    #[test]
    fn confirm_decimal_set() {
        // Given
        let mut cpu = CPU::empty();
        cpu.processor_status = 0x00;

        // When
        SED{}.execute(&mut cpu);

        // Then
        assert_eq!(0x08, cpu.processor_status);
    }

    #[test]
    fn existing_decimal_set() {
        // Given
        let mut cpu = CPU::empty();
        cpu.processor_status = 0x09;   // decimal and carry

        // When
        SED{}.execute(&mut cpu);

        // Then
        assert_eq!(0x09, cpu.processor_status);  // still there
    }
}