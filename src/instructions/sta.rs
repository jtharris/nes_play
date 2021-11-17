use crate::cpu::{AddressingMode, Instruction, CPU};

// http://www.obelisk.me.uk/6502/reference.html#STA
struct STA {
    mode: AddressingMode
}

impl STA {
    pub fn new(mode: AddressingMode) -> Self {
        STA{ mode }
    }
}

impl Instruction for STA {
    fn execute(&self, cpu: &mut CPU) -> u8 {
        cpu.write(&self.mode, cpu.accumulator);

        match self.mode {
            AddressingMode::ZeroPage(_) => 3,
            AddressingMode::ZeroPageX(_) => 4,
            AddressingMode::Absolute(_) => 4,
            AddressingMode::AbsoluteX(_) => 5,
            AddressingMode::AbsoluteY(_) => 5,
            AddressingMode::IndirectX(_) => 6,
            AddressingMode::IndirectY(_) => 6,
            _ => panic!("Invalid addressing mode for STA")
        }
    }
}

#[cfg(test)]
mod test {
    use crate::cpu::{CPU, Instruction};
    use crate::cpu::AddressingMode::ZeroPage;
    use super::STA;

    #[test]
    fn acc_is_stored() {
        // Given
        let mut cpu = CPU::new();
        cpu.accumulator = 0xA7;

        // When
        STA::new(ZeroPage(0x88)).execute(&mut cpu);

        // Then
        assert_eq!(0xA7, cpu.read(&ZeroPage(0x88)))
    }
}
