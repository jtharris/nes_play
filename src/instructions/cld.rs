use std::fmt::{Display, Formatter};
use crate::cpu::{CPU, Instruction, StatusFlag};

// http://www.obelisk.me.uk/6502/reference.html#CLD
// Note:  NES hardware doesn't even support this mode, so this
// likely will never be used...
pub(super) struct CLD {}

impl Display for CLD {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "CLD")
    }
}

impl Instruction for CLD {
    fn execute(&self, cpu: &mut CPU) -> u8 {
        cpu.set_flag(StatusFlag::Decimal, false);
        2
    }

    fn bytes(&self) -> Vec<u8> {
        todo!()
    }
}


#[cfg(test)]
mod test {
    use crate::cpu::{CPU, Instruction};
    use self::super::CLD;

    #[test]
    fn clears_set_d() {
        // Given
        let mut cpu = CPU::empty();
        cpu.processor_status = 0b0000_1000;

        // When
        CLD{}.execute(&mut cpu);

        // Then
        assert_eq!(0b0000_0000, cpu.processor_status);
    }

    #[test]
    fn zeroed_d_is_unchanged() {
        // Given
        let mut cpu = CPU::empty();
        cpu.processor_status = 0b0100_0101;

        // When
        CLD{}.execute(&mut cpu);

        // Then
        assert_eq!(0b0100_0101, cpu.processor_status);
    }
}
