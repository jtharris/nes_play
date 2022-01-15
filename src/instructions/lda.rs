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

    fn bytes(&self) -> Vec<u8> {
        match self.mode {
            AddressingMode::Immediate(val) => vec![0xA9, val],
            AddressingMode::ZeroPage(addr) => vec![0xA5, addr],
            AddressingMode::ZeroPageX(addr) => vec![0xB5, addr],
            AddressingMode::Absolute(addr) => self.bytes_for_opcode(0xAD, addr),
            AddressingMode::AbsoluteX(addr) => self.bytes_for_opcode(0xBD, addr),
            AddressingMode::AbsoluteY(addr) => self.bytes_for_opcode(0xB9, addr),
            AddressingMode::IndirectX(addr) => vec![0xA1, addr],
            AddressingMode::IndirectY(addr) => vec![0xB1, addr],
            _ => panic!("Addressing mode not allowed for LDA")
        }
    }
}

#[cfg(test)]
mod test {
    use crate::cpu::{CPU, Instruction};
    use crate::cpu::AddressingMode::{ZeroPage, Immediate, IndirectX, IndirectY, Absolute, AbsoluteX};
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
    fn nestest_scenario1() {
        // Given
        let mut cpu = CPU::empty();
        cpu.accumulator = 0x01;
        cpu.index_register_x = 0x65;
        cpu.index_register_y = 0xFF;
        cpu.processor_status = 0xE5;

        cpu.write(&ZeroPage(0xFF), 0x46);
        cpu.write(&ZeroPage(0x00), 0x01);
        cpu.write(&Absolute(0x0245), 0x12);

        // When
        LDA::new(IndirectY(0xFF)).execute(&mut cpu);

        // Then
        assert_eq!(0x12, cpu.accumulator);
        assert_eq!(0x65, cpu.processor_status);
    }

    #[test]
    fn string_representation() {
        let lda = LDA::new(IndirectX(0x4D));

        assert_eq!("LDA ($4D,X)", lda.to_string())
    }

    #[test]
    fn bytes_representation() {
        let lda = LDA::new(AbsoluteX(0xB280));

        assert_eq!(vec![0xBD, 0x80, 0xB2], lda.bytes());
    }
}
