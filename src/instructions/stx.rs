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
        todo!()
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
}
