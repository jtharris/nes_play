use crate::cpu::{CPU, Instruction, StatusFlag};

// http://www.obelisk.me.uk/6502/reference.html#CLV
pub struct CLV {}

impl Instruction for CLV {
    fn execute(&self, cpu: &mut CPU) {
        cpu.set_flag(StatusFlag::Overflow, false);
    }
}


#[cfg(test)]
mod test {
    use crate::cpu::{CPU, Instruction};
    use self::super::CLV;

    #[test]
    fn clears_set_id() {
        // Given
        let mut cpu = CPU::new();
        cpu.processor_status = 0b0100_0000;

        // When
        CLV{}.execute(&mut cpu);

        // Then
        assert_eq!(0b0000_0000, cpu.processor_status);
    }

    #[test]
    fn zeroed_id_is_unchanged() {
        // Given
        let mut cpu = CPU::new();
        cpu.processor_status = 0b1000_1001;

        // When
        CLV{}.execute(&mut cpu);

        // Then
        assert_eq!(0b1000_1001, cpu.processor_status);
    }
}
