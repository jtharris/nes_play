use std::fmt::{Display, Formatter};
use crate::cpu::{AddressingMode, Instruction, CPU};

// http://www.obelisk.me.uk/6502/reference.html#STY
pub(super) struct STY {
    mode: AddressingMode
}

impl STY {
    pub fn new(mode: AddressingMode) -> Self {
        STY{ mode }
    }
}

impl Display for STY {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "STY {}", self.mode)
    }
}

impl Instruction for STY {
    fn execute(&self, cpu: &mut CPU) -> u8 {
        cpu.write(&self.mode, cpu.index_register_y);

        cpu.default_cycles(&self.mode)
    }

    fn bytes(&self) -> Vec<u8> {
        match self.mode {
            AddressingMode::ZeroPage(addr) => vec![0x84, addr],
            AddressingMode::ZeroPageX(addr) => vec![0x94, addr],
            AddressingMode::Absolute(addr) => self.bytes_for_opcode(0x8C, addr),
            _ => panic!("Addressing mode not allowed for STY")
        }
    }

    fn debug_string(&self, cpu: &CPU) -> String {
        format!("STY {}", self.mode.debug_string(&cpu))
    }
}

#[cfg(test)]
mod test {
    use crate::cpu::{CPU, Instruction};
    use crate::cpu::AddressingMode::{ZeroPage, ZeroPageX};
    use super::STY;

    #[test]
    fn y_is_stored() {
        // Given
        let mut cpu = CPU::empty();
        cpu.index_register_y = 0xA7;

        // When
        STY::new(ZeroPage(0x88)).execute(&mut cpu);

        // Then
        assert_eq!(0xA7, cpu.read(&ZeroPage(0x88)))
    }

    #[test]
    fn string_representation() {
        let sty = STY::new(ZeroPageX(0xF1));

        assert_eq!("STY $F1,X", sty.to_string())
    }

    #[test]
    fn bytes_representation() {
        let sty = STY::new(ZeroPage(0xFE));

        assert_eq!(vec![0x84, 0xFE], sty.bytes());
    }
}
