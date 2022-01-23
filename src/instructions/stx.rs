use std::fmt::{Display, Formatter};
use crate::cpu::{AddressingMode, Instruction, CPU};

// http://www.obelisk.me.uk/6502/reference.html#STX
pub(super) struct STX {
    mode: AddressingMode
}

impl STX {
    pub fn new(mode: AddressingMode) -> Self {
        STX{ mode }
    }
}

impl Display for STX {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "STX {}", self.mode)
    }
}

impl Instruction for STX {
    fn execute(&self, cpu: &mut CPU) -> u8 {
        cpu.write(&self.mode, cpu.index_register_x);

        cpu.default_cycles(&self.mode)
    }

    fn bytes(&self) -> Vec<u8> {
        match self.mode {
            AddressingMode::ZeroPage(addr) => vec![0x86, addr],
            AddressingMode::ZeroPageY(addr) => vec![0x96, addr],
            AddressingMode::Absolute(addr) => self.bytes_for_opcode(0x8E, addr),
            _ => panic!("Addressing mode not allowed for STX")
        }
    }

    fn debug_string(&self, cpu: &CPU) -> String {
        format!("STX {}", self.mode.debug_string(&cpu))
    }
}

#[cfg(test)]
mod test {
    use crate::cpu::{CPU, Instruction};
    use crate::cpu::AddressingMode::{ZeroPage, ZeroPageY};
    use super::STX;

    #[test]
    fn x_is_stored() {
        // Given
        let mut cpu = CPU::empty();
        cpu.index_register_x = 0xA7;

        // When
        STX::new(ZeroPage(0x88)).execute(&mut cpu);

        // Then
        assert_eq!(0xA7, cpu.read(&ZeroPage(0x88)))
    }

    #[test]
    fn string_representation() {
        let stx = STX::new(ZeroPageY(0x44));

        assert_eq!("STX $44,Y", stx.to_string())
    }

    #[test]
    fn bytes_representation() {
        let stx = STX::new(ZeroPageY(0x8A));

        assert_eq!(vec![0x96, 0x8A], stx.bytes());
    }
}
