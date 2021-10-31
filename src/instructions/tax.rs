use crate::cpu::{Instruction, CPU, StatusFlag};

// http://www.obelisk.me.uk/6502/reference.html#TAX
pub(super) struct TAX {}

impl Instruction for TAX {
    fn execute(&self, cpu: &mut CPU) {
        cpu.index_register_x = cpu.accumulator;

        cpu.set_flag(StatusFlag::Zero, cpu.index_register_x == 0);
        cpu.set_flag(StatusFlag::Negative, cpu.index_register_x > 0x7F);
    }
}

#[cfg(test)]
mod test {
    use crate::cpu::{CPU, Instruction};
    use super::TAX;

    #[test]
    fn copies_to_x_no_flags() {
        // Given
        let mut cpu = CPU::new();
        cpu.accumulator = 0x7E;

        // When
        TAX{}.execute(&mut cpu);

        // Then
        assert_eq!(0x7E, cpu.index_register_x);
        assert_eq!(0x00, cpu.processor_status);
    }

    #[test]
    fn copies_to_x_zero_flag() {
        // Given
        let mut cpu = CPU::new();
        cpu.accumulator = 0x00;
        cpu.index_register_x = 0xAF;

        // When
        TAX{}.execute(&mut cpu);

        // Then
        assert_eq!(0x00, cpu.index_register_x);
        assert_eq!(0x02, cpu.processor_status);  // zero flag
    }

    #[test]
    fn copies_to_x_negative_flag() {
        // Given
        let mut cpu = CPU::new();
        cpu.accumulator = 0xFC;
        cpu.index_register_x = 0xAF;

        // When
        TAX{}.execute(&mut cpu);

        // Then
        assert_eq!(0xFC, cpu.index_register_x);
        assert_eq!(0x80, cpu.processor_status);  // negative flag
    }
}
