use std::fmt::{Display, Formatter};
use crate::cpu::{AddressingMode, CPU, Instruction, StatusFlag};

// https://www.masswerk.at/6502/6502_instruction_set.html#SLO
pub(super) struct SLO {
    mode: AddressingMode
}

impl SLO {
    pub fn new(mode: AddressingMode) -> Self {
        SLO { mode }
    }
}

impl Display for SLO {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "*SLO {}", self.mode)
    }
}

impl Instruction for SLO {
    fn execute(&self, cpu: &mut CPU) -> u8 {
        let mem_value = cpu.read(&self.mode);
        let shifted_value = mem_value << 1;

        cpu.write(&self.mode, shifted_value);
        cpu.accumulator |= shifted_value;

        cpu.set_flag(StatusFlag::Carry, mem_value > 0x7F);
        cpu.set_flag(StatusFlag::Zero, cpu.accumulator == 0);
        cpu.set_flag(StatusFlag::Negative, cpu.accumulator > 0x7F);

        cpu.memory_cycles(&self.mode)
    }

    fn bytes(&self) -> Vec<u8> {
        match self.mode {
            AddressingMode::ZeroPage(addr) => vec![0x07, addr],
            AddressingMode::ZeroPageX(addr) => vec![0x17, addr],
            AddressingMode::Absolute(addr) => self.bytes_for_opcode(0x0F, addr),
            AddressingMode::AbsoluteX(addr) => self.bytes_for_opcode(0x1F, addr),
            AddressingMode::AbsoluteY(addr) => self.bytes_for_opcode(0x1B, addr),
            AddressingMode::IndirectX(addr) => vec![0x03, addr],
            AddressingMode::IndirectY(addr) => vec![0x13, addr],
            _ => panic!("Addressing mode not allowed for SLO")
        }
    }
}

#[cfg(test)]
mod test {
    use crate::cpu::{AddressingMode, CPU, Instruction};
    use crate::cpu::AddressingMode::Absolute;
    use crate::instructions::slo::SLO;

    #[test]
    fn basic_operation() {
        // Given
        let mut cpu = CPU::empty();

        cpu.accumulator = 0x07;
        cpu.write(&AddressingMode::ZeroPage(0x8C), 0x0A);

        // When
        SLO::new(AddressingMode::ZeroPage(0x8C)).execute(&mut cpu);

        // Then
        assert_eq!(0x14, cpu.read(&AddressingMode::ZeroPage(0x8C)));
        assert_eq!(0x17, cpu.accumulator);
        assert_eq!(0b0000_0000, cpu.processor_status);
    }

    #[test]
    fn nestest_scenario1() {
        // Given
        let mut cpu = CPU::empty();

        cpu.accumulator = 0xB3;
        cpu.index_register_x = 0x02;
        cpu.processor_status = 0xE4;
        cpu.write_mem16(0x0047, 0x647);
        cpu.write(&AddressingMode::Absolute(0x0647), 0xA5);

        // When
        SLO::new(AddressingMode::IndirectX(0x45)).execute(&mut cpu);

        // Then
        assert_eq!(0xFB, cpu.accumulator);
        assert_eq!(0x02, cpu.index_register_x);
        assert_eq!(0xE5, cpu.processor_status);

        // 1010_0101 ($A5) becomes 0100_1010 ($4A)
        assert_eq!(0x4A, cpu.read(&Absolute(0x0647)));
    }

    #[test]
    fn string_representation() {
        let slo = SLO::new(AddressingMode::ZeroPageY(0xFB));

        assert_eq!("*SLO $FB,Y", slo.to_string())
    }

    #[test]
    fn bytes_representation() {
        let slo = SLO::new(AddressingMode::ZeroPageX(0xAA));

        assert_eq!(vec![0x17, 0xAA], slo.bytes());
    }
}