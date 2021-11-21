use crate::cpu::{CPU, Instruction, StatusFlag, AddressingMode};

// http://www.obelisk.me.uk/6502/reference.html#INC
pub(super) struct INC {
    mode: AddressingMode
}

impl INC {
    pub fn new(mode: AddressingMode) -> Self {
        INC{ mode }
    }
}

impl Instruction for INC {
    fn execute(&self, cpu: &mut CPU) -> u8 {
        let (val, _) = cpu.read(&self.mode).overflowing_add(1);
        cpu.write(&self.mode, val);

        cpu.set_flag(StatusFlag::Zero,  val == 0);
        cpu.set_flag(StatusFlag::Negative, val > 0x7F);

        match self.mode {
            AddressingMode::ZeroPage(_) => 5,
            AddressingMode::ZeroPageX(_) => 6,
            AddressingMode::Absolute(_) => 6,
            AddressingMode::AbsoluteX(_) => 7,
            _ => panic!("Invalid addressing mode for INC")
        }
    }
}

#[cfg(test)]
mod test {
    use crate::cpu::{CPU, Instruction};
    use crate::cpu::AddressingMode::ZeroPage;
    use crate::instructions::inc::INC;

    #[test]
    fn basic_increment() {
        // Given
        let mut cpu = CPU::new();
        let mode = ZeroPage(0xAA);
        cpu.write(&mode, 0x0C);

        // When
        INC::new(mode).execute(&mut cpu);

        // Then
        assert_eq!(0x0D, cpu.read(&ZeroPage(0xAA)));
        assert_eq!(0, cpu.processor_status);    // Make sure no bits were set
    }

    #[test]
    fn negative_result() {
        // Given
        let mut cpu = CPU::new();
        let mode = ZeroPage(0x04);
        cpu.write(&mode, 0xFD);

        // When
        INC::new(mode).execute(&mut cpu);

        // Then
        assert_eq!(0xFE, cpu.read(&ZeroPage(0x04)));
        assert_eq!(0x80, cpu.processor_status);    // Negative flag is set
    }

    #[test]
    fn zero_wrap() {
        // Given
        let mut cpu = CPU::new();
        let mode = ZeroPage(0x04);
        cpu.write(&mode, 0xFF);

        // When
        INC::new(mode).execute(&mut cpu);

        // Then
        assert_eq!(0x00, cpu.read(&ZeroPage(0x04)));
        assert_eq!(0x02, cpu.processor_status);    // Zero flag is set
    }
}
