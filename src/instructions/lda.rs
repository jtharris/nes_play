use crate::cpu::{CPU, Instruction, AddressingMode};

// http://www.obelisk.me.uk/6502/reference.html#LDA
pub struct LDA {
    mode: AddressingMode
}

impl LDA {
    pub fn new(mode: AddressingMode) -> Self {
        LDA{ mode }
    }
}

impl Instruction for LDA {
    fn execute(&self, cpu: &mut CPU) {
        cpu.accumulator = cpu.read(&self.mode);
    }
}

#[cfg(test)]
mod test {
    use crate::cpu::{CPU, Instruction};
    use crate::cpu::AddressingMode::ZeroPage;
    use super::LDA;

    #[test]
    fn accumulator_is_loaded() {
        // Given
        let mut cpu = CPU::new();
        cpu.write(&ZeroPage(0x88), 0xF1);

        // When
        LDA::new(ZeroPage(0x88)).execute(&mut cpu);
        
        // Then
        assert_eq!(0xF1, cpu.accumulator);
    }
}
