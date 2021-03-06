use crate::cpu::{CPU, Instruction, StatusFlag};

// http://www.obelisk.me.uk/6502/reference.html#CLD
// Note:  NES hardware doesn't even support this mode, so this
// likely will never be used...
pub struct CLD {}

impl Instruction for CLD {
    fn execute(&self, cpu: &mut CPU) {
        cpu.set_flag(StatusFlag::Decimal, false);
    }
}


#[cfg(test)]
mod test {
    use crate::cpu::{CPU, Instruction};
    use self::super::CLD;

    #[test]
    fn clears_set_d() {
        // Given
        let mut cpu = CPU::new();
        cpu.processor_status = 0b0000_1000;

        // When
        CLD{}.execute(&mut cpu);

        // Then
        assert_eq!(0b0000_0000, cpu.processor_status);
    }

    #[test]
    fn zeroed_d_is_unchanged() {
        // Given
        let mut cpu = CPU::new();
        cpu.processor_status = 0b0100_0101;

        // When
        CLD{}.execute(&mut cpu);

        // Then
        assert_eq!(0b0100_0101, cpu.processor_status);
    }
}
