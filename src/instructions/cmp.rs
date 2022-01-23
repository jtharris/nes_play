use std::fmt::{Display, Formatter};
use crate::cpu::{CPU, Instruction, StatusFlag, AddressingMode};

// http://www.obelisk.me.uk/6502/reference.html#CMP
pub struct CMP {
    mode: AddressingMode
}

impl CMP {
    pub fn new(mode: AddressingMode) -> Self {
        CMP{ mode }
    }
}

impl Display for CMP {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "CMP {}", self.mode)
    }
}

impl Instruction for CMP {
    fn execute(&self, cpu: &mut CPU) -> u8 {
        let mem = cpu.read(&self.mode);

        cpu.set_flag(StatusFlag::Carry, cpu.accumulator >= mem);
        cpu.set_flag(StatusFlag::Zero, cpu.accumulator == mem);
        cpu.set_flag(StatusFlag::Negative, cpu.accumulator.wrapping_sub(mem) > 0x7F);

        cpu.default_cycles(&self.mode)
    }

    fn bytes(&self) -> Vec<u8> {
        match self.mode {
            AddressingMode::Immediate(val) => vec![0xC9, val],
            AddressingMode::ZeroPage(addr) => vec![0xC5, addr],
            AddressingMode::ZeroPageX(addr) => vec![0xD5, addr],
            AddressingMode::Absolute(addr) => self.bytes_for_opcode(0xCD, addr),
            AddressingMode::AbsoluteX(addr) => self.bytes_for_opcode(0xDD, addr),
            AddressingMode::AbsoluteY(addr) => self.bytes_for_opcode(0xD9, addr),
            AddressingMode::IndirectX(addr) => vec![0xC1, addr],
            AddressingMode::IndirectY(addr) => vec![0xD1, addr],
            _ => panic!("Addressing mode not allowed for CMP")
        }

    }

    fn debug_string(&self, cpu: &CPU) -> String {
        format!("CMP {}", self.mode.debug_string(&cpu))
    }
}

#[cfg(test)]
mod test {
    use crate::cpu::{CPU, AddressingMode::AbsoluteX, AddressingMode::Immediate, Instruction};
    use crate::cpu::AddressingMode::{Absolute, AbsoluteY};
    use super::CMP;

    #[test]
    fn values_are_equal_positive() {
        // Given
        let mut cpu = CPU::empty();
        cpu.accumulator = 0x3B;

        // When
        CMP::new(Immediate(0x3B)).execute(&mut cpu);

        // Then
        assert_eq!(0b0000_0011, cpu.processor_status);
    }

    #[test]
    fn acc_greater_and_negative() {
        // Given
        let mut cpu = CPU::empty();
        cpu.accumulator = 0xAB;

        // When
        CMP::new(Immediate(0x0B)).execute(&mut cpu);

        // Then
        assert_eq!(0b1000_0001, cpu.processor_status);
    }

    #[test]
    fn acc_less_and_positive() {
        // Given
        let mut cpu = CPU::empty();
        cpu.accumulator = 0x02;

        // When
        let mode = AbsoluteX(0x88);
        cpu.write(&mode, 0x01);
        CMP::new(mode).execute(&mut cpu);

        // Then
        assert_eq!(0b0000_0001, cpu.processor_status);
    }

    #[test]
    fn nestest_scenario1() {
        // Given
        let mut cpu = CPU::empty();
        cpu.accumulator = 0xFF;
        cpu.processor_status = 0xE4;

        // When
        CMP::new(Immediate(0xFF)).execute((&mut cpu));

        // Then
        assert_eq!(0x67, cpu.processor_status);
    }

    #[test]
    fn nestest_scenario2() {
        // Given
        let mut cpu = CPU::empty();
        cpu.accumulator = 0x40;
        cpu.processor_status = 0x25;

        // When
        CMP::new(Immediate(0x41)).execute((&mut cpu));

        // Then
        assert_eq!(0x40, cpu.accumulator);
        assert_eq!(0xA4, cpu.processor_status);
    }

    #[test]
    fn string_representation() {
        let cmp = CMP::new(Absolute(0x0A1E));

        assert_eq!("CMP $0A1E", cmp.to_string())
    }

    #[test]
    fn bytes_representation() {
        let cmp = CMP::new(AbsoluteY(0x0AAE));

        assert_eq!(vec![0xD9, 0xAE, 0x0A], cmp.bytes());
    }
}
