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
        todo!()
    }
}

impl Instruction for STY {
    fn execute(&self, cpu: &mut CPU) -> u8 {
        cpu.write(&self.mode, cpu.index_register_y);

        cpu.default_cycles(&self.mode)
    }
}

#[cfg(test)]
mod test {
    use crate::cpu::{CPU, Instruction};
    use crate::cpu::AddressingMode::ZeroPage;
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
}
