use std::fmt::{Display, Formatter};
use crate::cpu::{AddressingMode, CPU, Instruction, StatusFlag};

// https://www.masswerk.at/6502/6502_instruction_set.html#SRE
pub(super) struct SRE {
    mode: AddressingMode
}

impl SRE {
    pub fn new(mode: AddressingMode) -> Self {
        SRE { mode }
    }
}

impl Display for SRE {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "SRE {}", self.mode)
    }
}

impl Instruction for SRE {
    fn execute(&self, cpu: &mut CPU) -> u8 {
        let value = cpu.read(&self.mode);
        let carry = value & 0x01 == 0x01;
        let shifted = value >> 1;

        cpu.write(&self.mode, shifted);
        cpu.accumulator ^= shifted;


        cpu.set_flag(StatusFlag::Carry, carry);
        cpu.set_flag(StatusFlag::Zero, cpu.accumulator == 0);
        cpu.set_flag(StatusFlag::Negative, cpu.accumulator > 0x7F);

        cpu.memory_cycles(&self.mode)
    }

    fn bytes(&self) -> Vec<u8> {
        match self.mode {
            AddressingMode::ZeroPage(addr) => vec![0x47, addr],
            AddressingMode::ZeroPageX(addr) => vec![0x57, addr],
            AddressingMode::Absolute(addr) => self.bytes_for_opcode(0x4F, addr),
            AddressingMode::AbsoluteX(addr) => self.bytes_for_opcode(0x5F, addr),
            AddressingMode::AbsoluteY(addr) => self.bytes_for_opcode(0x5B, addr),
            AddressingMode::IndirectX(addr) => vec![0x43, addr],
            AddressingMode::IndirectY(addr) => vec![0x53, addr],
            _ => panic!("Addressing mode not allowed for SRE")
        }
    }

    fn debug_string(&self, cpu: &CPU) -> String {
        format!("SRE {}", self.mode.debug_string(&cpu))
    }

    fn illegal(&self) -> bool {
        true
    }
}

#[cfg(test)]
mod test {
    use crate::cpu::{AddressingMode, CPU, Instruction};
    use crate::cpu::AddressingMode::IndirectX;
    use crate::instructions::sre::SRE;

    #[test]
    fn combined_behavior() {
        // Given
        let mut cpu = CPU::empty();
        cpu.accumulator = 0xB3;
        cpu.index_register_x = 0x02;
        cpu.processor_status = 0xE4;
        cpu.write(&AddressingMode::ZeroPage(0x47), 0x47);
        cpu.write(&AddressingMode::ZeroPage(0x48), 0x06);
        cpu.write(&AddressingMode::Absolute(0x0647), 0xA5);

        // When
        SRE::new(AddressingMode::IndirectX(0x45)).execute(&mut cpu);

        // Then
        assert_eq!(0xE1, cpu.accumulator);
        assert_eq!(0xE5, cpu.processor_status);
        assert_eq!(0x52, cpu.read(&AddressingMode::Absolute(0x0647)));
    }

    #[test]
    fn string_representation() {
        let sre = SRE::new(AddressingMode::ZeroPage(0xF1));

        assert_eq!("SRE $F1", sre.to_string());
    }

    #[test]
    fn bytes_representation() {
        let sre = SRE::new(AddressingMode::AbsoluteX(0x88AC));

        assert_eq!(vec![0x5F, 0xAC, 0x88], sre.bytes());
    }
}