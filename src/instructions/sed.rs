use crate::cpu::{Instruction, CPU, StatusFlag};

// http://www.obelisk.me.uk/6502/reference.html#SED
pub(super) struct SED {}

impl Instruction for SED {
    fn execute(&self, cpu: &mut CPU) {
        cpu.set_flag(StatusFlag::Decimal, true);
    }
}

#[cfg(test)]
mod test {
    use crate::cpu::{CPU, Instruction};
    use super::SED;

    #[test]
    fn confirm_decimal_set() {
        // Given
        let mut cpu = CPU::new();
        cpu.processor_status = 0x00;

        // When
        SED{}.execute(&mut cpu);

        // Then
        assert_eq!(0x08, cpu.processor_status);
    }

    #[test]
    fn existing_decimal_set() {
        // Given
        let mut cpu = CPU::new();
        cpu.processor_status = 0x09;   // decimal and carry

        // When
        SED{}.execute(&mut cpu);

        // Then
        assert_eq!(0x09, cpu.processor_status);  // still there
    }
}