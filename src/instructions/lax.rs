use std::fmt::{Display, Formatter};
use crate::cpu::{AddressingMode, CPU, Instruction, StatusFlag};

// Note:  This is an ILLEGAL opcode but is used in the nestest ROM.

// https://www.masswerk.at/6502/6502_instruction_set.html#LAX
pub(super) struct LAX {
    mode: AddressingMode
}

impl LAX {
    pub fn new(mode: AddressingMode) -> Self {
        LAX{ mode }
    }
}

impl Display for LAX {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "*LAX {}", self.mode)
    }
}

impl Instruction for LAX {
    fn execute(&self, cpu: &mut CPU) -> u8 {
        let mem = cpu.read(&self.mode);
        cpu.accumulator = mem;
        cpu.index_register_x = mem;

        cpu.set_flag(StatusFlag::Zero,  mem == 0);
        cpu.set_flag(StatusFlag::Negative, mem > 0x7F);

        cpu.default_cycles(&self.mode)
    }

    fn bytes(&self) -> Vec<u8> {
        match self.mode {
            AddressingMode::ZeroPage(addr) => vec![0xA7, addr],
            AddressingMode::ZeroPageY(addr) => vec![0xB7, addr],
            AddressingMode::Absolute(addr) => self.bytes_for_opcode(0xAF, addr),
            AddressingMode::AbsoluteY(addr) => self.bytes_for_opcode(0xBF, addr),
            AddressingMode::IndirectX(addr) => vec![0xA3, addr],
            AddressingMode::IndirectY(addr) => vec![0xB3, addr],
            _ => panic!("Addressing mode not allowed for LAX")
        }
    }
}

#[cfg(test)]
mod test {
    use crate::cpu::{AddressingMode, CPU, Instruction};
    use crate::cpu::AddressingMode::ZeroPageY;
    use crate::instructions::jmp::JumpAddressMode::Absolute;
    use crate::instructions::lax::LAX;

    #[test]
    fn both_registers_are_loaded() {
        // Given
        let mut cpu = CPU::empty();
        cpu.write(&AddressingMode::Absolute(0x00AC), 0xFE);
        cpu.index_register_y = 0x0C;

        // When
        LAX::new(ZeroPageY(0xA0)).execute(&mut cpu);

        // Then
        assert_eq!(0xFE, cpu.accumulator);
        assert_eq!(0xFE, cpu.index_register_x);
        assert_eq!(0b1000_0000, cpu.processor_status);
    }

    #[test]
    fn string_representation() {
        let lax = LAX::new(AddressingMode::IndirectY(0x55));

        assert_eq!("*LAX ($55),Y", lax.to_string());
    }

    #[test]
    fn bytes_representation() {
        let lax = LAX::new(AddressingMode::AbsoluteY(0x00AB));

        assert_eq!(vec![0xBF, 0xAB, 0x00], lax.bytes());
    }
}