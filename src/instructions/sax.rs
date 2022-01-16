use std::fmt::{Display, Formatter};
use crate::cpu::{AddressingMode, CPU, Instruction};

// https://www.masswerk.at/6502/6502_instruction_set.html#SAX
pub(super) struct SAX {
    mode: AddressingMode
}

impl SAX {
    pub fn new(mode: AddressingMode) -> Self {
        SAX {mode}
    }
}

impl Display for SAX {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "*SAX {}", self.mode)
    }
}

impl Instruction for SAX {
    fn execute(&self, cpu: &mut CPU) -> u8 {
        cpu.write(&self.mode, cpu.accumulator & cpu.index_register_x);

        cpu.default_cycles(&self.mode)
    }

    fn bytes(&self) -> Vec<u8> {
        match self.mode {
            AddressingMode::ZeroPage(addr) => vec![0x87, addr],
            AddressingMode::ZeroPageY(addr) => vec![0x97, addr],
            AddressingMode::Absolute(addr) => self.bytes_for_opcode(0x8F, addr),
            AddressingMode::IndirectX(addr) => vec![0x83, addr],
            _ => panic!("Addressing mode not allowed for SAX")
        }
    }
}

#[cfg(test)]
mod test {
    use crate::cpu::{AddressingMode, CPU, Instruction};
    use crate::cpu::AddressingMode::{Absolute, ZeroPageY};
    use crate::instructions::sax::SAX;

    #[test]
    fn basic_write() {
        // Given
        let mut cpu = CPU::empty();
        cpu.accumulator = 0x8D;
        cpu.index_register_x = 0xF7;

        // When
        SAX::new(AddressingMode::Absolute(0x0182)).execute(&mut cpu);

        // Then
        assert_eq!(0x85, cpu.read(&Absolute(0x0182)));
    }

    #[test]
    fn string_representation() {
        let sax = SAX::new(ZeroPageY(0xFF));

        assert_eq!("*SAX $FF,Y", sax.to_string())
    }

    #[test]
    fn bytes_representation() {
        let sax = SAX::new(ZeroPageY(0xDF));

        assert_eq!(vec![0x97, 0xDF], sax.bytes());
    }
}
