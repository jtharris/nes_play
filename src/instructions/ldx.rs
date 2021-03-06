use crate::cpu::{CPU, Instruction, AddressingMode, StatusFlag};

// http://www.obelisk.me.uk/6502/reference.html#LDX
pub struct LDX {
    mode: AddressingMode
}

impl LDX {
    pub fn new(mode: AddressingMode) -> Self {
        LDX{ mode }
    }
}

impl Instruction for LDX {
    fn execute(&self, cpu: &mut CPU) {
        cpu.index_register_x = cpu.read(&self.mode);

        cpu.set_flag(StatusFlag::Zero,  cpu.index_register_x == 0);
        cpu.set_flag(StatusFlag::Negative, cpu.index_register_x > 0x7F);
    }
}

#[cfg(test)]
mod test {
    use crate::cpu::{CPU, Instruction};
    use crate::cpu::AddressingMode::{ZeroPage, Immediate};
    use super::LDX;

    #[test]
    fn register_is_loaded_neg() {
        // Given
        let mut cpu = CPU::new();
        cpu.write(&ZeroPage(0x88), 0xF1);

        // When
        LDX::new(ZeroPage(0x88)).execute(&mut cpu);

        // Then
        assert_eq!(0xF1, cpu.index_register_x);
        assert_eq!(0x80, cpu.processor_status);  // Only negative bit is set
    }

    #[test]
    fn register_is_loaded_zero() {
        // Given
        let mut cpu = CPU::new();
        cpu.index_register_x = 0x89;

        // When
        LDX::new(Immediate(0x00)).execute(&mut cpu);

        // Then
        assert_eq!(0x00, cpu.index_register_x);
        assert_eq!(0x02, cpu.processor_status);  // Only zero bit is set
    }

    #[test]
    fn register_is_loaded_no_flags() {
        // Given
        let mut cpu = CPU::new();
        cpu.index_register_x = 0x89;

        // When
        LDX::new(Immediate(0x6A)).execute(&mut cpu);

        // Then
        assert_eq!(0x6A, cpu.index_register_x);
        assert_eq!(0x00, cpu.processor_status);
    }
}
