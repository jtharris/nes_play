use std::fmt::{Display, Formatter};
use crate::cpu::{CPU, Instruction, AddressingMode, StatusFlag};

// http://www.obelisk.me.uk/6502/reference.html#LDA
pub(super) struct LDA {
    mode: AddressingMode
}

impl LDA {
    pub fn new(mode: AddressingMode) -> Self {
        LDA{ mode }
    }
}

impl Display for LDA {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "LDA {}", self.mode)
    }
}

impl Instruction for LDA {
    fn execute(&self, cpu: &mut CPU) -> u8 {
        cpu.accumulator = cpu.read(&self.mode);

        cpu.set_flag(StatusFlag::Zero,  cpu.accumulator == 0);
        cpu.set_flag(StatusFlag::Negative, cpu.accumulator > 0x7F);

        cpu.default_cycles(&self.mode)
    }
}

#[cfg(test)]
mod test {
    use crate::cpu::{CPU, Instruction};
    use crate::cpu::AddressingMode::{ZeroPage, Immediate, IndirectX};
    use super::LDA;

    #[test]
    fn accumulator_is_loaded_neg() {
        // Given
        let mut cpu = CPU::empty();
        cpu.write(&ZeroPage(0x88), 0xF1);

        // When
        LDA::new(ZeroPage(0x88)).execute(&mut cpu);

        // Then
        assert_eq!(0xF1, cpu.accumulator);
        assert_eq!(0x80, cpu.processor_status);  // Only negative bit is set
    }

    #[test]
    fn accumulator_is_loaded_zero() {
        // Given
        let mut cpu = CPU::empty();
        cpu.accumulator = 0x89;

        // When
        LDA::new(Immediate(0x00)).execute(&mut cpu);

        // Then
        assert_eq!(0x00, cpu.accumulator);
        assert_eq!(0x02, cpu.processor_status);  // Only zero bit is set
    }

    #[test]
    fn accumulator_is_loaded_no_flags() {
        // Given
        let mut cpu = CPU::empty();
        cpu.accumulator = 0x89;

        // When
        LDA::new(Immediate(0x6A)).execute(&mut cpu);

        // Then
        assert_eq!(0x6A, cpu.accumulator);
        assert_eq!(0x00, cpu.processor_status);
    }

    #[test]
    fn string_representation() {
        let lda = LDA::new(IndirectX(0x4D));

        assert_eq!("LDA ($4D,X)", lda.to_string())
    }
}
