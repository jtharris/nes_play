use crate::cpu::{CPU, Instruction, StatusFlag, AddressingMode};

// http://www.obelisk.me.uk/6502/reference.html#DEC
pub struct DEC {
    mode: AddressingMode
}

impl DEC {
    pub fn new(mode: AddressingMode) -> Self {
        DEC{ mode }
    }
}

impl Instruction for DEC {
    fn execute(&self, cpu: &mut CPU) {
        let (val, _) = cpu.read(&self.mode).overflowing_sub(1);
        cpu.write(&self.mode, val);

        cpu.set_flag(StatusFlag::Zero,  val == 0);
        cpu.set_flag(StatusFlag::Negative, val > 0x7F);
    }
}

#[cfg(test)]
mod test {
    use crate::cpu::{CPU, Instruction};
    use crate::cpu::AddressingMode::ZeroPage;
    use crate::instructions::dec::DEC;

    #[test]
    fn basic_decrement() {
        // Given
        let mut cpu = CPU::new();
        let mode = ZeroPage(0xAA);
        cpu.write(&mode, 0x0C);

        // When
        DEC::new(mode).execute(&mut cpu);

        // Then
        assert_eq!(0x0B, cpu.read(&ZeroPage(0xAA)));
        assert_eq!(0, cpu.processor_status);    // Make sure no bits were set
    }

    #[test]
    fn zero_result() {
        // Given
        let mut cpu = CPU::new();
        let mode = ZeroPage(0x09);
        cpu.write(&mode, 0x01);

        // When
        DEC::new(mode).execute(&mut cpu);

        // Then
        assert_eq!(0, cpu.read(&ZeroPage(0x09)));
        assert_eq!(0x02, cpu.processor_status);    // Zero flag is set
    }

    #[test]
    fn negative_result() {
        // Given
        let mut cpu = CPU::new();
        let mode = ZeroPage(0x04);
        cpu.write(&mode, 0xFD);

        // When
        DEC::new(mode).execute(&mut cpu);

        // Then
        assert_eq!(0xFC, cpu.read(&ZeroPage(0x04)));
        assert_eq!(0x80, cpu.processor_status);    // Negative flag is set
    }

    #[test]
    fn negative_wrap() {
        // Given
        let mut cpu = CPU::new();
        let mode = ZeroPage(0x04);
        cpu.write(&mode, 0x00);

        // When
        DEC::new(mode).execute(&mut cpu);

        // Then
        assert_eq!(0xFF, cpu.read(&ZeroPage(0x04)));
        assert_eq!(0x80, cpu.processor_status);    // Negative flag is set
    }
}
