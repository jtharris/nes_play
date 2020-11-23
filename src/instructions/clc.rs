use crate::cpu::{CPU, Instruction};

// http://www.obelisk.me.uk/6502/reference.html#CLC
pub struct CLC {}

impl Instruction for CLC {
    fn execute(&self, cpu: &mut CPU) {
        cpu.processor_status &= 0xFE;
    }
}


#[cfg(test)]
mod test {
    use crate::cpu::{CPU, Instruction};
    use self::super::CLC;

    #[test]
    fn clears_set_c() {
        // Given
        let mut cpu = CPU::new();
        cpu.processor_status = 0b00000001;

        // When
        let clc = CLC{};
        clc.execute(&mut cpu);

        // Then
        assert_eq!(0b00000000, cpu.processor_status);
    }

    #[test]
    fn zeroed_c_is_unchanged() {
        // Given
        let mut cpu = CPU::new();
        cpu.processor_status = 0b01001100;

        // When
        let clc = CLC{};
        clc.execute(&mut cpu);

        // Then
        assert_eq!(0b01001100, cpu.processor_status);
    }
}
